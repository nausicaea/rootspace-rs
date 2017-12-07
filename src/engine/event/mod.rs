use ecs::{EventFlag, EventTrait, EcsEvent};

pub const SHUTDOWN: EventFlag = 0b1;
pub const IMMEDIATE_SHUTDOWN: EventFlag = 0b10;
pub const READY: EventFlag = 0b100;
pub const CONSOLE_COMMAND: EventFlag = 0b1000;

#[derive(Debug, Clone)]
pub enum EngineEvent {
    Shutdown,
    ImmediateShutdown,
    Ready,
    ConsoleCommand(Vec<String>),
}

impl EventTrait for EngineEvent {
    fn match_filter(&self, filter: EventFlag) -> bool {
        (EventFlag::from(self.clone()) & filter) > 0
    }
    fn as_ecs_event(&self) -> Option<EcsEvent> {
        use self::EngineEvent::*;
        match *self {
            Shutdown => Some(EcsEvent::Shutdown),
            ImmediateShutdown => Some(EcsEvent::ImmediateShutdown),
            Ready => Some(EcsEvent::Ready),
            ConsoleCommand(ref c) => Some(EcsEvent::ConsoleCommand(c.clone())),
        }
    }
}

impl From<EcsEvent> for EngineEvent {
    fn from(value: EcsEvent) -> EngineEvent {
        match value {
            EcsEvent::ImmediateShutdown => EngineEvent::ImmediateShutdown,
            EcsEvent::Shutdown => EngineEvent::Shutdown,
            EcsEvent::Ready => EngineEvent::Ready,
            EcsEvent::ConsoleCommand(c) => EngineEvent::ConsoleCommand(c),
        }
    }
}

impl From<EngineEvent> for EventFlag {
    fn from(value: EngineEvent) -> EventFlag {
        use self::EngineEvent::*;
        match value {
            Shutdown => SHUTDOWN,
            ImmediateShutdown => IMMEDIATE_SHUTDOWN,
            Ready => READY,
            ConsoleCommand(_) => CONSOLE_COMMAND,
        }
    }
}
