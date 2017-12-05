use ecs::{EventFlag, EventTrait, EcsEvent};

pub const SHUTDOWN: EventFlag = 0b1;
pub const IMMEDIATE_SHUTDOWN: EventFlag = 0b10;
pub const READY: EventFlag = 0b100;
pub const CONSOLE_COMMAND: EventFlag = 0b1000;

#[derive(Debug, Clone)]
pub enum Event {
    Shutdown,
    ImmediateShutdown,
    Ready,
    ConsoleCommand(Vec<String>),
}

impl EventTrait for Event {
    fn match_filter(&self, filter: EventFlag) -> bool {
        (EventFlag::from(self.clone()) & filter) > 0
    }
    fn as_ecs_event(&self) -> Option<EcsEvent> {
        use self::Event::*;
        match *self {
            Shutdown => Some(EcsEvent::Shutdown),
            ImmediateShutdown => Some(EcsEvent::ImmediateShutdown),
            Ready => Some(EcsEvent::Ready),
            ConsoleCommand(ref c) => Some(EcsEvent::ConsoleCommand(c.clone())),
        }
    }
}

impl From<EcsEvent> for Event {
    fn from(value: EcsEvent) -> Event {
        match value {
            EcsEvent::ImmediateShutdown => Event::ImmediateShutdown,
            EcsEvent::Shutdown => Event::Shutdown,
            EcsEvent::Ready => Event::Ready,
            EcsEvent::ConsoleCommand(c) => Event::ConsoleCommand(c),
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
            ConsoleCommand(_) => CONSOLE_COMMAND,
        }
    }
}
