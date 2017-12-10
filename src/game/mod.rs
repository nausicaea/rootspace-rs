use std::io;
use nalgebra::{Point3, Vector3};
use engine::{Float, Orchestrator, EngineEvent, EventMonitor, DebugConsole, DebugShell, Renderer,
    EventInterface, Projection, View, Description};

pub fn run(debugging: bool) {
    // The following variables set up the state of the engine.
    let title = String::from("Rootspace");
    let dimensions = [1024, 768];
    let vsync = true;
    let msaa = 4;
    let clear_color = [0.1, 0.15, 0.3, 1.0];
    let aspect = dimensions[0] as Float / dimensions[1] as Float;
    let fov_y = 3.1415926 / 4.0;
    let z_near = 0.01;
    let z_far = 1000.0;
    let eye = Point3::new(0.0, 0.0, 0.0);
    let target = Point3::new(0.0, 0.0, -1.0);
    let up = Vector3::y();

    // Create the engine instance and run it.
    let mut orchestrator: Orchestrator<EngineEvent> = Orchestrator::new(debugging);
    orchestrator.run(move |o| {
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

            o.world.add_component(&camera, Description::new("camera")).unwrap();
            o.world.add_component(&camera, Projection::new(aspect, fov_y, z_near, z_far)).unwrap();
            o.world.add_component(&camera, View::new(&eye, &target, &up)).unwrap();
        }
    });
}
