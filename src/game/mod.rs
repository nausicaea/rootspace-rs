use std::io;
use std::path::Path;
use std::time::Duration;
use nalgebra;
use nalgebra::{Point3, Vector3};
use engine::{Float, Orchestrator, EngineEvent, EventMonitor, DebugConsole, DebugShell, Renderer,
    EventInterface, Projection, View, Model, Description, Mesh, Material};

pub fn run(resource_path: &Path, debugging: bool) {
    // The following variables set up the state of the engine.
    let delta_time = Duration::from_millis(100);
    let max_frame_time = Duration::from_millis(250);
    let title = String::from("Rootspace");
    let dimensions = [1024, 768];
    let vsync = true;
    let msaa = 4;
    let clear_color = [0.1, 0.15, 0.3, 1.0];

    // Create the engine instance and run it.
    let mut orchestrator: Orchestrator<EngineEvent> = Orchestrator::new(resource_path, delta_time, max_frame_time, debugging);
    orchestrator.run(move |o| {
        let event_interface = EventInterface::new();
        let renderer = Renderer::new(&event_interface.events_loop, &title, &dimensions, vsync, msaa, &clear_color)
            .unwrap();

        // Assemble the camera entity.
        {
            let aspect = dimensions[0] as Float / dimensions[1] as Float;
            let fov_y = 3.1415926 / 4.0;
            let z_near = 0.01;
            let z_far = 1000.0;
            let eye = Point3::new(0.0, 0.0, 0.0);
            let target = Point3::new(0.0, 0.0, -1.0);
            let up = Vector3::y();

            let camera = o.world.create_entity();
            let d = Description::new("camera");
            let p = Projection::new(aspect, fov_y, z_near, z_far);
            let v = View::new(&eye, &target, &up);

            o.world.add_component(&camera, d).unwrap();
            o.world.add_component(&camera, p).unwrap();
            o.world.add_component(&camera, v).unwrap();
        }

        // Assemble the test entity.
        {
            let position = Vector3::new(0.0, 0.0, -10.0);
            let axisangle = nalgebra::zero();
            let vs = o.resource_path.join("shaders").join("test-vertex.glsl");
            let fs = o.resource_path.join("shaders").join("test-fragment.glsl");

            let test_entity = o.world.create_entity();
            let d = Description::new("test-entity");
            let model = Model::new(&position, &axisangle);
            let mesh = Mesh::quad(&renderer.display).unwrap();
            let material = Material::new(&renderer.display, &vs, &fs, None, None, None).unwrap();

            o.world.add_component(&test_entity, d).unwrap();
            o.world.add_component(&test_entity, model).unwrap();
            o.world.add_component(&test_entity, mesh).unwrap();
            o.world.add_component(&test_entity, material).unwrap();
        }

        // Add systems to the world.
        if o.debug {
            o.world.add_system(EventMonitor::new());
            o.world.add_system(DebugConsole::new(io::stdin()));
            o.world.add_system(DebugShell::new());
        }
        o.world.add_system(renderer);
        o.world.add_system(event_interface);
    });
}
