use std::time::Duration;

use event::EventTrait;
use loop_stage::LoopStageFlag;
use assembly::Assembly;

/// Every system encodes behaviour and every system must supply at least one of the methods
/// defined in the trait `SystemTrait`.
pub trait SystemTrait<E: EventTrait, A> {
    /// Returns `true` if the supplied assembly satisfies the requirements of the current system.
    /// Can be used to require components or specific sets of them.
    fn verify_requirements(&self, _entities: &Assembly) -> bool;
    /// Returns a bitmask that corresponds to a combination of `LoopStage`s. Based on that value,
    /// `World` will thus regularly call the other methods.
    fn get_loop_stage_filter(&self) -> LoopStageFlag;
    /// Returns a bitmask that corresponds to a combination of events. Based on that value, `World`
    /// will call `handle_event` only with the selected events.
    fn get_event_filter(&self) -> E::EventFlag {
        unimplemented!("Did you forget to implement the get_event_filter method for your system?");
    }
    /// Processes events received by the `World`, and in turn, by the engine. May optionally return
    /// two events; the first of which will be dispatched immediately, while the second set will be
    /// handled in the next main loop iteration.
    fn handle_event(&mut self, _entities: &mut Assembly, _aux: &mut A, _event: &E) -> (Option<E>, Option<E>) {
        unimplemented!("Did you forget to implement the handle_event method for your system?");
    }
    /// Updates the game simulation. May optionally return two vectors of events; the first of
    /// which will be dispatched immediately, while the second set will be handled in the next main
    /// loop iteration.
    fn update(&mut self, _entities: &mut Assembly, _aux: &mut A, _time: &Duration, _delta_time: &Duration) -> Option<(Vec<E>, Vec<E>)> {
        unimplemented!("Did you forget to implement the update method for your system?");
    }
    /// Renders the `World` state. May optionally return an event that will be handled in
    /// the next main loop iteration.
    fn render(&mut self, _entities: &Assembly, _time: &Duration, _delta_time: &Duration) -> Option<E> {
        unimplemented!("Did you forget to implement the render method for your system?");
    }
}

