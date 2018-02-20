use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use event::{EcsEvent, EventTrait};
use loop_stage::LoopStage;
use assembly::Assembly;
use system::SystemTrait;
use error::EcsError;

/// Encapsulates a set of systems, entities and components that describe an abstract universe of
/// data and behaviour.
pub struct World<E: EventTrait, A: Default, S: SystemTrait<E, A>> {
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
    systems: Vec<S>,
    /// The `Assembly` stores entities and their components. A reference is passed to systems
    /// during the update, event-handling and render calls.
    assembly: Assembly,
}

impl<E: EventTrait, A: Default, S: SystemTrait<E, A>> Default for World<E, A, S> {
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

impl<E: EventTrait, A: Default, S: SystemTrait<E, A>> World<E, A, S> {
    /// Creates a new, empty instance of `World`.
    pub fn new() -> Self {
        Default::default()
    }
    /// Adds a new system to the `World`.
    pub fn add_system(&mut self, system: S) -> Result<(), EcsError> {
        if system.verify_requirements(&self.assembly) {
            self.systems.push(system);
            Ok(())
        } else {
            Err(EcsError::UnsatisfiedRequirements)
        }
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
                _ => self.dispatch_immediate(&e),
            }
        }

        true
    }
    /// Updates the current simulation of the `World` by iterating through all systems that
    /// subscribe to the update call. This update call should be performed at fixed time steps.
    pub fn update(&mut self, time: &Duration, delta_time: &Duration) {
        let mut priority_events = Vec::new();
        let mut events = Vec::new();

        for system in &mut self.systems {
            if LoopStage::Update.match_filter(system.get_loop_stage_filter()) {
                let (pe, e) = system.update(&mut self.assembly, &mut self.aux, time, delta_time);

                if let Some(mut pe) = pe {
                    priority_events.append(&mut pe);
                }
                if let Some(mut e) = e {
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
    /// Updates the current simulation of the `World` by iterating through all systems that
    /// subscribe to the update call. This update call should be performed at variable time steps.
    pub fn dynamic_update(&mut self, time: &Duration, delta_time: &Duration) {
        let mut priority_events = Vec::new();
        let mut events = Vec::new();

        for system in &mut self.systems {
            if LoopStage::DynamicUpdate.match_filter(system.get_loop_stage_filter()) {
                let (pe, e) = system.dynamic_update(&mut self.assembly, &mut self.aux, time, delta_time);

                if let Some(mut pe) = pe {
                    priority_events.append(&mut pe);
                }
                if let Some(mut e) = e {
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
            for system in &mut self.systems {
                if LoopStage::Render.match_filter(system.get_loop_stage_filter()) {
                    system.render(&self.assembly, &mut self.aux, time, delta_time);
                }
            }
        }
    }
    /// Sends an event to the queue for later processing.
    pub fn dispatch(&mut self, event: E) {
        self.event_queue.push_back(event);
    }
    /// Processes the current event by iterating over all applicable systems (e.g. they subscribe
    /// to the event handling call and also to the current event).
    fn dispatch_immediate(&mut self, event: &E) {
        let mut priority_events = Vec::new();
        let mut events = Vec::new();

        for system in &mut self.systems {
            if LoopStage::HandleEvent.match_filter(system.get_loop_stage_filter()) && event.match_filter(system.get_event_filter()) {
                let (pe, e) = system.handle_event(&mut self.assembly, &mut self.aux, event);

                if let Some(mut pe) = pe {
                    priority_events.append(&mut pe);
                }
                if let Some(mut e) = e {
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
}

impl<E: EventTrait, A: Default, S: SystemTrait<E, A>> Deref for World<E, A, S> {
    type Target = Assembly;

    fn deref(&self) -> &Self::Target {
        &self.assembly
    }
}

impl<E: EventTrait, A: Default, S: SystemTrait<E, A>> DerefMut for World<E, A, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.assembly
    }
}
