use std::u64;
use ecs::{EventTrait, EcsEvent};

bitflags! {
    pub struct EngineEventFlag: u64 {
        const SHUTDOWN = 0x01;
        const IMMEDIATE_SHUTDOWN = 0x02;
        const READY = 0x04;
        const CONSOLE_COMMAND = 0x08;
        const RENDERER_READY = 0x10;
        const SUSPEND = 0x20;
        const RESIZE_WINDOW = 0x40;
        const RELOAD_SHADERS = 0x80;
        const SPEECH_BUBBLE = 0x100;
        const ALL_EVENTS = u64::MAX;
    }
}

#[derive(Debug, Clone)]
pub enum EngineEvent {
    Shutdown,
    ImmediateShutdown,
    Ready,
    Suspend(bool),
    RendererReady,
    ConsoleCommand(Vec<String>),
    ResizeWindow(u32, u32),
    ReloadShaders,
    SpeechBubble(String, String, u64),
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
            Suspend(v) => Some(EcsEvent::Suspend(v)),
            _ => None,
        }
    }
}

impl From<EcsEvent> for EngineEvent {
    fn from(value: EcsEvent) -> EngineEvent {
        match value {
            EcsEvent::ImmediateShutdown => EngineEvent::ImmediateShutdown,
            EcsEvent::Shutdown => EngineEvent::Shutdown,
            EcsEvent::Ready => EngineEvent::Ready,
            EcsEvent::Suspend(v) => EngineEvent::Suspend(v),
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
            Suspend(_) => EngineEventFlag::SUSPEND,
            RendererReady => EngineEventFlag::RENDERER_READY,
            ConsoleCommand(_) => EngineEventFlag::CONSOLE_COMMAND,
            ResizeWindow(..) => EngineEventFlag::RESIZE_WINDOW,
            ReloadShaders => EngineEventFlag::RELOAD_SHADERS,
            SpeechBubble(..) => EngineEventFlag::SPEECH_BUBBLE,
        }
    }
}