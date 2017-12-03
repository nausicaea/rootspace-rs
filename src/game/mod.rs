use std::cmp;
use std::time;

pub struct Orchestrator {
    delta_time: time::Duration,
    max_frame_time: time::Duration,
    debug: bool,
}

impl Orchestrator {
    pub fn new(debug: bool) -> Orchestrator {
        Orchestrator {
            delta_time: time::Duration::from_millis(100),
            max_frame_time: time::Duration::from_millis(250),
            debug: debug,
        }
    }
    pub fn run(&mut self) {
        let mut game_time = time::Duration::new(0, 0);
        let mut accumulator = time::Duration::new(0, 0);
        let mut loop_time = time::Instant::now();

        let mut running = true;
        while running {
            let frame_time = cmp::min(loop_time.elapsed(), self.max_frame_time);
            loop_time = time::Instant::now();
            accumulator += frame_time;

            while accumulator >= self.delta_time {
                // Update with (game_time, delta_time)
                game_time += self.delta_time;
                accumulator -= self.delta_time;
            }

            // Render with (game_time, delta_time)

            // Handle events and set the variable `running`.
            running = true;
        }
    }
}
