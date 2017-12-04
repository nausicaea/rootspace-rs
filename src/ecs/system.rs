use std::time::Duration;

use super::event::EventTrait;
use super::assembly::Assembly;

/// Every system encodes behaviour and every system must supply at least one of the methods
/// defined in the trait `SystemTrait`.
pub trait SystemTrait<E: EventTrait> {
    /// Returns a bitmask that corresponds to a combination of `LoopStage`s. Based on that value,
    /// `World` will thus regularly call the other methods.
    fn get_loop_stage_filter(&self) -> u8;
    /// Returns a bitmask that corresponds to a combination of events. Based on that value, `World`
    /// will call `handle_event` only with the selected events.
    fn get_event_filter(&self) -> u64 {
        unimplemented!("Did you forget to implement the get_event_filter method for your system?");
    }
    /// Processes events received by the `World`, and in turn, by the engine. May optionally return
    /// a vector of events that will be handled in the next main loop iteration.
    fn handle_event(&mut self, _entities: &mut Assembly, _event: &E) -> Option<Vec<E>> {
        unimplemented!("Did you forget to implement the handle_event method for your system?");
    }
    /// Updates the game simulation. May optionally return two vectors of events; the first of
    /// which will be dispatched immediately, while the second set will be handled in the next main
    /// loop iteration.
    fn update(&mut self, _entities: &mut Assembly, _time: &Duration, _delta_time: &Duration) -> Option<(Vec<E>, Vec<E>)> {
        unimplemented!("Did you forget to implement the update method for your system?");
    }
    /// Renders the `World` state. May optionally return a vector of events that will be handled in
    /// the next main loop iteration.
    fn render(&mut self, _entities: &mut Assembly, _time: &Duration, _delta_time: &Duration) -> Option<Vec<E>> {
        unimplemented!("Did you forget to implement the render method for your system?");
    }
}

