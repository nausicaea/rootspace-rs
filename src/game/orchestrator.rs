use std::cmp;
use std::time;

use ecs::World;
use super::event::Event;
use super::debugging::event_monitor::EventMonitor;

pub struct Orchestrator {
    delta_time: time::Duration,
    max_frame_time: time::Duration,
    debug: bool,
    world: World<Event>,
}

impl Orchestrator {
    pub fn new(debug: bool) -> Orchestrator {
        Orchestrator {
            delta_time: time::Duration::from_millis(100),
            max_frame_time: time::Duration::from_millis(250),
            debug: debug,
            world: World::new(),
        }
    }
    pub fn run(&mut self) {
        self.initialize();
        self.main_loop();
    }
    fn initialize(&mut self) {
        if self.debug {
            self.world.add_system(EventMonitor::new());
        }

        self.world.dispatch(Event::Shutdown);
    }
    fn main_loop(&mut self) {
        let mut game_time = time::Duration::new(0, 0);
        let mut accumulator = time::Duration::new(0, 0);
        let mut loop_time = time::Instant::now();

        let mut running = true;
        while running {
            let frame_time = cmp::min(loop_time.elapsed(), self.max_frame_time);
            loop_time = time::Instant::now();
            accumulator += frame_time;

            while accumulator >= self.delta_time {
                self.world.update(&game_time, &self.delta_time);
                game_time += self.delta_time;
                accumulator -= self.delta_time;
            }

            self.world.render(&game_time, &self.delta_time);

            running = self.world.handle_events();
        }
    }
}
