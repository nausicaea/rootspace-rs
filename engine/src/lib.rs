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
extern crate petgraph;
extern crate ecs;

mod utilities;
mod event;
mod factory;
mod scene_graph;
mod orchestrator;
mod common;
mod components;
mod systems;

pub use self::event::EngineEvent;
pub use self::common::ui_styles::{Common, SpeechBubble};
pub use self::components::description::Description;
pub use self::components::projection::Projection;
pub use self::components::view::View;
pub use self::components::model::Model;
pub use self::components::mesh::Mesh;
pub use self::components::material::Material;
pub use self::components::ui_state::UiState;
pub use self::systems::event_interface::EventInterface;
pub use self::systems::event_monitor::EventMonitor;
pub use self::systems::debug_console::DebugConsole;
pub use self::systems::debug_shell::DebugShell;
pub use self::systems::renderer::Renderer;
pub use self::systems::user_interface::UserInterface;
pub use self::factory::ComponentFactory;
pub use self::orchestrator::Orchestrator;
