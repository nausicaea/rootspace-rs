use ecs::{LoopStageFlag, EventFlag, ALL_EVENTS, EventTrait, SystemTrait, Assembly};

/// Defines a system that logs all events on the bus to the console (log level TRACE).
pub struct EventMonitor;

impl EventMonitor {
    /// Creates a new `EventMonitor` instance.
    pub fn new() -> EventMonitor {
        EventMonitor {}
    }
}

impl<E: EventTrait> SystemTrait<E> for EventMonitor {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EventFlag {
        ALL_EVENTS
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &E) -> Option<Vec<E>> {
        trace!("Received event '{:?}'", event);
        None
    }
}
