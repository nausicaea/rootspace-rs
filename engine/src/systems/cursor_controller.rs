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
            EngineEvent::CursorPosition(p) => {
                // Update the cursor component.
                entities.ws1::<Cursor>()
                    .map(|(_, c)| c.position = p)
                    .expect("Could not access the Cursor component");
            },
            EngineEvent::MouseInput(b, s) => {
                match b {
                    MouseButton::Left => {
                        entities.ws1::<Cursor>()
                            .map(|(_, c)| {
                                match c.left_button {
                                    ElementState::Pressed => match s {
                                        ElementState::Pressed => (),
                                        ElementState::Released =>
                                }
                            })
                            .expect("Could not access the Cursor component");
                    },
                    MouseButton::Right => {
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        resulting_event
    }
}
