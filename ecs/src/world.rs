use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use event::{EcsEvent, EventTrait};
use loop_stage::LoopStage;
use assembly::Assembly;
use system::SystemTrait;

/// Encapsulates a set of systems, entities and components that describe an abstract universe of
/// data and behaviour.
pub struct World<E: EventTrait, A: Default> {
    /// This field stores an arbitrary auxiliary object, that is passed to systems during the
    /// update and event-handling calls. Consider using this for caching, file-system persistence,
    /// global state, singleton objects, etc.
    pub aux: A,
    /// If this flag is `true`, the `World` will suspend any render calls.
    pub rendering_suspended: bool,
    /// Stores any currently queued events. These will be passed on to the relevant systems in
    /// event-handling calls.
    event_queue: VecDeque<E>,
    /// Stores all systems as boxed trait objects. Systems primarily encode behaviour.
    systems: Vec<Box<SystemTrait<E, A>>>,
    /// The `Assembly` stores entities and their components. A reference is passed to systems
    /// during the update, event-handling and render calls.
    assembly: Assembly,
}

impl<E: EventTrait, A: Default> Default for World<E, A> {
    /// Creates a default instance of `World`.
    fn default() -> Self {
        World {
            aux: Default::default(),
            rendering_suspended: Default::default(),
            event_queue: Default::default(),
            systems: Default::default(),
            assembly: Default::default(),
        }
    }
}

impl<E: EventTrait, A: Default> World<E, A> {
    /// Creates a new, empty instance of `World`.
    pub fn new() -> Self {
        Default::default()
    }
    /// Adds a new system to the `World`.
    pub fn add_system<S: SystemTrait<E, A> + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }
    /// Iterates over all queued events and dispatches them to the relevant systems.
    pub fn handle_events(&mut self) -> bool {
        let events = self.event_queue.iter().cloned().collect::<Vec<_>>();
        self.event_queue.clear();

        for e in events {
            match e.as_ecs_event() {
                Some(EcsEvent::ImmediateShutdown) => {
                    return false;
                },
                Some(EcsEvent::Shutdown) => {
                    self.dispatch_immediate(&e);
                    self.dispatch(EcsEvent::ImmediateShutdown.into())
                },
                Some(EcsEvent::Suspend(v)) => self.rendering_suspended = v,
                _ => self.dispatch_immediate(&e),
            }
        }

        true
    }
    /// Updates the current simulation of the `World` by iterating through all systems that
    /// subscribe to the update call.
    pub fn update(&mut self, time: &Duration, delta_time: &Duration) {
        let mut priority_events = Vec::new();
        let mut events = Vec::new();

        for system in &mut self.systems {
            if LoopStage::Update.match_filter(system.get_loop_stage_filter()) {
                if let Some((mut pe, mut e)) = system.update(&mut self.assembly, &mut self.aux, time, delta_time) {
                    priority_events.append(&mut pe);
                    events.append(&mut e);
                }
            }
        }

        for pe in priority_events {
            self.dispatch_immediate(&pe);
        }
        for e in events {
            self.dispatch(e);
        }
    }
    /// Renders the current state of the `World` by iterating through all systems that subscribe to
    /// the render call.
    pub fn render(&mut self, time: &Duration, delta_time: &Duration) {
        if !self.rendering_suspended {
            let mut events = Vec::new();

            for system in &mut self.systems {
                if LoopStage::Render.match_filter(system.get_loop_stage_filter()) {
                    if let Some(e) = system.render(&self.assembly, time, delta_time) {
                        events.push(e);
                    }
                }
            }

            for e in events {
                self.dispatch(e);
            }
        }
    }
    /// Sends an event to the queue for later processing.
    pub fn dispatch(&mut self, event: E) {
        self.event_queue.push_back(event);
    }
    /// In its functioning analogous to `update` and `render`.
    fn dispatch_immediate(&mut self, event: &E) {
        let mut events = Vec::new();

        for system in &mut self.systems {
            if LoopStage::HandleEvent.match_filter(system.get_loop_stage_filter()) && event.match_filter(system.get_event_filter()) {
                if let Some(e) = system.handle_event(&mut self.assembly, &mut self.aux, event) {
                    events.push(e);
                }
            }
        }

        for e in events {
            self.dispatch(e);
        }
    }
}

impl<E: EventTrait, F: Default> Deref for World<E, F> {
    type Target = Assembly;

    fn deref(&self) -> &Self::Target {
        &self.assembly
    }
}

impl<E: EventTrait, F: Default> DerefMut for World<E, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.assembly
    }
}
