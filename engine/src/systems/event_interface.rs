use std::time::Duration;
use nalgebra::Point2;
use glium::glutin::{Event, EventsLoop, WindowEvent};
use ecs::{Assembly, DispatchEvents, LoopStageFlag, SystemTrait};
use singletons::Singletons;
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

impl SystemTrait<EngineEvent, Singletons> for EventInterface {
    /// `EventInterface` does not have any requirements wrt. to the `Assembly`.
    fn verify_requirements(&self, _: &Assembly) -> bool {
        true
    }
    /// `EventInterface` subscribes to the update call.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::DYNAMIC_UPDATE
    }
    /// Polls for operating system events and relays them to the ECS event queue.
    fn dynamic_update(
        &mut self,
        _: &mut Assembly,
        _: &mut Singletons,
        _: &Duration,
        _: &Duration,
    ) -> DispatchEvents<EngineEvent> {
        let mut pd = Vec::new();
        let mut d = Vec::new();

        self.events_loop.poll_events(|ge| {
            if let Event::WindowEvent { event: we, .. } = ge {
                match we {
                    WindowEvent::Closed => d.push(EngineEvent::Shutdown),
                    WindowEvent::Resized(w, h) => d.push(EngineEvent::ResizeWindow(w, h)),
                    WindowEvent::CursorMoved {
                        position: (x, y), ..
                    } => {
                        // Convert the coordinates to pixels.
                        let x = x.floor() as u32;
                        let y = y.floor() as u32;

                        // Dispatch the cursor movement event.
                        pd.push(EngineEvent::CursorPosition(Point2::new(x, y)));
                    }
                    WindowEvent::MouseInput {
                        state: s,
                        button: b,
                        ..
                    } => {
                        // Dispatch the mouse input event.
                        pd.push(EngineEvent::MouseInput(b, s));
                    }
                    _ => (),
                }
            }
        });

        let pd = if pd.is_empty() { None } else { Some(pd) };
        let d = if d.is_empty() { None } else { Some(d) };
        (pd, d)
    }
}
