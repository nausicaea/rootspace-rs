use ecs::{EventTrait, WorldEvent};

pub const SHUTDOWN: u64 = 0b01;
pub const IMMEDIATE_SHUTDOWN: u64 = 0b10;

#[derive(Debug, Clone)]
pub enum Event {
    Shutdown,
    ImmediateShutdown,
}

impl EventTrait for Event {
    fn match_filter(&self, filter: u64) -> bool {
        let value: u64 = self.clone().into();
        (value & filter) > 0
    }
    fn as_world_event(&self) -> Option<WorldEvent> {
        use self::Event::*;
        match *self {
            ImmediateShutdown => Some(WorldEvent::Shutdown),
            _ => None,
        }
    }
}

impl From<Event> for u64 {
    fn from(value: Event) -> u64 {
        use self::Event::*;
        match value {
            Shutdown => SHUTDOWN,
            ImmediateShutdown => IMMEDIATE_SHUTDOWN,
        }
    }
}
