use std::fmt;
use std::u64;

/// `EcsEvent` defines a set of foundational events that will cause `World` to do different
/// things, independent of the actual events later used by the engine.
#[derive(Debug, Clone)]
pub enum EcsEvent {
    Shutdown,
    ImmediateShutdown,
    Ready,
}

pub type EventFlag = u64;

pub const ALL_EVENTS: EventFlag = u64::MAX;

/// Every engine event must implement the trait `EventTrait`, as some events must be converted to
/// `EcsEvent`s for `World` to interact properly with the engine.
pub trait EventTrait: fmt::Debug + Clone + From<EcsEvent> {
    /// Returns true if the specified filter selects the current enum variant.
    fn match_filter(&self, filter: EventFlag) -> bool;
    /// Attempts to convert to a `EcsEvent`.
    fn as_ecs_event(&self) -> Option<EcsEvent>;
}
