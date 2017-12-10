use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use super::event::{EcsEvent, EventTrait};
use super::loop_stage::LoopStage;
use super::assembly::Assembly;
use super::system::SystemTrait;

/// Encapsulates a set of systems, entities and components that describe an abstract universe of
/// data and behaviour.
pub struct World<E: EventTrait> {
    event_queue: VecDeque<E>,
    systems: Vec<Box<SystemTrait<E>>>,
    assembly: Assembly,
    rendering_suspended: bool,
}

impl<E: EventTrait> World<E> {
    /// Creates a new, empty instance of `World`.
    pub fn new() -> Self {
        World {
            event_queue: VecDeque::new(),
            systems: Vec::new(),
            assembly: Assembly::new(),
            rendering_suspended: false,
        }
    }
    /// Adds a new system to the `World`.
    pub fn add_system<S: SystemTrait<E> + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }
    /// Iterates over all queued events and dispatches them to the relevant systems.
    pub fn handle_events(&mut self) -> bool {
        let events = self.event_queue.iter().cloned().collect::<Vec<_>>();
        self.event_queue.clear();

        for e in events.into_iter() {
            match e.as_ecs_event() {
                Some(EcsEvent::ImmediateShutdown) => {
                    info!("Shutting down now! At this point, all unsaved state is lost.");
                    return false
                },
                Some(EcsEvent::Shutdown) => {
                    self.dispatch_immediate(e);
                    self.dispatch(EcsEvent::ImmediateShutdown.into())
                },
                Some(EcsEvent::Suspend(v)) => self.rendering_suspended = v,
                _ => self.dispatch_immediate(e),
            }
        }

        true
    }
    /// Updated the current simulation of the `World`.
    pub fn update(&mut self, time: &Duration, delta_time: &Duration) {
        let mut priority_events = Vec::new();
        let mut events = Vec::new();

        for system in self.systems.iter_mut() {
            if LoopStage::Update.match_filter(system.get_loop_stage_filter()) {
                match system.update(&mut self.assembly, time, delta_time) {
                    Some((mut pe, mut e)) => {
                        priority_events.append(&mut pe);
                        events.append(&mut e);
                    },
                    None => (),
                }
            }
        }

        for pe in priority_events.into_iter() {
            self.dispatch_immediate(pe);
        }
        for e in events.into_iter() {
            self.dispatch(e);
        }
    }
    /// Renders the current state of the `World`.
    pub fn render(&mut self, time: &Duration, delta_time: &Duration) {
        if !self.rendering_suspended {
            let mut events = Vec::new();

            for system in self.systems.iter_mut() {
                if LoopStage::Render.match_filter(system.get_loop_stage_filter()) {
                    match system.render(&mut self.assembly, time, delta_time) {
                        Some(e) => {
                            events.push(e);
                        },
                        None => (),
                    }
                }
            }

            for e in events.into_iter() {
                self.dispatch(e);
            }
        }
    }
    /// Sends an event to the queue.
    pub fn dispatch(&mut self, event: E) {
        self.event_queue.push_back(event);
    }
    /// In its functioning analogous to `update` and `render`.
    fn dispatch_immediate(&mut self, event: E) {
        let mut events = Vec::new();

        for system in self.systems.iter_mut() {
            if LoopStage::HandleEvent.match_filter(system.get_loop_stage_filter()) {
                if event.match_filter(system.get_event_filter()) {
                    match system.handle_event(&mut self.assembly, &event) {
                        Some(e) => {
                            events.push(e);
                        },
                        None => (),
                    }
                }
            }
        }

        for e in events.into_iter() {
            self.dispatch(e);
        }
    }
}

impl<E: EventTrait> Deref for World<E> {
    type Target = Assembly;

    fn deref(&self) -> &Self::Target {
        &self.assembly
    }
}

impl<E: EventTrait> DerefMut for World<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.assembly
    }
}
