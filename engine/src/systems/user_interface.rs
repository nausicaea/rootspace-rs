use std::time::{Instant, Duration};
use glium::Display;
use nalgebra;
use nalgebra::{Point2, Point3, Vector2, Vector3};
use rusttype::gpu_cache::CacheWriteErr;
use uuid::Uuid;
use ecs::{LoopStageFlag, SystemTrait, Assembly, EcsError};
use event::{EngineEventFlag, EngineEvent};
use singletons::Singletons;
use singletons::factory::FactoryError;
use singletons::physics::StatefulHit;
use components::description::Description;
use components::camera::Camera;
use components::model::Model;
use components::mesh::{Mesh, MeshError};
use components::ui_state::UiState;
use common::ui_element::UiElement;
use common::ui_primitive::UiPrimitive;
use common::text_rendering::layout_paragraph_cached;

/// The `UserInterface` is responsible for managing the state associated with the user interface.
/// It also processes events that relate to the UI.
pub struct UserInterface {
    /// Provides access to the graphics `Display`. Internally this is just an Rc.
    display: Display,
}

impl UserInterface {
    /// Creates a new `UserInterface` system.
    pub fn new(display: &Display) -> Self {
        UserInterface {
            display: display.clone(),
        }
    }
    /// Creates a new speech-bubble `UiElement` and attaches it to the `UiState`.
    fn create_speech_bubble(&self, entities: &mut Assembly, aux: &mut Singletons, target: &str, content: &str, lifetime: u64) -> Result<(), UiError> {
        // Attempt to find the entity named in `target` and retreive its world position.
        let entity_pos_world = entities.rsf2::<_, Description, Model>(|&(_, d, _)| d.name == target)
            .map(|(_, _, m)| {
                let a = m.decompose();
                Point3::from_coordinates(a.translation.vector)
            })
            .map_err(|e| UiError::EntityNotFound(target.into(), e))?;

        // Project the entity position to normalized device coordinates (this requires the camera
        // entity).
        let (entity_pos_ndc, px_dims, dimensions) = entities.rs1::<Camera>()
            .map(|(_, c)| (c.world_point_to_ndc(&entity_pos_world), c.dimensions, Vector2::new(c.dimensions[0] as f32, c.dimensions[1] as f32)))?;

        // Obtain a mutable reference to the `UiState`.
        let (_, ui_state) = entities.ws1::<UiState>()?;

        // Layout the glyphs based on the textual `content`.
        let (glyphs, text_dims_px) = layout_paragraph_cached(&mut ui_state.font_cache_cpu,
                                                             &ui_state.font_cache_gpu,
                                                             &ui_state.common.font,
                                                             ui_state.common.font_scale,
                                                             ui_state.speech_bubble.width,
                                                             content)?;

        // Calculate positions and dimensions of the involved primitives: Text and Rect.
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
        let text_mesh = Mesh::new_text(&self.display, &px_dims, 0.0, &ui_state.font_cache_cpu, &glyphs, &text_dims_ndc.into())?;

        // Create the speech-bubble rectangle mesh.
        let rect_mesh = Mesh::new_quad(&self.display, 0.0)?;

        // Create the primitive materials.
        let text_material = aux.factory.new_material(&self.display,
                                          &ui_state.speech_bubble.text_vertex_shader,
                                          &ui_state.speech_bubble.text_fragment_shader, None, None,
                                          None)?;

        let rect_material = aux.factory.new_material(&self.display,
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
            .map(|(_, u)| {
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

impl SystemTrait<EngineEvent, Singletons> for UserInterface {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT | LoopStageFlag::UPDATE
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::READY | EngineEventFlag::SPEECH_BUBBLE | EngineEventFlag::CURSOR_POSITION
    }
    fn handle_event(&mut self, entities: &mut Assembly, aux: &mut Singletons, event: &EngineEvent) -> Option<EngineEvent> {
        match *event {
            EngineEvent::Ready => {
                // When first getting ready, make sure that exactly one UiState component is
                // present.
                entities.rs1::<UiState>().expect("The UiserInterface system requires exactly one entity with a UiState component");
                // Also require the camera to be present.
                entities.rs1::<Camera>().expect("The UserInterface system requires exactly one entity with a Camera component");
            },
            EngineEvent::SpeechBubble(ref t, ref c, l) => {
                self.create_speech_bubble(entities, aux, t, c, l)
                    .unwrap_or_else(|e| warn!("Could not create a speech bubble: {}", e))
            },
            EngineEvent::CursorPosition(x, y) => {
                let cursor_position = Point2::new(x, y);

                let cursor_ray = entities.rs1::<Camera>()
                    .map(|(_, c)| c.screen_point_to_ray(&cursor_position).expect("The cursor position cannot be represented as a ray"))
                    .expect("Unable to use the camera to perform point transformations");

                let previous_target = entities.rs1::<UiState>()
                    .map(|(_, u)| u.raycast_target.clone())
                    .expect("Unable to get the previous raycast target from the UI");

                match aux.physics.stateful_raycast(entities, &cursor_ray, &previous_target) {
                    StatefulHit::NewHit(hit) => {
                        debug!("You've hit a new object: {}", hit.target);
                        entities.ws1::<UiState>()
                            .map(|(_, u)| u.raycast_target = Some(hit.target.clone()))
                            .expect("Unable to update the current raycast target");
                    },
                    StatefulHit::RepeatHit(hit) => {
                        trace!("You've hit the same object: {}", hit.target);
                    },
                    _ => {
                        entities.ws1::<UiState>()
                            .map(|(_, u)| u.raycast_target = None)
                            .expect("Unable to update the current raycast target");
                    },
                }
            },
            _ => (),
        }
        None
    }
    fn update(&mut self, entities: &mut Assembly, _: &mut Singletons, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        self.update_lifetimes(entities)
            .expect("Unable to update the UI element lifetimes");

        None
    }
}

/// Operations of the `UserInterface` may fail. `UiError` describes those errors.
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
    #[fail(display = "Unable to uniquely identify the entity '{}'", _0)]
    EntityNotFound(String, #[cause] EcsError),
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
