use std::f32;
use std::io;
use std::path::Path;
use std::time::Duration;
use nalgebra;
use nalgebra::{Point3, Vector3};
use engine::{Orchestrator, EventMonitor, DebugConsole, DebugShell, Renderer, EventInterface, Model,
    Description, Mesh, SpeechBubbleController, TooltipController, UiState, SpeechBubble, Camera, Tooltip, TooltipData,
    ShaderGroup, TextureGroup, BoundingVolume, Cursor, CursorController, FontGroup, DebugMover};

pub fn run(resource_path: &Path, debugging: bool) {
    // The following variables set up the state of the engine.
    let delta_time = Duration::from_millis(50);
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
        // Create the renderer (and dependencies).
        let event_interface = EventInterface::new();
        let renderer = Renderer::new(&event_interface.events_loop, &title, &dimensions, vsync, msaa, &clear_color)
            .unwrap();

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

        // Assemble the UI canvas.
        {
            let font_path = o.get_file("fonts", "SourceCodePro-Regular.ttf").unwrap();
            let font_scale = 24.0;
            let font_color = Vector3::new(0.0, 0.0, 0.0);
            let tvs = o.get_file("shaders", "text-vertex.glsl").unwrap();
            let tfs = o.get_file("shaders", "text-fragment.glsl").unwrap();
            let rvs = o.get_file("shaders", "rect-vertex.glsl").unwrap();
            let rfs = o.get_file("shaders", "rect-fragment.glsl").unwrap();
            let rdt = o.get_file("textures", "speech-bubble.png").unwrap();
            let font_group = FontGroup::new(&font_path, font_scale, font_color).unwrap();
            let text_shaders = ShaderGroup::new(&tvs, &tfs, None).unwrap();
            let rect_shaders = ShaderGroup::new(&rvs, &rfs, None).unwrap();
            let rect_textures = TextureGroup::new(Some(&rdt), None).unwrap();
            let speech_bubble = SpeechBubble::new(font_group, text_shaders.clone(), rect_shaders.clone(), rect_textures);
            let rdt = o.get_file("textures", "tooltip.png").unwrap();
            let rect_textures = TextureGroup::new(Some(&rdt), None).unwrap();
            let font_group = FontGroup::new(&font_path, font_scale, font_color).unwrap();
            let tooltip = Tooltip::new(font_group, text_shaders, rect_shaders, rect_textures);

            let canvas = o.world.create_entity();
            let d = Description::new("canvas");
            let u = UiState::new(&renderer.display, &dimensions, hi_dpi_factor, speech_bubble, tooltip).unwrap();

            o.world.add_component(&canvas, d).unwrap();
            o.world.add_component(&canvas, u).unwrap();
        }

        // Create the Cursor
        {
            let cursor = o.world.create_entity();
            let d = Description::new("cursor");
            let c = Cursor::new();

            o.world.add_component(&cursor, d).unwrap();
            o.world.add_component(&cursor, c).unwrap();
        }

        // Assemble the first test entity.
        {
            let position = Vector3::new(0.0, 0.0, -10.0);
            let axisangle = nalgebra::zero();
            let scale = Vector3::new(1.0, 1.0, 1.0);
            let vs = o.get_file("shaders", "test-vertex.glsl").unwrap();
            let fs = o.get_file("shaders", "test-fragment.glsl").unwrap();
            let shaders = ShaderGroup::new(&vs, &fs, None).unwrap();
            let textures = TextureGroup::empty();

            let test_entity_a = o.world.create_entity();
            let d = Description::new("test-entity-a");
            let tooltip = TooltipData::new("Hi, I'm a quad!");
            let model = Model::new(position, axisangle, scale);
            let mesh = Mesh::new_quad(&renderer.display).unwrap();
            let material = o.world.aux.factory.new_material(&renderer.display, &shaders, &textures).unwrap();
            let bounding_volume = BoundingVolume::from_mesh_aabb(&mesh).unwrap();

            o.world.aux.scene_graph.insert(test_entity_a.clone(), model.clone()).unwrap();

            o.world.add_component(&test_entity_a, d).unwrap();
            o.world.add_component(&test_entity_a, tooltip).unwrap();
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
            let vs = o.get_file("shaders", "test-vertex.glsl").unwrap();
            let fs = o.get_file("shaders", "test-fragment.glsl").unwrap();
            let shaders = ShaderGroup::new(&vs, &fs, None).unwrap();
            let textures = TextureGroup::empty();

            let test_entity_b = o.world.create_entity();
            let d = Description::new("test-entity-b");
            let tooltip = TooltipData::new("Hi, I'm a cube!");
            let model = Model::new(position, axisangle, scale);
            let mesh = Mesh::new_cube(&renderer.display).unwrap();
            let material = o.world.aux.factory.new_material(&renderer.display, &shaders, &textures).unwrap();
            let bounding_volume = BoundingVolume::from_mesh_aabb(&mesh).unwrap();

            o.world.aux.scene_graph.insert(test_entity_b.clone(), model.clone()).unwrap();

            o.world.add_component(&test_entity_b, d).unwrap();
            o.world.add_component(&test_entity_b, tooltip).unwrap();
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
            let vs = o.get_file("shaders", "test-vertex.glsl").unwrap();
            let fs = o.get_file("shaders", "test-fragment.glsl").unwrap();
            let shaders = ShaderGroup::new(&vs, &fs, None).unwrap();
            let textures = TextureGroup::empty();

            let test_entity_c = o.world.create_entity();
            let d = Description::new("test-entity-c");
            let tooltip = TooltipData::new("Hi, I'm a moving cube!");
            let model = Model::new(position, axisangle, scale);
            let mesh = Mesh::new_cube(&renderer.display).unwrap();
            let material = o.world.aux.factory.new_material(&renderer.display, &shaders, &textures).unwrap();
            let bounding_volume = BoundingVolume::from_mesh_aabb(&mesh).unwrap();

            o.world.aux.scene_graph.insert(test_entity_c.clone(), model.clone()).unwrap();

            o.world.add_component(&test_entity_c, d).unwrap();
            o.world.add_component(&test_entity_c, tooltip).unwrap();
            o.world.add_component(&test_entity_c, model).unwrap();
            o.world.add_component(&test_entity_c, mesh).unwrap();
            o.world.add_component(&test_entity_c, material).unwrap();
            o.world.add_component(&test_entity_c, bounding_volume).unwrap();
        }

        // Add systems to the world.
        if o.debug {
            o.world.add_system(EventMonitor::new().into()).unwrap();
            o.world.add_system(DebugMover::new("test-entity-c").into()).unwrap();
            o.world.add_system(DebugConsole::new(io::stdin()).into()).unwrap();
            o.world.add_system(DebugShell::new().into()).unwrap();
        }
        o.world.add_system(CursorController::new().into()).unwrap();
        o.world.add_system(TooltipController::new(&renderer.display).into()).unwrap();
        o.world.add_system(SpeechBubbleController::new(&renderer.display).into()).unwrap();
        o.world.add_system(renderer.into()).unwrap();
        o.world.add_system(event_interface.into()).unwrap();
    });
}
