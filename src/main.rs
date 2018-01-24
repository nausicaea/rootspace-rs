//! Rootspace is a 3D game written entirely in Rust. Currently, it is only executable with Cargo,
//! but should run on all operating systems supported by the Rust compiler.

extern crate clap;
extern crate log;
extern crate fern;
extern crate nalgebra;

extern crate ecs;
extern crate engine;

mod game;

use std::io;
use std::path::PathBuf;
use clap::{Arg, App};
use log::LevelFilter;
use fern::Dispatch;

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
    let resource_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("rootspace");

    // Configure the logging system.
    let log_level_filter = if quiet {
        LevelFilter::Off
    } else {
        match verbosity {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
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
        .expect("Unable to configure the logger");

    game::run(&resource_path, debugging);
}
