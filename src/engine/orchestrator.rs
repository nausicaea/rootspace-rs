use std::cmp;
use std::time;

use ecs::{World, EcsEvent, EventTrait};

pub struct Orchestrator<E: EventTrait> {
    pub delta_time: time::Duration,
    pub max_frame_time: time::Duration,
    pub debug: bool,
    pub world: World<E>,
}

impl<E: EventTrait> Orchestrator<E> {
    pub fn new(debug: bool) -> Orchestrator<E> {
        Orchestrator {
            delta_time: time::Duration::from_millis(100),
            max_frame_time: time::Duration::from_millis(250),
            debug: debug,
            world: World::new(),
        }
    }
    pub fn run<F>(&mut self, init: F) where F: FnOnce(&mut Orchestrator<E>) {
        init(self);
        self.world.dispatch(EcsEvent::Ready.into());
        self.main_loop();
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
