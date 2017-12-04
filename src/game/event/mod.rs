use ecs::{EventFlag, EventTrait, EcsEvent};

pub const SHUTDOWN: EventFlag = 0b1;
pub const IMMEDIATE_SHUTDOWN: EventFlag = 0b10;
pub const READY: EventFlag = 0b100;

#[derive(Debug, Clone)]
pub enum Event {
    Shutdown,
    ImmediateShutdown,
    Ready,
}

impl EventTrait for Event {
    fn match_filter(&self, filter: EventFlag) -> bool {
        (EventFlag::from(self.clone()) & filter) > 0
    }
    fn as_ecs_event(&self) -> Option<EcsEvent> {
        use self::Event::*;
        match *self {
            ImmediateShutdown => Some(EcsEvent::Shutdown),
            Ready => Some(EcsEvent::Ready),
            _ => None,
        }
    }
}

impl From<EcsEvent> for Event {
    fn from(value: EcsEvent) -> Event {
        match value {
            EcsEvent::Shutdown => Event::ImmediateShutdown,
            EcsEvent::Ready => Event::Ready,
        }
    }
}

impl From<Event> for EventFlag {
    fn from(value: Event) -> EventFlag {
        use self::Event::*;
        match value {
            Shutdown => SHUTDOWN,
            ImmediateShutdown => IMMEDIATE_SHUTDOWN,
            Ready => READY,
        }
    }
}
