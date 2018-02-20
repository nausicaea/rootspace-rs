use std::time::Duration;
use glium::Display;
use nalgebra::{Vector3, Vector2};
use uuid::Uuid;
use ecs::{SystemTrait, LoopStageFlag, Assembly, DispatchEvents};
use event::EngineEvent;
use singletons::Singletons;
use common::ui_element::UiElement;
use components::camera::Camera;
use components::ui_state::UiState;

pub struct DebugUi {
    display: Display,
    element: Option<Uuid>,
}

impl DebugUi {
    pub fn new(display: &Display) -> Self {
        DebugUi {
            display: display.clone(),
            element: None,
        }
    }
}

impl SystemTrait<EngineEvent, Singletons> for DebugUi {
    fn verify_requirements(&self, _: &Assembly) -> bool {
        true
    }
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::DYNAMIC_UPDATE
    }
    fn dynamic_update(&mut self, entities: &mut Assembly, aux: &mut Singletons, _: &Duration, _: &Duration) -> DispatchEvents<EngineEvent> {
        if self.element.is_none() {
            // Obtain the viewport dimensions.
            let dimensions = entities.rs1::<Camera>()
                .map(|(_, c)| Vector2::new(c.dimensions[0] as f32, c.dimensions[1] as f32))
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
                                                    &Vector3::new(0.5, 0.5, 0.0),
                                                    &ui_state.tooltip.relative_position_offset,
                                                    &dimensions, 200,
                                                    "0000 ms (000 FPS)")
                .expect("Could not create the debug UI");

            // Create and register the element.
            let id = Uuid::new_v4();
            ui_state.elements.insert(id, element);
            self.element = Some(id);
        }
        (None, None)
    }
}
