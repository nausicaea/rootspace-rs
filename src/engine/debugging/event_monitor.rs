use ecs::{LoopStageFlag, SystemTrait, Assembly};
use super::super::event::{EngineEventFlag, EngineEvent};

/// Defines a system that logs all events on the bus to the console (log level TRACE).
pub struct EventMonitor;

impl EventMonitor {
    /// Creates a new `EventMonitor` instance.
    pub fn new() -> EventMonitor {
        EventMonitor {}
    }
}

impl SystemTrait<EngineEvent> for EventMonitor {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::ALL_EVENTS
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        trace!("Received event '{:?}'", event);
        None
    }
}
