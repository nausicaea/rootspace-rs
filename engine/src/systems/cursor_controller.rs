use glium::glutin::ElementState;
use ecs::{Assembly, DispatchEvents, LoopStageFlag, SystemTrait};
use event::{EngineEvent, EngineEventFlag};
use singletons::Singletons;
use components::cursor::{Cursor, FlankDirection};

/// Updates data within the `Cursor` component and emits events on mouse button state changes.
#[derive(Default)]
pub struct CursorController;

impl CursorController {
    /// Creates a new `CursorController` system.
    pub fn new() -> Self {
        Default::default()
    }
}

impl SystemTrait<EngineEvent, Singletons> for CursorController {
    /// The `CursorController` requires exactly one `Cursor` component.
    fn verify_requirements(&self, entities: &Assembly) -> bool {
        entities.count1::<Cursor>() == 1
    }
    /// The `CursorController` receives the event handling calls.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    /// The `CursorController` listens for the `CursorPosition` and `MouseInput` events.
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::CURSOR_POSITION | EngineEventFlag::MOUSE_INPUT
    }
    /// Upon receiving a `CursorPosition` event, the respective field of the `Cursor` component is
    /// updated. Upon receiving a `MouseInput` event, the new button state is compared to the
    /// previous one and a `MouseInputFlank` event is dispatched upon state changes. This allows
    /// other systems to listen for state changes and not just for button press events.
    fn handle_event(
        &mut self,
        entities: &mut Assembly,
        _: &mut Singletons,
        event: &EngineEvent,
    ) -> DispatchEvents<EngineEvent> {
        match *event {
            EngineEvent::CursorPosition(position) => {
                // Update the cursor component's position.
                entities
                    .ws1::<Cursor>()
                    .map(|(_, c)| c.position = position)
                    .expect("Could not access the Cursor component");
                (None, None)
            }
            EngineEvent::MouseInput(button, state) => {
                // Given the current button state and the new one, emit an event if the button went
                // from released to pressed (eg. a down flank) or from pressed to released (eg. an
                // up flank). Subsequently update the button state on the cursor component.
                entities
                    .ws1::<Cursor>()
                    .map(|(_, c)| {
                        // Get the current mouse button state
                        let current_state =
                            c.buttons.entry(button).or_insert(ElementState::Released);

                        // Determine the derivative of the mouse button press function.
                        let resulting_event = match *current_state {
                            ElementState::Pressed => match state {
                                ElementState::Pressed => None,
                                ElementState::Released => Some(vec![
                                    EngineEvent::MouseInputFlank(button, FlankDirection::Up),
                                ]),
                            },
                            ElementState::Released => match state {
                                ElementState::Pressed => Some(vec![
                                    EngineEvent::MouseInputFlank(button, FlankDirection::Down),
                                ]),
                                ElementState::Released => None,
                            },
                        };

                        // Update the current mouse button state.
                        *current_state = state;

                        // Return the resulting flank event for immediate dispatch.
                        (resulting_event, None)
                    })
                    .expect("Could not access the Cursor component")
            }
            _ => (None, None),
        }
    }
}
