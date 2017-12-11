#![feature(core_intrinsics)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate bitflags;
extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate glium;
extern crate nalgebra;
extern crate image;
extern crate uuid;

mod ecs;
mod engine;
mod game;

use std::io;
use std::path::PathBuf;
use clap::{Arg, App};
use log::LogLevelFilter;
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

    game::run(&resource_path, debugging);
}
