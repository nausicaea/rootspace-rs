extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate bitflags;
extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate glium;
extern crate alga;
extern crate nalgebra;
extern crate image;
extern crate uuid;
extern crate unicode_normalization;
extern crate rusttype;
extern crate ecs;

mod utilities;
mod event;
mod debugging;
mod geometry;
mod graphics;
mod interface;
mod factory;
mod orchestrator;

pub use self::event::EngineEvent;
pub use self::event::event_interface::EventInterface;
pub use self::debugging::event_monitor::EventMonitor;
pub use self::debugging::debug_console::DebugConsole;
pub use self::debugging::debug_shell::DebugShell;
pub use self::debugging::description::Description;
pub use self::geometry::projection::Projection;
pub use self::geometry::view::View;
pub use self::geometry::model::Model;
pub use self::graphics::renderer::Renderer;
pub use self::graphics::mesh::Mesh;
pub use self::graphics::material::Material;
pub use self::interface::user_interface::UserInterface;
pub use self::interface::ui_state::UiState;
pub use self::interface::ui_styles::{Common, SpeechBubble};
pub use self::factory::ComponentFactory;
pub use self::orchestrator::Orchestrator;
