use glium::Display;
use nalgebra::{Point3, Vector2};
use uuid::Uuid;
use ecs::{Assembly, DispatchEvents, EcsError, Entity, LoopStageFlag, SystemTrait};
use event::{EngineEvent, EngineEventFlag};
use singletons::Singletons;
use components::camera::Camera;
use components::model::Model;
use components::tooltip::TooltipData;
use components::ui_state::UiState;
use common::ui_element::{UiElement, UiElementError as RootUiElementError};

/// The `TooltipController` is responsible for managing the state associated with the user interface.
/// It also processes events that relate to the UI.
pub struct TooltipController {
    /// Provides access to the graphics `Display`. Internally this is just an Rc.
    display: Display,
    /// Holds the entity currently selected by the cursor.
    current_target: Option<Entity>,
    /// Holds the currently active tooltip. There may only be one tooltip at a time.
    current_tooltip: Option<Uuid>,
}

impl TooltipController {
    /// Creates a new `TooltipController` system.
    pub fn new(display: &Display) -> Self {
        TooltipController {
            display: display.clone(),
            current_target: None,
            current_tooltip: None,
        }
    }
    /// Creates a new tooltip, if the supplied target has a `TooltipData` component.
    fn create_tooltip(
        &mut self,
        entities: &mut Assembly,
        aux: &mut Singletons,
        target: &Entity,
    ) -> Result<(), TooltipError> {
        if let Ok(tooltip_text) = entities
            .borrow_component::<TooltipData>(target)
            .map(|t| t.text.to_owned())
        {
            // Attempt to determine the location of the entity.
            let entity_pos_world = entities
                .borrow_component::<Model>(target)
                .map(|m| Point3::from_coordinates(*m.translation()))
                .map_err(|e| TooltipError::ComponentNotFound("Model".into(), target.clone(), e))?;

            // Project the entity position to normalized device coordinates (this requires the camera
            // entity).
            let (entity_pos_ndc, dimensions) = entities
                .rs1::<Camera>()
                .map(|(_, c)| {
                    (
                        c.world_point_to_ndc(&entity_pos_world),
                        Vector2::new(c.dimensions[0] as f32, c.dimensions[1] as f32),
                    )
                })
                .expect("Could not access the Camera component");

            // Obtain a mutable reference to the `UiState`.
            let (_, ui_state) = entities
                .ws1::<UiState>()
                .expect("Could not access the UiState component");

            // Create the text box
            let element = UiElement::create_textbox(
                &self.display,
                &mut aux.factory,
                &mut ui_state.font_cache,
                &ui_state.tooltip.margin,
                &ui_state.tooltip.font,
                &ui_state.tooltip.rect_shaders,
                &ui_state.tooltip.rect_textures,
                &ui_state.tooltip.text_shaders,
                &entity_pos_ndc.coords,
                &ui_state.tooltip.relative_position_offset,
                &dimensions,
                ui_state.tooltip.text_width,
                &tooltip_text,
            )?;

            // Create and register the element.
            let id = Uuid::new_v4();
            aux.ui_hierarchy.insert(id, element.model.clone());
            ui_state.elements.insert(id, element);
            self.current_tooltip = Some(id);
        }

        Ok(())
    }
    /// Removes the current tooltip (if any) from the `UiElement` registry.
    fn destroy_tooltip(&mut self, entities: &mut Assembly, aux: &mut Singletons) {
        entities
            .ws1::<UiState>()
            .map(|(_, u)| {
                if let Some(ref id) = self.current_tooltip {
                    u.elements.remove(id);
                    aux.ui_hierarchy.remove(id).unwrap_or_else(|_| unreachable!());
                }
                self.current_tooltip = None;
            })
            .expect("Could not access the UiState component")
    }
}

impl SystemTrait<EngineEvent, Singletons> for TooltipController {
    /// The `TooltipController` depends on the presence of exactly one `UiState` and exactly one
    /// `Camera` component.
    fn verify_requirements(&self, entities: &Assembly) -> bool {
        entities.count1::<UiState>() == 1 && entities.count1::<Camera>() == 1
    }
    /// `TooltipController` subscribes to the `handle_event` and update calls.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    /// `TooltipController` subscribes to the `CursorPosition` event.
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::CURSOR_POSITION
    }
    fn handle_event(
        &mut self,
        entities: &mut Assembly,
        aux: &mut Singletons,
        event: &EngineEvent,
    ) -> DispatchEvents<EngineEvent> {
        match *event {
            EngineEvent::CursorPosition(position) => {
                let menu_active = entities
                    .rs1::<UiState>()
                    .map(|(_, u)| u.menu_active)
                    .expect("Could not access the UiState component");

                if menu_active {
                    // Perform 2D raycasting.
                    unimplemented!();
                } else {
                    // Perform 3D raycasting.
                    let cursor_ray = entities
                        .rs1::<Camera>()
                        .map(|(_, c)| {
                            c.screen_point_to_ray(&position)
                                .expect("Could not translate the mouse coordinates to a ray")
                        })
                        .expect("Could not access the Camera component");

                    if let Some(hit) = aux.physics.raycast(entities, &cursor_ray) {
                        if let Some(tgt) = self.current_target {
                            if hit.target != tgt {
                                // A new object was hit (two objects probably intersect from the
                                // pov of the camera).
                                self.destroy_tooltip(entities, aux);
                                self.create_tooltip(entities, aux, &hit.target)
                                    .unwrap_or_else(|e| warn!("Unable to create a tooltip: {}", e));
                                self.current_target = Some(hit.target.clone());
                            }
                        } else {
                            // A new object was hit, where none was hit before.
                            self.create_tooltip(entities, aux, &hit.target)
                                .unwrap_or_else(|e| warn!("Unable to create a tooltip: {}", e));
                            self.current_target = Some(hit.target.clone());
                        }
                    } else if self.current_target.is_some() {
                        // The current object was exited.
                        self.destroy_tooltip(entities, aux);
                        self.current_target = None;
                    }
                }
            }
            _ => (),
        }
        (None, None)
    }
}

/// Operations of the `TooltipController` may fail. `TooltipError` describes those errors.
#[derive(Debug, Fail)]
pub enum TooltipError {
    #[fail(display = "The entity {1} has no component '{0}'", _0, _1)]
    ComponentNotFound(String, Entity, #[cause] EcsError),
    #[fail(display = "{}", _0)] UiElementError(#[cause] RootUiElementError),
}

impl From<RootUiElementError> for TooltipError {
    fn from(value: RootUiElementError) -> Self {
        TooltipError::UiElementError(value)
    }
}
