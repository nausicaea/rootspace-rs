#![allow(dead_code)]

//! The `ecs` module provides functionality to represent an Entity-Component-System architecture.
//! The `World` collects multiple boxed `SystemTrait`s which operate on the `ComponentTrait`s of
//! the registered `Entities` (in an `Assembly`) via three main loop stages: event handling,
//! simulation updates, and state rendering. Systems (types that implement `SystemTrait`) may be
//! added dynamically to `World`, as well as components (types that implement `ComponentTrait`. The
//! latter must be linked to an `Entity` however, which must first be created by the `Assembly` (or
//! the `World` via Deref trait).

/// Returns the names of multiple types as String. Uses `std::intrinsics::type_name` and thus
/// requires the unstable feature `core_intrinsics` only available on rust nightly. Without this,
/// ECS errors are practically impossible to diagnose.
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
pub use self::event::{EcsEvent, EventTrait};
pub use self::loop_stage::{LoopStageFlag, LoopStage};
pub use self::entity::Entity;
pub use self::system::SystemTrait;
pub use self::component_group::{ComponentTrait, ComponentGroup};
pub use self::assembly::Assembly;
pub use self::world::World;
