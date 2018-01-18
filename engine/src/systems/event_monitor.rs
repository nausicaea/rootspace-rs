use ecs::{LoopStageFlag, SystemTrait, Assembly};
use event::{EngineEventFlag, EngineEvent};

/// Defines a system that logs all events on the bus to the console (log level TRACE).
#[derive(Default)]
pub struct EventMonitor;

impl EventMonitor {
    /// Creates a new `EventMonitor` instance.
    pub fn new() -> Self {
        Default::default()
    }
}

impl<A> SystemTrait<EngineEvent, A> for EventMonitor {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::ALL_EVENTS & !EngineEventFlag::CURSOR_POSITION
    }
    fn handle_event(&mut self, _: &mut Assembly, _: &mut A, event: &EngineEvent) -> Option<EngineEvent> {
        trace!("Received event '{:?}'", event);
        None
    }
}
