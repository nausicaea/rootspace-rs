use std::collections::VecDeque;
use std::time::Duration;
use glium::Display;
use nalgebra::{Vector2, Vector3};
use uuid::Uuid;
use ecs::{Assembly, DispatchEvents, LoopStageFlag, SystemTrait};
use event::EngineEvent;
use singletons::Singletons;
use common::ui_element::UiElement;
use common::physics::duration_as_float;
use components::camera::Camera;
use components::ui_state::UiState;

pub struct DebugUi {
    display: Display,
    element: Option<Uuid>,
    dt_history: VecDeque<Duration>,
    window_size: usize,
    last_display_time: Duration,
    display_interval: Duration,
}

impl DebugUi {
    pub fn new(display: &Display) -> Self {
        let window_size = 10;
        let display_interval = Duration::new(1, 0);

        DebugUi {
            display: display.clone(),
            element: None,
            dt_history: VecDeque::with_capacity(window_size + 1),
            window_size: window_size,
            last_display_time: Duration::new(0, 0),
            display_interval: display_interval,
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
    fn dynamic_update(
        &mut self,
        entities: &mut Assembly,
        aux: &mut Singletons,
        time: &Duration,
        delta_time: &Duration,
    ) -> DispatchEvents<EngineEvent> {
        // Update the buffer of past delta time values.
        self.dt_history.push_back(*delta_time);
        while self.dt_history.len() > self.window_size {
            self.dt_history.pop_front();
        }

        // Create the display element, or update it.
        if self.element.is_none() {
            // Obtain the viewport dimensions.
            let dimensions = entities
                .rs1::<Camera>()
                .map(|(_, c)| Vector2::new(c.dimensions[0] as f32, c.dimensions[1] as f32))
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
                &Vector3::new(0.5, 0.5, 0.0),
                &ui_state.tooltip.relative_position_offset,
                &dimensions,
                200,
                "0000 ms (000 FPS)",
            ).expect("Could not create the debug UI");

            // Create and register the element.
            let id = Uuid::new_v4();
            aux.ui_hierarchy.insert(id, element.model.clone());
            ui_state.elements.insert(id, element);
            self.element = Some(id);
        } else if *time - self.last_display_time >= self.display_interval {
            self.last_display_time = *time;

            let dt_sum: Duration = self.dt_history.iter().sum();
            let frame_time = duration_as_float::<f32>(dt_sum).unwrap() / (self.dt_history.len() as f32);
            let frame_time_ms = (frame_time * 1e3).round();
            let fps = (1.0 / frame_time).round();
            trace!("{:04.0} ms ({:03.0} FPS)", frame_time_ms, fps);
        }

        (None, None)
    }
}
