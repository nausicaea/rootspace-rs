#![feature(core_intrinsics)]

extern crate clap;

mod ecs;
mod game;

use clap::{Arg, App};
use game::Orchestrator;

fn main() {
    let matches = App::new("Rootspace")
        .version("0.1.0")
        .author("Eleanore Young")
        .about("A game wrapped in uncertainty and mis(t)ery")
        .arg(Arg::with_name("debugging")
             .short("d")
             .long("debug")
             .help("Enables debugging features"))
        .arg(Arg::with_name("verbosity")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Determines the amount of output logged"))
        .get_matches();

    let debugging = matches.is_present("debugging");
    let verbosity = matches.occurrences_of("verbosity");

    let mut orchestrator = Orchestrator::new(debugging);
    orchestrator.run();
}
