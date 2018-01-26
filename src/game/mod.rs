use std::f32;
use std::io;
use std::path::Path;
use std::time::Duration;
use nalgebra;
use nalgebra::{Point3, Vector3};
use engine::{Orchestrator, EventMonitor, DebugConsole, DebugShell,
    Renderer, EventInterface, Model, Description, Mesh, UserInterface,
    UiState, Common, SpeechBubble, SceneGraph, SceneNode, Camera,
    BoundingVolume};

pub fn run(resource_path: &Path, debugging: bool) {
    // The following variables set up the state of the engine.
    let delta_time = Duration::from_millis(100);
    let max_frame_time = Duration::from_millis(250);
    let title = String::from("Rootspace");
    let dimensions = [1024, 768];
    let hi_dpi_factor = 1.0;
    let vsync = true;
    let msaa = 4;
    let clear_color = [0.1, 0.15, 0.3, 1.0];

    // Create the engine instance and run it.
    let mut orchestrator = Orchestrator::new(resource_path, delta_time, max_frame_time, debugging);
    orchestrator.run(move |o| {
        // Create the root entity.
        let scene = o.world.create_entity();
        let scene_description = Description::new("scene");
        let scene_model = Model::identity();

        // Create the renderer (and dependencies).
        let scene_graph = SceneGraph::new(SceneNode::new(scene.clone(), scene_model.clone()));
        let event_interface = EventInterface::new();
        let mut renderer = Renderer::new(&event_interface.events_loop, scene_graph, &title, &dimensions, vsync, msaa, &clear_color)
            .unwrap();

        // Register the scene entity.
        o.world.add_component(&scene, scene_description).unwrap();
        o.world.add_component(&scene, scene_model).unwrap();

        // Assemble the camera entity.
        {
            let fov_y = f32::consts::PI / 4.0;
            let z_near = 0.01;
            let z_far = 1000.0;
            let eye = Point3::new(0.0, 0.0, 1.0);
            let target = Point3::new(0.0, 0.0, -1.0);
            let up = Vector3::y();

            let camera = o.world.create_entity();
            let d = Description::new("camera");
            let c = Camera::new(dimensions, fov_y, z_near, z_far, &eye, &target, &up);

            o.world.add_component(&camera, d).unwrap();
            o.world.add_component(&camera, c).unwrap();
        }

        // Assembly the UI canvas.
        {
            let font_path = o.resource_path.join("fonts").join("SourceCodePro-Regular.ttf");
            let font_scale = 24.0;
            let common = Common::new(&font_path, font_scale).unwrap();
            let tvs = o.resource_path.join("shaders").join("text-vertex.glsl");
            let tfs = o.resource_path.join("shaders").join("text-fragment.glsl");
            let rvs = o.resource_path.join("shaders").join("rect-vertex.glsl");
            let rfs = o.resource_path.join("shaders").join("rect-fragment.glsl");
            let rdt = o.resource_path.join("textures").join("speech-bubble.png");
            let speech_bubble = SpeechBubble::new(&tvs, &tfs, &rvs, &rfs, &rdt);

            let canvas = o.world.create_entity();
            let d = Description::new("canvas");
            let u = UiState::new(&renderer.display, &dimensions, hi_dpi_factor, common, speech_bubble).unwrap();

            o.world.add_component(&canvas, d).unwrap();
            o.world.add_component(&canvas, u).unwrap();
        }

        // Assemble the first test entity.
        {
            let position = Vector3::new(0.0, 0.0, -10.0);
            let axisangle = nalgebra::zero();
            let scale = Vector3::new(1.0, 1.0, 1.0);
            let vs = o.resource_path.join("shaders").join("test-vertex.glsl");
            let fs = o.resource_path.join("shaders").join("test-fragment.glsl");

            let test_entity_a = o.world.create_entity();
            let d = Description::new("test-entity-a");
            let model = Model::new(position, axisangle, scale);
            let mesh = Mesh::new_quad(&renderer.display, 0.0).unwrap();
            let material = o.world.aux.factory.new_material(&renderer.display, &vs, &fs, None, None, None).unwrap();
            let bounding_volume = BoundingVolume::from_mesh_aabb(&mesh).unwrap();

            renderer.scene_graph.insert(SceneNode::new(test_entity_a.clone(), model.clone())).unwrap();

            o.world.add_component(&test_entity_a, d).unwrap();
            o.world.add_component(&test_entity_a, model).unwrap();
            o.world.add_component(&test_entity_a, mesh).unwrap();
            o.world.add_component(&test_entity_a, material).unwrap();
            o.world.add_component(&test_entity_a, bounding_volume).unwrap();
        }

        // Assemble the second test entity.
        {
            let position = Vector3::new(-2.0, 1.0, -7.0);
            let axisangle = Vector3::new(0.0, f32::consts::PI / 4.0, 0.0);
            let scale = Vector3::new(1.0, 1.0, 1.0);
            let vs = o.resource_path.join("shaders").join("test-vertex.glsl");
            let fs = o.resource_path.join("shaders").join("test-fragment.glsl");

            let test_entity_b = o.world.create_entity();
            let d = Description::new("test-entity-b");
            let model = Model::new(position, axisangle, scale);
            let mesh = Mesh::new_cube(&renderer.display).unwrap();
            let material = o.world.aux.factory.new_material(&renderer.display, &vs, &fs, None, None, None).unwrap();
            let bounding_volume = BoundingVolume::from_mesh_aabb(&mesh).unwrap();

            renderer.scene_graph.insert(SceneNode::new(test_entity_b.clone(), model.clone())).unwrap();

            o.world.add_component(&test_entity_b, d).unwrap();
            o.world.add_component(&test_entity_b, model).unwrap();
            o.world.add_component(&test_entity_b, mesh).unwrap();
            o.world.add_component(&test_entity_b, material).unwrap();
            o.world.add_component(&test_entity_b, bounding_volume).unwrap();
        }

        // Assemble the third test entity.
        {
            let position = Vector3::new(1.0, -1.5, -5.0);
            let axisangle = Vector3::new(1.0, 1.0, 1.0) * f32::consts::PI / 4.0;
            let scale = Vector3::new(1.0, 1.0, 1.0);
            let vs = o.resource_path.join("shaders").join("test-vertex.glsl");
            let fs = o.resource_path.join("shaders").join("test-fragment.glsl");

            let test_entity_c = o.world.create_entity();
            let d = Description::new("test-entity-c");
            let model = Model::new(position, axisangle, scale);
            let mesh = Mesh::new_cube(&renderer.display).unwrap();
            let material = o.world.aux.factory.new_material(&renderer.display, &vs, &fs, None, None, None).unwrap();
            let bounding_volume = BoundingVolume::from_mesh_aabb(&mesh).unwrap();

            renderer.scene_graph.insert(SceneNode::new(test_entity_c.clone(), model.clone())).unwrap();

            o.world.add_component(&test_entity_c, d).unwrap();
            o.world.add_component(&test_entity_c, model).unwrap();
            o.world.add_component(&test_entity_c, mesh).unwrap();
            o.world.add_component(&test_entity_c, material).unwrap();
            o.world.add_component(&test_entity_c, bounding_volume).unwrap();
        }

        // Add systems to the world.
        if o.debug {
            o.world.add_system(EventMonitor::new()).unwrap();
            o.world.add_system(DebugConsole::new(io::stdin())).unwrap();
            o.world.add_system(DebugShell::new()).unwrap();
        }
        o.world.add_system(UserInterface::new(&renderer.display)).unwrap();
        o.world.add_system(renderer).unwrap();
        o.world.add_system(event_interface).unwrap();
    });
}
