use std::time::{Instant, Duration};
use glium::Display;
use nalgebra::{Point3, Vector2};
use uuid::Uuid;
use ecs::{Entity, LoopStageFlag, SystemTrait, Assembly, EcsError, DispatchEvents};
use event::{EngineEventFlag, EngineEvent};
use singletons::Singletons;
use singletons::factory::FactoryError;
use components::description::Description;
use components::camera::Camera;
use components::model::Model;
use components::mesh::MeshError;
use components::tooltip::TooltipData;
use components::ui_state::UiState;
use common::ui_element::{UiElement, UiElementError as RootUiElementError};
use common::ui_primitive::{UiPrimitiveError as RootUiPrimitiveError};

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
        let (entity_pos_ndc, dimensions) = entities.rs1::<Camera>()
            .map(|(_, c)| (c.world_point_to_ndc(&entity_pos_world), Vector2::from(c.dimensions)))
            .expect("Could not access the Camera component");

        // Obtain a mutable reference to the `UiState`.
        let (_, ui_state) = entities.ws1::<UiState>()
            .expect("Could not access the UiState component");

        // Create the text box
        let element = UiElement::create_textbox(&self.display, &mut aux.factory,
                                                &mut ui_state.font_cache,
                                                &ui_state.speech_bubble.margin,
                                                &ui_state.speech_bubble.font,
                                                &ui_state.speech_bubble.rect_shaders,
                                                &ui_state.speech_bubble.rect_textures,
                                                &ui_state.speech_bubble.text_shaders,
                                                &entity_pos_ndc.coords,
                                                &ui_state.speech_bubble.relative_position_offset,
                                                &dimensions, ui_state.speech_bubble.text_width,
                                                &content)?;

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
            let (entity_pos_ndc, dimensions) = entities.rs1::<Camera>()
                .map(|(_, c)| (c.world_point_to_ndc(&entity_pos_world), Vector2::from(c.dimensions)))
                .expect("Could not access the Camera component");

            // Obtain a mutable reference to the `UiState`.
            let (_, ui_state) = entities.ws1::<UiState>()
                .expect("Could not access the UiState component");

            // Create the text box
            let element = UiElement::create_textbox(&self.display, &mut aux.factory,
                                                    &mut ui_state.font_cache,
                                                    &ui_state.tooltip.margin,
                                                    &ui_state.tooltip.font,
                                                    &ui_state.tooltip.rect_shaders,
                                                    &ui_state.tooltip.rect_textures,
                                                    &ui_state.tooltip.text_shaders,
                                                    &entity_pos_ndc.coords,
                                                    &ui_state.tooltip.relative_position_offset,
                                                    &dimensions, ui_state.tooltip.text_width,
                                                    &tooltip_text)?;

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
    #[fail(display = "Unable to uniquely identify the entity name '{}'", _0)]
    EntityNameNotFound(String, #[cause] EcsError),
    #[fail(display = "The entity {1} has no component '{0}'", _0, _1)]
    ComponentNotFound(String, Entity, #[cause] EcsError),
    #[fail(display = "{}", _0)]
    UiPrimitiveError(#[cause] RootUiPrimitiveError),
    #[fail(display = "{}", _0)]
    UiElementError(#[cause] RootUiElementError),
    #[fail(display = "{}", _0)]
    AssemblyError(#[cause] EcsError),
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

impl From<FactoryError> for UiError {
    fn from(value: FactoryError) -> Self {
        UiError::FactError(value)
    }
}

impl From<RootUiElementError> for UiError {
    fn from(value: RootUiElementError) -> Self {
        UiError::UiElementError(value)
    }
}
