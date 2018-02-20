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
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate num_traits;
extern crate nalgebra;
extern crate image;
extern crate uuid;
extern crate unicode_normalization;
extern crate rusttype;
extern crate daggy;
#[macro_use]
extern crate ecs;
#[macro_use]
extern crate ecs_derive;

pub mod event;
pub mod singletons;
pub mod orchestrator;
pub mod common;
pub mod components;
pub mod systems;

pub use self::event::EngineEvent;
pub use self::common::ui_styles::{SpeechBubble, Tooltip};
pub use self::common::resource_group::{FontGroup, ShaderGroup, TextureGroup};
pub use self::components::bounding_volume::BoundingVolume;
pub use self::components::camera::Camera;
pub use self::components::cursor::Cursor;
pub use self::components::description::Description;
pub use self::components::material::Material;
pub use self::components::mesh::Mesh;
pub use self::components::model::Model;
pub use self::components::tooltip::TooltipData;
pub use self::components::ui_state::UiState;
pub use self::systems::SystemGroup;
pub use self::systems::cursor_controller::CursorController;
pub use self::systems::debug_mover::DebugMover;
pub use self::systems::debug_console::DebugConsole;
pub use self::systems::debug_shell::DebugShell;
pub use self::systems::event_interface::EventInterface;
pub use self::systems::event_monitor::EventMonitor;
pub use self::systems::renderer::Renderer;
pub use self::systems::tooltip_controller::TooltipController;
pub use self::systems::speech_bubble_controller::SpeechBubbleController;
pub use self::orchestrator::Orchestrator;
