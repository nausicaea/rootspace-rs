#![feature(core_intrinsics)]

extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;

mod ecs;
mod engine;
mod game;

use std::io;
use clap::{Arg, App};
use log::LogLevelFilter;
use fern::Dispatch;
use engine::orchestrator::Orchestrator;
use engine::debugging::event_monitor::EventMonitor;
use game::Event;

fn main() {
    // Define the command line interface.
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("debugging")
             .short("d")
             .long("debug")
             .help("Enables debugging features"))
        .arg(Arg::with_name("verbosity")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Determines the amount of output logged"))
        .arg(Arg::with_name("quiet")
             .short("q")
             .long("quiet")
             .conflicts_with("verbosity")
             .help("Disables all output"))
        .get_matches();

    // Obtain the command line arguments.
    let debugging = matches.is_present("debugging");
    let verbosity = matches.occurrences_of("verbosity");
    let quiet = matches.is_present("quiet");

    // Configure the logging system.
    let log_level_filter = if quiet {
        LogLevelFilter::Off
    } else {
        match verbosity {
            0 => LogLevelFilter::Error,
            1 => LogLevelFilter::Warn,
            2 => LogLevelFilter::Info,
            3 => LogLevelFilter::Debug,
            _ => LogLevelFilter::Trace,
        }
    };
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} @{}: {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level_filter)
        .chain(io::stdout())
        .apply()
        .unwrap();

    // Create the engine instance and run it.
    let mut orchestrator: Orchestrator<Event> = Orchestrator::new(debugging);
    orchestrator.run(|o| {
        if o.debug {
            o.world.add_system(EventMonitor::new());
        }
    });
}
