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
extern crate glium;
extern crate nalgebra;

mod ecs;
mod engine;
mod game;

use std::io;
use clap::{Arg, App};
use log::LogLevelFilter;
use fern::Dispatch;
use engine::{Orchestrator, EngineEvent, EventMonitor, DebugConsole, DebugShell, Renderer, EventInterface, Projection};

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

    // The following variables set up the state of the engine.
    type Float = f32;
    let title = String::from("Rootspace");
    let dimensions = [1024, 768];
    let aspect = dimensions[0] as Float / dimensions[1] as Float;
    let fov_y = 3.1415926 / 4.0;
    let z_near = 0.01;
    let z_far = 1000.0;
    let vsync = true;
    let msaa = 4;
    let clear_color = [0.1, 0.15, 0.3, 1.0];

    // Create the engine instance and run it.
    let mut orchestrator: Orchestrator<EngineEvent> = Orchestrator::new(debugging);
    orchestrator.run(|o| {
        let event_interface = EventInterface::new();
        let renderer = Renderer::new(&event_interface.events_loop, &title, &dimensions, vsync, msaa, &clear_color)
            .unwrap();

        // Add systems to the world.
        if o.debug {
            o.world.add_system(EventMonitor::new());
            o.world.add_system(DebugConsole::new(io::stdin()));
            o.world.add_system(DebugShell::new());
        }
        o.world.add_system(renderer);
        o.world.add_system(event_interface);

        // Add entities to the world.
        {
            let camera = o.world.create_entity();
            let p: Projection<Float> = Projection::new(aspect, fov_y, z_near, z_far);
            o.world.add_component(&camera, p).unwrap();
        }
    });
}
