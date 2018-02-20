use std::cmp;
use std::path::{PathBuf, Path};
use std::time::{Instant, Duration};

use ecs::World;
use event::EngineEvent;
use singletons::Singletons;
use systems::SystemGroup;
use common::file_manipulation::{verify_accessible_file, FileError};

/// The `Orchestrator` owns the `World` and manages time (and the game loop).
pub struct Orchestrator {
    /// Holds an instance of the `World`.
    pub world: World<EngineEvent, Singletons, SystemGroup>,
    /// If `true`, activate debugging functionality.
    pub debug: bool,
    /// Specifies the path to the resource tree.
    resource_path: PathBuf,
    /// Specifies the fixed time interval of the simulation.
    delta_time: Duration,
    /// Specifies the maximum duration of a single frame.
    max_frame_time: Duration,
}

impl Orchestrator {
    /// Creates a new instance of the `Orchestrator`.
    pub fn new(rp: &Path, delta_time: Duration, max_frame_time: Duration, debug: bool) -> Self {
        Orchestrator {
            world: Default::default(),
            debug: debug,
            resource_path: rp.to_owned(),
            delta_time: delta_time,
            max_frame_time: max_frame_time,
        }
    }
    /// Initializes state and starts the game loop. Using the supplied closure, the state of the
    /// `Orchestrator` and subsequently the `World` may be initialized.
    pub fn run<I>(&mut self, init: I) where I: FnOnce(&mut Orchestrator) {
        init(self);
        self.world.dispatch(EngineEvent::Ready);
        self.main_loop();
    }
    /// Attempts to retrieve a file path from the resource tree.
    pub fn get_file(&self, category: &str, filename: &str) -> Result<PathBuf, FileError> {
        let path = self.resource_path.join(category).join(filename);
        verify_accessible_file(&path)?;
        Ok(path)
    }
    /// Runs the actual game loop. This loop uses a fixed time-step method to ensure that
    /// `World::update` is called at a fixed interval, always. After a cycle of update calls,
    /// the loop then issues calls to `World::render` and `World::handle_events`. The return value
    /// of `World::handle_events` is used to terminate the loop.
    fn main_loop(&mut self) {
        let mut game_time = Duration::new(0, 0);
        let mut accumulator = Duration::new(0, 0);
        let mut loop_time = Instant::now();

        let mut running = true;
        while running {
            let frame_time = cmp::min(loop_time.elapsed(), self.max_frame_time);
            loop_time = Instant::now();
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
