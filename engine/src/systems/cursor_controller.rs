use glium::glutin::{MouseButton, ElementState};
use ecs::{SystemTrait, LoopStageFlag, Assembly};
use event::{EngineEvent, EngineEventFlag};
use components::cursor::Cursor;

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
    fn handle_event(&mut self, entities: &mut Assembly, _: &mut A, event: &EngineEvent) -> Option<EngineEvent> {
        let mut resulting_event = None;

        match *event {
            EngineEvent::CursorPosition(position) => {
                // Update the cursor component's position.
                entities.ws1::<Cursor>()
                    .map(|(_, c)| c.position = position)
                    .expect("Could not access the Cursor component");
            },
            EngineEvent::MouseInput(button, state) => {
                // Given the current button state and the new one, emit an event if the button went
                // from released to pressed (eg. a down flank) or from pressed to released (eg. an
                // up flank). Subsequently update the button state on the cursor component.
                match button {
                    MouseButton::Left => {
                        entities.rs1::<Cursor>()
                            .map(|(_, c)| {
                                match c.left_button {
                                    ElementState::Pressed => match state {
                                        ElementState::Pressed => (),
                                        ElementState::Released => trace!("Left mouse input up flank received"),
                                    },
                                    ElementState::Released => match state {
                                        ElementState::Pressed => trace!("Left mouse input down flank received"),
                                        ElementState::Released => (),
                                    },
                                }
                            })
                            .expect("Could not access the Cursor component");
                        entities.ws1::<Cursor>()
                            .map(|(_, c)| c.left_button = state)
                            .expect("Could not access the Cursor component");
                    },
                    MouseButton::Right => {
                        entities.rs1::<Cursor>()
                            .map(|(_, c)| {
                                match c.right_button {
                                    ElementState::Pressed => match state {
                                        ElementState::Pressed => (),
                                        ElementState::Released => trace!("Right mouse input up flank received"),
                                    },
                                    ElementState::Released => match state {
                                        ElementState::Pressed => trace!("Right mouse input down flank received"),
                                        ElementState::Released => (),
                                    },
                                }
                            })
                            .expect("Could not access the Cursor component");
                        entities.ws1::<Cursor>()
                            .map(|(_, c)| c.right_button = state)
                            .expect("Could not access the Cursor component");
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        resulting_event
    }
}
