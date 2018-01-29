use glium::glutin::ElementState;
use ecs::{SystemTrait, LoopStageFlag, Assembly};
use event::{EngineEvent, EngineEventFlag};
use components::cursor::{Cursor, FlankDirection};

#[derive(Default)]
pub struct CursorController;

impl CursorController {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<A> SystemTrait<EngineEvent, A> for CursorController {
    fn verify_requirements(&self, entities: &Assembly) -> bool {
        entities.count1::<Cursor>() == 1
    }
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::CURSOR_POSITION | EngineEventFlag::MOUSE_INPUT
    }
    fn handle_event(&mut self, entities: &mut Assembly, _: &mut A, event: &EngineEvent) -> (Option<EngineEvent>, Option<EngineEvent>) {
        match *event {
            EngineEvent::CursorPosition(position) => {
                // Update the cursor component's position.
                entities.ws1::<Cursor>()
                    .map(|(_, c)| c.position = position)
                    .expect("Could not access the Cursor component");
                (None, None)
            },
            EngineEvent::MouseInput(button, state) => {
                // Given the current button state and the new one, emit an event if the button went
                // from released to pressed (eg. a down flank) or from pressed to released (eg. an
                // up flank). Subsequently update the button state on the cursor component.
                entities.ws1::<Cursor>()
                    .map(|(_, c)| {
                        // Get the current mouse button state
                        let current_state = c.buttons.entry(button).or_insert(ElementState::Released);

                        // Determine the derivative of the mouse button press function.
                        let resulting_event = match *current_state {
                            ElementState::Pressed => match state {
                                ElementState::Pressed => None,
                                ElementState::Released => Some(EngineEvent::MouseInputFlank(button, FlankDirection::Up)),
                            },
                            ElementState::Released => match state {
                                ElementState::Pressed => Some(EngineEvent::MouseInputFlank(button, FlankDirection::Down)),
                                ElementState::Released => None,
                            },
                        };

                        // Update the current mouse button state.
                        *current_state = state;

                        // Return the resulting flank event for immediate dispatch.
                        (resulting_event, None)
                    })
                    .expect("Could not access the Cursor component")
            },
            _ => (None, None),
        }
    }
}
