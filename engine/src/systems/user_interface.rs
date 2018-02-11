use std::time::{Instant, Duration};
use glium::Display;
use nalgebra::{Point3, Vector2, Vector3, zero};
use rusttype::gpu_cache::CacheWriteErr;
use uuid::Uuid;
use ecs::{Entity, LoopStageFlag, SystemTrait, Assembly, EcsError, DispatchEvents};
use event::{EngineEventFlag, EngineEvent};
use singletons::Singletons;
use singletons::factory::FactoryError;
use components::description::Description;
use components::camera::Camera;
use components::model::Model;
use components::mesh::{Mesh, MeshError};
use components::tooltip::TooltipData;
use components::ui_state::UiState;
use common::resource_group::TextureGroup;
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
            .map_err(|e| UiError::EntityNameNotFound(target.into(), e))?;

        // Project the entity position to normalized device coordinates (this requires the camera
        // entity).
        let (entity_pos_ndc, px_dims, dimensions) = entities.rs1::<Camera>()
            .map(|(_, c)| (c.world_point_to_ndc(&entity_pos_world), c.dimensions, Vector2::new(c.dimensions[0] as f32, c.dimensions[1] as f32)))
            .expect("Could not access the Camera component");

        // Obtain a mutable reference to the `UiState`.
        let (_, ui_state) = entities.ws1::<UiState>()
            .expect("Could not access the UiState component");

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

        let margin_sum = Vector2::new(margin_left + margin_right, margin_top + margin_bottom);

        // Create the model matrices from the above values.
        let text_dims_ndc = Vector2::new(text_dims_px[0] as f32, text_dims_px[1] as f32).component_div(&dimensions);
        let text_center = Vector2::new(-margin_sum.x, margin_sum.y) / 2.0 + Vector2::new(margin_left, -margin_top);
        let text_model = Model::new(Vector3::new(text_center.x, text_center.y, -0.01), zero(), Vector3::new(1.0, 1.0, 1.0));
        let text_mesh = Mesh::new_text(&self.display, &px_dims, 0.0, &ui_state.font_cache_cpu, &glyphs, &text_dims_ndc.into())?;
        let text_material = aux.factory.new_material(&self.display, &ui_state.speech_bubble.text_shaders, &TextureGroup::empty())?;
        let text = UiPrimitive::new(text_model, text_mesh, text_material);

        let rect_dims_ndc = text_dims_ndc + margin_sum;
        let rect_model = Model::new(zero(), zero(), Vector3::new(rect_dims_ndc.x, rect_dims_ndc.y, 1.0));
        let rect_mesh = Mesh::new_quad(&self.display, 0.0)?;
        let rect_material = aux.factory.new_material(&self.display, &ui_state.speech_bubble.rect_shaders, &ui_state.speech_bubble.rect_textures)?;
        let rect = UiPrimitive::new(rect_model, rect_mesh, rect_material);

        let element_center = Vector2::new(entity_pos_ndc.x, entity_pos_ndc.y) + relative_offset.component_mul(&rect_dims_ndc);
        let element_model = Model::new(Vector3::new(element_center.x, element_center.y, -0.98), zero(), Vector3::new(1.0, 1.0, 1.0));
        let element = UiElement::new(element_model, vec![rect, text]);

        // Create and register the element.
        let id = Uuid::new_v4();
        ui_state.elements.insert(id, element);
        ui_state.lifetimes.insert(id, (Instant::now(), Duration::new(lifetime, 0)));

        Ok(())
    }
    /// Creates a new tooltip, if the supplied target has a `TooltipData` component.
    fn create_tooltip(&self, entities: &mut Assembly, aux: &mut Singletons, target: &Entity) -> Result<(), UiError> {
        if let Ok(tooltip_text) = entities.borrow_component::<TooltipData>(target).map(|t| t.text.to_owned()) {
            // Attempt to determine the location of the entity.
            let entity_pos_world = entities.borrow_component::<Model>(target)
                .map(|m| {
                    let a = m.decompose();
                    Point3::from_coordinates(a.translation.vector)
                })
                .map_err(|e| UiError::ComponentNotFound("Model".into(), target.clone(), e))?;

            // Project the entity position to normalized device coordinates (this requires the camera
            // entity).
            let (entity_pos_ndc, px_dims, dimensions) = entities.rs1::<Camera>()
                .map(|(_, c)| (c.world_point_to_ndc(&entity_pos_world), c.dimensions, Vector2::new(c.dimensions[0] as f32, c.dimensions[1] as f32)))
                .expect("Could not access the Camera component");

            // Obtain a mutable reference to the `UiState`.
            let (_, ui_state) = entities.ws1::<UiState>()
                .expect("Could not access the UiState component");

            // Layout the glyphs based on the textual `content`.
            let (glyphs, text_dims_px) = layout_paragraph_cached(&mut ui_state.font_cache_cpu,
                                                                 &ui_state.font_cache_gpu,
                                                                 &ui_state.common.font,
                                                                 ui_state.common.font_scale,
                                                                 ui_state.tooltip.width,
                                                                 &tooltip_text)?;

            // Calculate positions and dimensions of the involved primitives: Text and Rect.
            let margin_left = ui_state.tooltip.margin_left as f32 / dimensions.x;
            let margin_right = ui_state.tooltip.margin_right as f32 / dimensions.x;
            let margin_top = ui_state.tooltip.margin_top as f32 / dimensions.y;
            let margin_bottom = ui_state.tooltip.margin_bottom as f32 / dimensions.y;

            let margin_sum = Vector2::new(margin_left + margin_right, margin_top + margin_bottom);

            // Create the model matrices from the above values.
            let text_dims_ndc = Vector2::new(text_dims_px[0] as f32, text_dims_px[1] as f32).component_div(&dimensions);
            let text_center = Vector2::new(-margin_sum.x, margin_sum.y) / 2.0 + Vector2::new(margin_left, -margin_top);
            let text_model = Model::new(Vector3::new(text_center.x, text_center.y, -0.01), zero(), Vector3::new(1.0, 1.0, 1.0));
            let text_mesh = Mesh::new_text(&self.display, &px_dims, 0.0, &ui_state.font_cache_cpu, &glyphs, &text_dims_ndc.into())?;
            let text_material = aux.factory.new_material(&self.display, &ui_state.tooltip.text_shaders, &TextureGroup::empty())?;
            let text = UiPrimitive::new(text_model, text_mesh, text_material);

            let rect_dims_ndc = text_dims_ndc + margin_sum;
            let rect_center = Vector3::new(0.0, 0.0, 0.0);
            let rect_model = Model::new(rect_center, zero(), Vector3::new(rect_dims_ndc.x, rect_dims_ndc.y, 1.0));
            let rect_mesh = Mesh::new_quad(&self.display, 0.0)?;
            let rect_material = aux.factory.new_material(&self.display, &ui_state.tooltip.rect_shaders, &ui_state.tooltip.rect_textures)?;
            let rect = UiPrimitive::new(rect_model, rect_mesh, rect_material);

            let relative_offset = ui_state.tooltip.relative_position_offset;
            let element_center = Vector2::new(entity_pos_ndc.x, entity_pos_ndc.y) + relative_offset.component_mul(&rect_dims_ndc);
            let element_model = Model::new(Vector3::new(element_center.x, element_center.y, -0.98), zero(), Vector3::new(1.0, 1.0, 1.0));
            let element = UiElement::new(element_model, vec![rect, text]);

            // Create and register the element.
            let id = Uuid::new_v4();
            ui_state.elements.insert(id, element);
            ui_state.current_tooltip = Some(id);
        }

        Ok(())
    }
    /// Removes the current tooltip (if any) from the `UiElement` registry.
    fn destroy_tooltip(&self, entities: &mut Assembly) {
        entities.ws1::<UiState>()
            .map(|(_, u)| {
                if let Some(ref id) = u.current_tooltip {
                    u.elements.remove(id);
                }
                u.current_tooltip = None;
            })
            .expect("Could not access the UiState component")
    }
    /// Checks the lifetimes of the registered `UiElement`s and removes those with expired
    /// lifetimes.
    fn update_lifetimes(&self, entities: &mut Assembly) {
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
            .expect("Could not access the UiState component")
    }
}

impl SystemTrait<EngineEvent, Singletons> for UserInterface {
    /// The `UserInterface` depends on the presence of exactly one `UiState` and exactly one
    /// `Camera` component.
    fn verify_requirements(&self, entities: &Assembly) -> bool {
        entities.count1::<UiState>() == 1 && entities.count1::<Camera>() == 1
    }
    /// `UserInterface` subscribes to the `handle_event` and update calls.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT | LoopStageFlag::UPDATE
    }
    /// `UserInterface` subscribes to the `SpeechBubble` and the `CursorPosition` events.
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::SPEECH_BUBBLE | EngineEventFlag::CURSOR_POSITION
    }
    fn handle_event(&mut self, entities: &mut Assembly, aux: &mut Singletons, event: &EngineEvent) -> DispatchEvents<EngineEvent> {
        match *event {
            EngineEvent::SpeechBubble(ref t, ref c, l) => {
                self.create_speech_bubble(entities, aux, t, c, l)
                    .unwrap_or_else(|e| warn!("Could not create a speech bubble: {}", e))
            },
            EngineEvent::CursorPosition(position) => {
                let (menu_active, current_target) = entities.rs1::<UiState>()
                    .map(|(_, u)| (u.menu_active, u.current_target.clone()))
                    .expect("Could not access the UiState component");

                if menu_active {
                    // Perform 2D raycasting.
                    unimplemented!();
                } else {
                    // Perform 3D raycasting.
                    let cursor_ray = entities.rs1::<Camera>()
                        .map(|(_, c)| c.screen_point_to_ray(&position).expect("Could not translate the mouse coordinates to a ray"))
                        .expect("Could not access the Camera component");

                    if let Some(hit) = aux.physics.raycast(entities, &cursor_ray) {
                        if let Some(tgt) = current_target {
                            if hit.target != tgt {
                                // A new object was hit (two objects probably intersect from the
                                // pov of the camera).
                                self.destroy_tooltip(entities);
                                self.create_tooltip(entities, aux, &hit.target)
                                    .unwrap_or_else(|e| warn!("Unable to create a tooltip: {}", e));
                                entities.ws1::<UiState>()
                                    .map(|(_, u)| u.current_target = Some(hit.target.clone()))
                                    .expect("Could not access the UiState component");
                            }
                        } else {
                            // A new object was hit, where none was hit before.
                            self.create_tooltip(entities, aux, &hit.target)
                                .unwrap_or_else(|e| warn!("Unable to create a tooltip: {}", e));
                            entities.ws1::<UiState>()
                                .map(|(_, u)| u.current_target = Some(hit.target.clone()))
                                .expect("Could not access the UiState component");
                        }
                    } else if current_target.is_some() {
                        // The current object was exited.
                        self.destroy_tooltip(entities);
                        entities.ws1::<UiState>()
                            .map(|(_, u)| u.current_target = None)
                            .expect("Could not access the UiState component");
                    }
                }
            },
            _ => (),
        }
        (None, None)
    }
    fn update(&mut self, entities: &mut Assembly, _: &mut Singletons, _: &Duration, _: &Duration) -> DispatchEvents<EngineEvent> {
        self.update_lifetimes(entities);
        (None, None)
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
    #[fail(display = "Unable to uniquely identify the entity name '{}'", _0)]
    EntityNameNotFound(String, #[cause] EcsError),
    #[fail(display = "The entity {1} has no component '{0}'", _0, _1)]
    ComponentNotFound(String, Entity, #[cause] EcsError),
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
