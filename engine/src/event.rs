use std::u64;
use nalgebra::Point2;
use glium::glutin::{MouseButton, ElementState};
use ecs::{EventTrait, EcsEvent};

bitflags! {
    pub struct EngineEventFlag: u64 {
        const SHUTDOWN = 0x01;
        const IMMEDIATE_SHUTDOWN = 0x02;
        const READY = 0x04;
        const CONSOLE_COMMAND = 0x08;
        const RENDERER_READY = 0x10;
        const RESIZE_WINDOW = 0x20;
        const RELOAD_SHADERS = 0x40;
        const SPEECH_BUBBLE = 0x80;
        const CURSOR_POSITION = 0x100;
        const MOUSE_INPUT = 0x200;
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
    ResizeWindow(u32, u32),
    ReloadShaders,
    SpeechBubble(String, String, u64),
    CursorPosition(Point2<u32>),
    MouseInput(MouseButton, ElementState),
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
            ResizeWindow(..) => EngineEventFlag::RESIZE_WINDOW,
            ReloadShaders => EngineEventFlag::RELOAD_SHADERS,
            SpeechBubble(..) => EngineEventFlag::SPEECH_BUBBLE,
            CursorPosition(_) => EngineEventFlag::CURSOR_POSITION,
            MouseInput(..) => EngineEventFlag::MOUSE_INPUT,
        }
    }
}
