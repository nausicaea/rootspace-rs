use std::time::Duration;
use glium::glutin::{Event, WindowEvent, EventsLoop};
use ecs::{SystemTrait, LoopStageFlag, Assembly};
use event::EngineEvent;

/// The task of the `EventInterface` is to regularly poll for events from the operating system and
/// graphical backend. Any events of interest are then sent off to the event bus of `World`.
pub struct EventInterface {
    /// Provides access to the `EventsLoop`.
    pub events_loop: EventsLoop,
}

impl Default for EventInterface {
    fn default() -> Self {
        EventInterface {
            events_loop: EventsLoop::new(),
        }
    }
}


impl EventInterface {
    /// Creates a new `EventInterface` instance.
    pub fn new() -> Self {
        Default::default()
    }
}

impl<F> SystemTrait<EngineEvent, F> for EventInterface {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::UPDATE
    }
    fn update(&mut self, _: &mut Assembly, _: &mut F, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        let pd = Vec::new();
        let mut d = Vec::new();

        self.events_loop.poll_events(|ge| {
            match ge {
                Event::WindowEvent {event: we, ..} => match we {
                    WindowEvent::Closed => d.push(EngineEvent::Shutdown),
                    WindowEvent::Resized(w, h) => d.push(EngineEvent::ResizeWindow(w, h)),
                    _ => (),
                },
                Event::Suspended(v) => d.push(EngineEvent::Suspend(v)),
                _ => (),
            }
        });

        if !(pd.is_empty() && d.is_empty()) {
            Some((pd, d))
        } else {
            None
        }
    }
}
