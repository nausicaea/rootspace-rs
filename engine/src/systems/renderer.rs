use std::borrow::Borrow;
use std::f32;
use std::time::Duration;
use glium;
use glium::{Display, DrawParameters, Frame, Surface};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::{Api, ContextBuilder, EventsLoop, GlProfile, GlRequest, WindowBuilder};
use nalgebra::Vector3;
use ecs::{Assembly, DispatchEvents, LoopStageFlag, SystemTrait};
use event::{EngineEvent, EngineEventFlag};
use singletons::Singletons;
use components::camera::Camera;
use components::material::Material;
use components::mesh::Mesh;
use components::model::Model;
use components::render_mode::RenderMode;
use common::uniforms::Uniforms;
use components::ui_state::UiState;
use common::ui_uniforms::UiUniforms;

/// The `Renderer`'s task is to manage the graphical display and render entities as well as the
/// user interface.
pub struct Renderer {
    /// Provides access to the `Display`.
    pub display: Display,
    ready: bool,
    clear_color: (f32, f32, f32, f32),
    draw_params: DrawParameters<'static>,
}

impl Renderer {
    /// Creates a new instance of `Renderer`.
    pub fn new(
        events_loop: &EventsLoop,
        title: &str,
        dimensions: &[u32; 2],
        vsync: bool,
        msaa: u16,
        clear_color: &[f32; 4],
    ) -> Result<Self, RendererError> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dimensions[0], dimensions[1]);
        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_vsync(vsync)
            .with_multisampling(msaa);
        let display = Display::new(window, context, events_loop)?;
        let draw_params = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            ..Default::default()
        };

        Ok(Renderer {
            display: display,
            ready: false,
            clear_color: (
                clear_color[0],
                clear_color[1],
                clear_color[2],
                clear_color[3],
            ),
            draw_params: draw_params,
        })
    }
    fn render_world_entity(&self, target: &mut Frame, camera: &Camera, model: &Model, mesh: &Mesh, material: &Material, params: &DrawParameters) {
        let uniforms = Uniforms {
            pvm_matrix: camera.matrix * model.matrix(),
        };
        target
            .draw(
                &mesh.vertices,
                &mesh.indices,
                &material.shader,
                &uniforms,
                params,
                )
            .expect("Unable to execute the draw call");
    }
    fn render_ui_entity(&self, target: &mut Frame, ui_state: &UiState, model: &Model, mesh: &Mesh, material: &Material, params: &DrawParameters) {
        let uniforms = UiUniforms {
            pvm_matrix: *model.matrix(),
            font_cache: &ui_state.font_cache.gpu,
            font_color: Vector3::new(0.0, 0.0, 0.0),
            diff_tex: material.diff_tex.as_ref().map(|dt| dt.borrow()),
            norm_tex: material.norm_tex.as_ref().map(|nt| nt.borrow()),
        };

        target
            .draw(
                &mesh.vertices,
                &mesh.indices,
                &material.shader,
                &uniforms,
                params,
                )
            .expect("Unable to execute the draw call");
    }
    fn render_user_interface(
        &self,
        entities: &Assembly,
        aux: &mut Singletons,
        target: &mut Frame,
        params: &DrawParameters,
    ) {
        entities
            .rs1::<UiState>()
            .map(|(_, u)| {
                // Update the UI scene graph.
                aux.ui_hierarchy
                    .update(&|id, parent_model| {
                        let current_element = u.elements.get(id)?;
                        Some(parent_model * &current_element.model)
                    })
                    .expect("Unable to update the UI scene graph.");

                // Sort the UI scene graph nodes.
                let mut nodes = aux.ui_hierarchy.iter().collect::<Vec<_>>();
                nodes.sort_unstable_by_key(|n| (n.data.translation().z / f32::EPSILON).round() as i32);

                // Render all UI elements.
                for node in nodes {
                    if let Some(e) = u.elements.get(&node.key) {
                        for p in &e.primitives {
                            let uniforms = UiUniforms {
                                pvm_matrix: node.data.matrix() * p.model.matrix(),
                                font_cache: &u.font_cache.gpu,
                                font_color: p.text_color,
                                diff_tex: p.material.diff_tex.as_ref().map(|dt| dt.borrow()),
                                norm_tex: p.material.norm_tex.as_ref().map(|nt| nt.borrow()),
                            };

                            target
                                .draw(
                                    &p.mesh.vertices,
                                    &p.mesh.indices,
                                    &p.material.shader,
                                    &uniforms,
                                    params,
                                )
                                .expect("Unable to execute the draw call");
                        }
                    }
                }
            })
            .expect("Failed to render the user interface");
    }
}

impl SystemTrait<EngineEvent, Singletons> for Renderer {
    /// The `Renderer` depends on the presence of exactly one `Camera` component and one `UiState`
    /// component.
    fn verify_requirements(&self, entities: &Assembly) -> bool {
        entities.count1::<Camera>() == 1 && entities.count1::<UiState>() == 1
    }
    /// If the `Renderer` has completed initialization, it subscribes to the `handle_event` and
    /// render calls. Otherwise, it will only listen for events.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        if self.ready {
            LoopStageFlag::HANDLE_EVENT | LoopStageFlag::RENDER
        } else {
            LoopStageFlag::HANDLE_EVENT
        }
    }
    /// `Renderer` subscribes to the `Ready` and `ResizeWindow` events.
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::READY | EngineEventFlag::RESIZE_WINDOW
    }
    /// Once the `Ready` event has been received, the `Renderer` completes its initialization and
    /// emits a `RendererReady` event. Upon receiving a `ResizeWindow` event, the `Camera`
    /// component is updated.
    fn handle_event(
        &mut self,
        entities: &mut Assembly,
        _: &mut Singletons,
        event: &EngineEvent,
    ) -> DispatchEvents<EngineEvent> {
        match *event {
            EngineEvent::Ready => {
                self.ready = true;
                (None, Some(vec![EngineEvent::RendererReady]))
            }
            EngineEvent::ResizeWindow(w, h) => {
                entities
                    .ws1::<Camera>()
                    .map(|(_, c)| c.set_dimensions([w, h]))
                    .expect("Unable to update the projection matrices.");
                (None, None)
            }
            _ => (None, None),
        }
    }
    /// First updates the `Hierarchy` to receive accurate and current hierarchical model data.
    /// Subsequently renders the `Entity`s to the frame, followed by the user interface state as
    /// defined in `UiState`.
    fn render(&mut self, entities: &Assembly, aux: &mut Singletons, _: &Duration, _: &Duration) {
        // Create the current frame.
        let mut target = self.display.draw();
        target.clear_color_and_depth(self.clear_color, 1.0);

        // Update the scene graph.
        aux.scene_graph
            .update(&|entity, parent_model| {
                let current_model = entities
                    .borrow_component(entity)
                    .ok()?;
                Some(parent_model * current_model)
            })
            .expect("Unable to update the scene graph");

        // Get a reference to the camera.
        let (_, camera) = entities.rs1::<Camera>().expect("Could not access the camera component.");

        // Get a reference to the UI state.
        let (_, ui_state) = entities.rs1::<UiState>().expect("Could not access the UI state component.");

        // Sort the nodes according to their z-value.
        let mut nodes = aux.scene_graph.iter().collect::<Vec<_>>();
        nodes.sort_unstable_by_key(|n| (n.data.translation().z / f32::EPSILON).round() as i32);

        // Render all entities
        for node in nodes {
            if entities.has_component::<Mesh>(&node.key) && entities.has_component::<Material>(&node.key) && entities.has_component::<RenderMode>(&node.key) {
                let mesh = entities.borrow_component::<Mesh>(&node.key).unwrap();
                let material = entities.borrow_component::<Material>(&node.key).unwrap();
                let render_mode = entities.borrow_component::<RenderMode>(&node.key).unwrap();

                match render_mode {
                    &RenderMode::World => self.render_world_entity(&mut target, camera, &node.data, mesh, material, &self.draw_params),
                    &RenderMode::Ui => self.render_ui_entity(&mut target, ui_state, &node.data, mesh, material, &self.draw_params),
                }
            }
        }

        // Render the user interface.
        // self.render_user_interface(entities, aux, &mut target, &self.draw_params);

        target
            .finish()
            .expect("Unable to finalize the current frame");
    }
}

#[derive(Debug)]
pub enum RendererError {
    DisplayError(DisplayCreationError),
}

impl From<DisplayCreationError> for RendererError {
    fn from(value: DisplayCreationError) -> RendererError {
        RendererError::DisplayError(value)
    }
}
