/// `WorldEvent` defines a set of foundational events that will cause `World` to do different
/// things, independent of the actual events later used by the engine.
#[derive(Debug, Clone)]
pub enum WorldEvent {
    Shutdown,
}

/// Every engine event must implement the trait `EventTrait`, as some events must be converted to
/// `WorldEvent`s for `World` to interact properly with the engine.
pub trait EventTrait: Clone + Into<u64> {
    /// Returns true if the specified filter selects the current enum variant.
    fn match_filter(&self, filter: u64) -> bool;
    /// Attempts to convert to a `WorldEvent`.
    fn as_world_event(&self) -> Option<WorldEvent>;
}
