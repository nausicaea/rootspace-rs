use ecs::{EventFlag, EventTrait, WorldEvent};

pub const SHUTDOWN: EventFlag = 0b01;
pub const IMMEDIATE_SHUTDOWN: EventFlag = 0b10;

#[derive(Debug, Clone)]
pub enum Event {
    Shutdown,
    ImmediateShutdown,
}

impl EventTrait for Event {
    fn match_filter(&self, filter: EventFlag) -> bool {
        (EventFlag::from(self.clone()) & filter) > 0
    }
    fn as_world_event(&self) -> Option<WorldEvent> {
        use self::Event::*;
        match *self {
            ImmediateShutdown => Some(WorldEvent::Shutdown),
            _ => None,
        }
    }
}

impl From<Event> for EventFlag {
    fn from(value: Event) -> EventFlag {
        use self::Event::*;
        match value {
            Shutdown => SHUTDOWN,
            ImmediateShutdown => IMMEDIATE_SHUTDOWN,
        }
    }
}
