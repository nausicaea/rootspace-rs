use std::u64;
use ecs::{EventTrait, EcsEvent};

bitflags! {
    pub struct EngineEventFlag: u64 {
        const SHUTDOWN = 0b1;
        const IMMEDIATE_SHUTDOWN = 0b10;
        const READY = 0b100;
        const CONSOLE_COMMAND = 0b1000;
        const RENDERER_READY = 0b10000;
        const ALL_EVENTS = u64::MAX;
    }
}

#[derive(Debug, Clone)]
pub enum EngineEvent {
    Shutdown,
    ImmediateShutdown,
    Ready,
    RendererReady,
    ConsoleCommand(Vec<String>),
}

impl EventTrait for EngineEvent {
    type EventFlag = EngineEventFlag;
    fn match_filter(&self, filter: EngineEventFlag) -> bool {
        filter.contains(EngineEventFlag::from(self.clone()))
    }
    fn as_ecs_event(&self) -> Option<EcsEvent> {
        use self::EngineEvent::*;
        match *self {
            Shutdown => Some(EcsEvent::Shutdown),
            ImmediateShutdown => Some(EcsEvent::ImmediateShutdown),
            Ready => Some(EcsEvent::Ready),
            RendererReady => None,
            ConsoleCommand(_) => None,
        }
    }
}

impl From<EcsEvent> for EngineEvent {
    fn from(value: EcsEvent) -> EngineEvent {
        match value {
            EcsEvent::ImmediateShutdown => EngineEvent::ImmediateShutdown,
            EcsEvent::Shutdown => EngineEvent::Shutdown,
            EcsEvent::Ready => EngineEvent::Ready,
        }
    }
}

impl From<EngineEvent> for EngineEventFlag {
    fn from(value: EngineEvent) -> EngineEventFlag {
        use self::EngineEvent::*;
        match value {
            Shutdown => EngineEventFlag::SHUTDOWN,
            ImmediateShutdown => EngineEventFlag::IMMEDIATE_SHUTDOWN,
            Ready => EngineEventFlag::READY,
            RendererReady => EngineEventFlag::RENDERER_READY,
            ConsoleCommand(_) => EngineEventFlag::CONSOLE_COMMAND,
        }
    }
}
