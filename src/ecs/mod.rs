/// Returns the names of multiple types as String.
macro_rules! type_names {
    ($t:tt) => {{
        use std::intrinsics::type_name;
        String::from(unsafe {type_name::<$t>()})
    }};
    ($($t:tt),*) => {{
        use std::intrinsics::type_name;
        let mut buf = String::from("(");
        $(buf.push_str(unsafe {type_name::<$t>()}); buf.push_str(", ");)*
        buf
    }};
}

mod error;
mod event;
mod loop_stage;
mod entity;
mod system;
mod component_group;
mod assembly;
mod world;

pub use self::error::EcsError;
pub use self::event::{ALL_EVENTS, EventFlag, WorldEvent, EventTrait};
pub use self::loop_stage::{LoopStageFlag, HANDLE_EVENT, UPDATE, RENDER, ALL_STAGES, LoopStage};
pub use self::entity::Entity;
pub use self::system::SystemTrait;
pub use self::component_group::ComponentGroup;
pub use self::assembly::Assembly;
pub use self::world::World;
