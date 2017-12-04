use ecs::{LoopStageFlag, EventFlag, ALL_EVENTS, HANDLE_EVENT, EventTrait, SystemTrait, Assembly};

pub struct EventMonitor;

impl EventMonitor {
    pub fn new() -> EventMonitor {
        EventMonitor {}
    }
}

impl<E: EventTrait> SystemTrait<E> for EventMonitor {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EventFlag {
        ALL_EVENTS
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &E) -> Option<Vec<E>> {
        trace!("Received event '{:?}'", event);
        None
    }
}
