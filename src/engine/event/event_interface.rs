use std::time::Duration;
use glium::glutin::{Event, WindowEvent, EventsLoop};
use ecs::{SystemTrait, LoopStageFlag, Assembly};
use super::EngineEvent;

pub struct EventInterface {
    pub events_loop: EventsLoop,
}

impl EventInterface {
    pub fn new() -> EventInterface {
        EventInterface {
            events_loop: EventsLoop::new(),
        }
    }
}

impl SystemTrait<EngineEvent> for EventInterface {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::UPDATE
    }
    fn update(&mut self, _: &mut Assembly, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        let mut pd = Vec::new();
        let mut d = Vec::new();

        self.events_loop.poll_events(|ge| {
            match ge {
                Event::WindowEvent {window_id: _, event: we} => match we {
                    WindowEvent::Closed => d.push(EngineEvent::Shutdown),
                    _ => (),
                },
                Event::Suspended(v) => d.push(EngineEvent::Suspend(v)),
                _ => (),
            }
        });

        if pd.len() > 0 || d.len() > 0 {
            Some((pd, d))
        } else {
            None
        }
    }
}
