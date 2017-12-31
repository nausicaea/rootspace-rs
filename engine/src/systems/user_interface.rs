use std::time::{Instant, Duration};
use glium::Display;
use alga::linear::Transformation;
use nalgebra;
use nalgebra::{Point3, Vector2, Vector3};
use rusttype::gpu_cache::CacheWriteErr;
use uuid::Uuid;
use ecs::{LoopStageFlag, SystemTrait, Assembly, EcsError};
use utilities::{layout_paragraph_cached, decompose_trs_matrix};
use event::{EngineEventFlag, EngineEvent};
use factory::{FactoryError, ComponentFactory};
use components::description::Description;
use components::projection::Projection;
use components::view::View;
use components::model::Model;
use components::mesh::{Mesh, MeshError};
use components::ui_state::UiState;
use common::ui_element::UiElement;
use common::ui_primitive::UiPrimitive;

#[derive(Debug, Fail)]
pub enum UiError {
    #[fail(display = "{}", _0)]
    AssemblyError(#[cause] EcsError),
    #[fail(display = "{}", _0)]
    CacheError(String),
    #[fail(display = "{}", _0)]
    FactError(#[cause] FactoryError),
    #[fail(display = "{}", _0)]
    MeshCreationError(#[cause] MeshError),
}

impl From<EcsError> for UiError {
    fn from(value: EcsError) -> Self {
        UiError::AssemblyError(value)
    }
}

impl From<CacheWriteErr> for UiError {
    fn from(value: CacheWriteErr) -> Self {
        use rusttype::gpu_cache::CacheWriteErr::*;
        use self::UiError::*;

        match value {
            GlyphTooLarge => CacheError("At least one of the queued glyphs is too big to fit into
                                        the cache, even if all other glyphs are removed".into()),
            NoRoomForWholeQueue => CacheError("Not all of the requested glyphs can fit into the
                                              cache, even if the cache is completely cleared before
                                              the attempt".into()),
        }
    }
}

impl From<FactoryError> for UiError {
    fn from(value: FactoryError) -> Self {
        UiError::FactError(value)
    }
}

impl From<MeshError> for UiError {
    fn from(value: MeshError) -> Self {
        UiError::MeshCreationError(value)
    }
}

/// The `UserInterface` is responsible for managing the state associated with the user interface.
/// It also processes events that relate to the UI.
pub struct UserInterface {
    display: Display,
}

impl UserInterface {
    /// Creates a new `UserInterface` system.
    pub fn new(display: &Display) -> Self {
        UserInterface {
            display: display.clone(),
        }
    }
    fn create_speech_bubble(&self, entities: &mut Assembly, factory: &mut ComponentFactory, target: &str, content: &str, lifetime: u64) -> Result<(), UiError> {
        // Attempt to find the entity named in `target` and retreive its world position.
        let entity_pos_world = entities.rsf2::<_, Description, Model>(|&(d, _)| d.name == target)
            .map(|(_, m)| {
                let (t, _, _) = decompose_trs_matrix(&m);
                Point3::from_coordinates(t.vector)
            })?;

        // Project the entity position to normalized device coordinates (this requires the camera
        // entity).
        let entity_pos_ndc = entities.rs2::<Projection, View>()
            .map(|(p, v)| p.project_point(&v.transform_point(&entity_pos_world)))?;

        // Obtain a mutable reference to the `UiState`.
        let ui_state = entities.ws1::<UiState>()?;

        // Layout the glyphs based on the textual `content`.
        let (glyphs, text_dims_px) = layout_paragraph_cached(&mut ui_state.font_cache_cpu,
                                                             &ui_state.font_cache_gpu,
                                                             &ui_state.common.font,
                                                             ui_state.common.font_scale,
                                                             ui_state.speech_bubble.width,
                                                             content)?;

        // Calculate positions and dimensions of the involved primitives: Text and Rect.
        let dimensions = Vector2::new(ui_state.dimensions[0] as f32, ui_state.dimensions[1] as f32);
        let margin_left = ui_state.speech_bubble.margin_left as f32 / dimensions.x;
        let margin_right = ui_state.speech_bubble.margin_right as f32 / dimensions.x;
        let margin_top = ui_state.speech_bubble.margin_top as f32 / dimensions.y;
        let margin_bottom = ui_state.speech_bubble.margin_bottom as f32 / dimensions.y;
        let relative_offset = ui_state.speech_bubble.relative_position_offset;

        let text_dims_ndc = Vector2::new(text_dims_px[0] as f32, text_dims_px[1] as f32)
            .component_div(&dimensions);
        let rect_dims_ndc = text_dims_ndc + Vector2::new(margin_left + margin_right, margin_top + margin_bottom);

        let element_center = Vector2::new(entity_pos_ndc.x, entity_pos_ndc.y) + relative_offset.component_mul(&rect_dims_ndc);
        let text_center = Vector2::new(text_dims_ndc.x - rect_dims_ndc.x, rect_dims_ndc.y - text_dims_ndc.y) / 2.0 + Vector2::new(margin_left, -margin_top);

        // Create the model matrices from the above values.
        let element_model = Model::new(Vector3::new(element_center.x, element_center.y, -0.98), nalgebra::zero(), Vector3::new(1.0, 1.0, 1.0));
        let text_model = Model::new(Vector3::new(text_center.x, text_center.y, -0.01), nalgebra::zero(), Vector3::new(1.0, 1.0, 1.0));
        let rect_model = Model::new(nalgebra::zero(), nalgebra::zero(), Vector3::new(rect_dims_ndc.x, rect_dims_ndc.y, 1.0));

        // Create the text mesh.
        let text_mesh = Mesh::new_text(&self.display, &ui_state.dimensions, 0.0, &ui_state.font_cache_cpu, &glyphs, &text_dims_ndc.into())?;

        // Create the speech-bubble rectangle mesh.
        let rect_mesh = Mesh::new_quad(&self.display, 0.0)?;

        // Create the primitive materials.
        let text_material = factory.new_material(&self.display,
                                          &ui_state.speech_bubble.text_vertex_shader,
                                          &ui_state.speech_bubble.text_fragment_shader, None, None,
                                          None)?;

        let rect_material = factory.new_material(&self.display,
                                          &ui_state.speech_bubble.rect_vertex_shader,
                                          &ui_state.speech_bubble.rect_fragment_shader, None,
                                          Some(&ui_state.speech_bubble.rect_diffuse_texture),
                                          None)?;

        // Create the primitives.
        let rect = UiPrimitive::new(rect_model, rect_mesh, rect_material);
        let text = UiPrimitive::new(text_model, text_mesh, text_material);

        // Create and register the element.
        let id = Uuid::new_v4();
        ui_state.elements.insert(id, UiElement::new(element_model, vec![rect, text]));
        ui_state.lifetimes.insert(id, (Instant::now(), Duration::new(lifetime, 0)));

        Ok(())
    }
    /// Checks the lifetimes of the registered `UiElement`s and removes those with expired
    /// lifetimes.
    fn update_lifetimes(&self, entities: &mut Assembly) -> Result<(), EcsError> {
        entities.ws1::<UiState>()
            .map(|u| {
                if !u.lifetimes.is_empty() {
                    let to_delete = u.lifetimes.iter()
                        .filter(|&(_, l)| l.0.elapsed() >= l.1)
                        .map(|(i, _)| i)
                        .cloned()
                        .collect::<Vec<_>>();

                    to_delete.iter()
                        .for_each(|i| {
                            u.elements.remove(i);
                            u.lifetimes.remove(i);
                        });
                }
            })
    }
}

impl SystemTrait<EngineEvent, ComponentFactory> for UserInterface {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT | LoopStageFlag::UPDATE
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::SPEECH_BUBBLE
    }
    fn handle_event(&mut self, entities: &mut Assembly, factory: &mut ComponentFactory, event: &EngineEvent) -> Option<EngineEvent> {
        if let EngineEvent::SpeechBubble(ref t, ref c, l) = *event {
            self.create_speech_bubble(entities, factory, t, c, l).unwrap_or_else(|e| warn!("{}", e));
        }
        None
    }
    fn update(&mut self, entities: &mut Assembly, _: &mut ComponentFactory, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        self.update_lifetimes(entities).unwrap();
        None
    }
}