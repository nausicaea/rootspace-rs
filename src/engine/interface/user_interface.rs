use std::collections::HashMap;
use uuid::Uuid;
use ecs::{LoopStageFlag, SystemTrait, Assembly};
use super::super::event::{EngineEventFlag, EngineEvent};
use super::ui_element::UiElement;

pub struct UserInterface {
    elements: HashMap<Uuid, UiElement>,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            elements: HashMap::new(),
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
