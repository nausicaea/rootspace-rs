use glium::Display;
use ecs::{LoopStageFlag, SystemTrait, Assembly};
use super::super::event::{EngineEventFlag, EngineEvent};

pub struct UserInterface {
    display: Display,
}

impl UserInterface {
    pub fn new(display: &Display) -> Self {
        UserInterface {
            display: display.clone(),
        }
    }
}

impl SystemTrait<EngineEvent> for UserInterface {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::empty()
    }
    fn handle_event(&mut self, entities: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        None
    }
}
