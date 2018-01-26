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
    /// `EventMonitor` has no requirements wrt. the `Assembly`.
    fn verify_requirements(&self, _: &Assembly) -> bool {
        true
    }
    /// `EventMonitor` subscribes to the `handle_event` call.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    /// `EventMonitor` subscribes to all events except for very frequent events.
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::ALL_EVENTS & !EngineEventFlag::CURSOR_POSITION
    }
    /// Issues a logging call (TRACE level) for each received event.
    fn handle_event(&mut self, _: &mut Assembly, _: &mut A, event: &EngineEvent) -> Option<EngineEvent> {
        trace!("Received event '{:?}'", event);
        None
    }
}
