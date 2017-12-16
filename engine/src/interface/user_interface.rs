use std::time::{Instant, Duration};
use glium::Display;
use alga::linear::Transformation;
use nalgebra;
use nalgebra::{Point3, Vector2, Vector3};
use rusttype::gpu_cache::CacheWriteErr;
use uuid::Uuid;
use ecs::{LoopStageFlag, SystemTrait, Assembly, EcsError};
use utilities::layout_paragraph_cached;
use event::{EngineEventFlag, EngineEvent};
use debugging::description::Description;
use geometry::projection::Projection;
use geometry::view::View;
use geometry::model::Model;
use graphics::material::{Material, MaterialError};
use interface::ui_state::UiState;
use interface::ui_element::UiElement;
use interface::ui_primitive::{UiPrimitive, UiPrimitiveError};

#[derive(Debug, Fail)]
pub enum UiError {
    #[fail(display = "{}", _0)]
    AssemblyError(#[cause] EcsError),
    #[fail(display = "{}", _0)]
    CacheError(String),
    #[fail(display = "{}", _0)]
    PrimitiveError(#[cause] UiPrimitiveError),
    #[fail(display = "{}", _0)]
    MaterialCreationError(#[cause] MaterialError),
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

impl From<UiPrimitiveError> for UiError {
    fn from(value: UiPrimitiveError) -> Self {
        UiError::PrimitiveError(value)
    }
}

impl From<MaterialError> for UiError {
    fn from(value: MaterialError) -> Self {
        UiError::MaterialCreationError(value)
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
    fn create_speech_bubble(&self, entities: &mut Assembly, target: &str, content: &str, lifetime: u64) -> Result<(), UiError> {
        let entity_pos_world = entities.rsf2::<_, Description, Model>(|&(d, _)| d.name == target)
            .map(|(_, m)| Point3::from_coordinates(m.translation.vector))?;

        let entity_pos_ndc = entities.rs2::<Projection, View>()
            .map(|(p, v)| p.project_point(&v.transform_point(&entity_pos_world)))?;

        let ui_state = entities.ws1::<UiState>()?;

        let (glyphs, text_dims_px) = layout_paragraph_cached(&mut ui_state.font_cache_cpu,
                                                             &ui_state.font_cache_gpu,
                                                             &ui_state.common.font,
                                                             ui_state.common.font_scale,
                                                             ui_state.speech_bubble.width,
                                                             content)?;

        // Calculate positions and dimensions of the involved primitives.
        let dimensions = Vector2::new(ui_state.dimensions[0] as f32, ui_state.dimensions[1] as f32);

        let margin_left = ui_state.speech_bubble.margin_left as f32 / dimensions.x;
        let margin_right = ui_state.speech_bubble.margin_right as f32 / dimensions.x;
        let margin_top = ui_state.speech_bubble.margin_top as f32 / dimensions.y;
        let margin_bottom = ui_state.speech_bubble.margin_bottom as f32 / dimensions.y;
        let relative_offset = ui_state.speech_bubble.relative_position_offset;

        let text_dims_px = Vector2::new(text_dims_px[0] as f32, text_dims_px[1] as f32);
        let text_dims_ndc = text_dims_px.component_div(&dimensions);
        let rect_dims_ndc = text_dims_ndc + Vector2::new(margin_left + margin_right, margin_top + margin_bottom);

        let entity_pos_ndc = Vector2::new(entity_pos_ndc.x, entity_pos_ndc.y);
        let bubble_center = entity_pos_ndc + relative_offset.component_mul(&rect_dims_ndc);
        let text_center = Vector2::new(text_dims_ndc.x - rect_dims_ndc.x, rect_dims_ndc.y - text_dims_ndc.y) / 2.0 + Vector2::new(margin_left, -margin_top);

        let element_model = Model::new(&Vector3::new(bubble_center.x, bubble_center.y, -0.98), &nalgebra::zero());
        let text_model = Model::new(&Vector3::new(text_center.x, text_center.y, -0.01), &nalgebra::zero());
        let rect_model = Model::new(&nalgebra::zero(), &nalgebra::zero());

        let vs = &ui_state.speech_bubble.text_vertex_shader;
        let fs = &ui_state.speech_bubble.text_fragment_shader;
        let text_material = Material::new(&self.display, vs, fs, None, None, None)?;

        let vs = &ui_state.speech_bubble.rect_vertex_shader;
        let fs = &ui_state.speech_bubble.rect_fragment_shader;
        let dt = &ui_state.speech_bubble.rect_diffuse_texture;
        let rect_material = Material::new(&self.display, vs, fs, None, Some(dt), None)?;

        let rect = UiPrimitive::new_rect(&self.display, &rect_dims_ndc.into(), 0.0, rect_model, rect_material)?;
        let text = UiPrimitive::new_text(&self.display, &ui_state.dimensions, 0.0, &ui_state.font_cache_cpu, &glyphs, &text_dims_ndc.into(), text_model, text_material)?;

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

impl SystemTrait<EngineEvent> for UserInterface {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT | LoopStageFlag::UPDATE
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::SPEECH_BUBBLE
    }
    fn handle_event(&mut self, entities: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        if let EngineEvent::SpeechBubble(ref t, ref c, l) = *event {
            self.create_speech_bubble(entities, t, c, l).unwrap_or_else(|e| warn!("{}", e));
        }
        None
    }
    fn update(&mut self, entities: &mut Assembly, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        self.update_lifetimes(entities).unwrap();
        None
    }
}
