pub mod cursor_controller;
pub mod debug_console;
pub mod debug_mover;
pub mod debug_shell;
pub mod event_monitor;
pub mod event_interface;
pub mod renderer;
pub mod tooltip_controller;
pub mod speech_bubble_controller;

use event::{EngineEvent, EngineEventFlag};
use singletons::Singletons;

impl_system_group! {
    pub enum SystemGroup<EngineEvent, EngineEventFlag, Singletons> {
        CursorControllerSys(cursor_controller::CursorController),
        DebugConsoleSys(debug_console::DebugConsole),
        DebugMoverSys(debug_mover::DebugMover),
        DebugShellSys(debug_shell::DebugShell),
        EventMonitorSys(event_monitor::EventMonitor),
        EventInterfaceSys(event_interface::EventInterface),
        RendererSys(renderer::Renderer),
        TooltipControllerSys(tooltip_controller::TooltipController),
        SpeechBubbleControllerSys(speech_bubble_controller::SpeechBubbleController),
    }
}
