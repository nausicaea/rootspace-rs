/// `EcsEvent` defines a set of foundational events that will cause `World` to do different
/// things, independent of the actual events later used by the engine.
#[derive(Debug, Clone)]
pub enum EcsEvent {
    Shutdown,
    ImmediateShutdown,
    Ready,
}

/// Every engine event must implement the trait `EventTrait`, as some events must be converted to
/// `EcsEvent`s for `World` to interact properly with the engine.
pub trait EventTrait: Clone + From<EcsEvent> {
    /// Specifies a type (ex. bitflags) that may select multiple events.
    type EventFlag;
    /// Returns true if the specified filter selects the current enum variant.
    fn match_filter(&self, filter: Self::EventFlag) -> bool;
    /// Attempts to convert to a `EcsEvent`.
    fn as_ecs_event(&self) -> Option<EcsEvent>;
}
