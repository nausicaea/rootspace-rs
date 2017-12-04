use std::cmp;
use std::time;

use ecs::{World, EcsEvent, EventTrait};

/// The `Orchestrator` owns the `World` and manages time (and the game loop).
pub struct Orchestrator<E: EventTrait> {
    /// Specifies the fixed time interval of the simulation.
    pub delta_time: time::Duration,
    /// Specifies the maximum duration of a single frame.
    pub max_frame_time: time::Duration,
    /// If `true`, activate debugging functionality.
    pub debug: bool,
    /// Holds an instance of the `World`.
    pub world: World<E>,
}

impl<E: EventTrait> Orchestrator<E> {
    /// Creates a new instance of the `Orchestrator`.
    pub fn new(debug: bool) -> Orchestrator<E> {
        Orchestrator {
            delta_time: time::Duration::from_millis(100),
            max_frame_time: time::Duration::from_millis(250),
            debug: debug,
            world: World::new(),
        }
    }
    /// Initializes state and starts the game loop. Using the supplied closure, the state of the
    /// `Orchestrator` and subsequently the `World` may be initialized.
    pub fn run<F>(&mut self, init: F) where F: FnOnce(&mut Orchestrator<E>) {
        init(self);
        self.world.dispatch(EcsEvent::Ready.into());
        self.world.dispatch(EcsEvent::Shutdown.into());
        self.main_loop();
    }
    /// Runs the actual game loop. This loop uses a fixed time-step method to ensure that
    /// `World::update` is called at a fixed interval, always. After a cycle of update calls,
    /// the loop then issues calls to `World::render` and `World::handle_events`. The return value
    /// of `World::handle_events` is used to terminate the loop.
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
