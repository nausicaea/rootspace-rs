#![feature(core_intrinsics)]

mod ecs;
mod game;

use game::Orchestrator;

fn main() {
    let debug = true;

    let mut orchestrator = Orchestrator::new(debug);
    orchestrator.run();
}
