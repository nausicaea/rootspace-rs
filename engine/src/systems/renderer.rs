use std::borrow::Borrow;
use std::time::Duration;
use glium;
use glium::{Frame, Surface, Display, DrawParameters};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::{Api, GlRequest, GlProfile, EventsLoop, WindowBuilder, ContextBuilder};
use ecs::{LoopStageFlag, SystemTrait, Assembly};
use event::{EngineEventFlag, EngineEvent};
use scene_graph::SceneGraph;
use components::camera::Camera;
use components::model::Model;
use components::mesh::Mesh;
use components::material::Material;
use common::uniforms::Uniforms;
use components::ui_state::UiState;
use common::ui_uniforms::UiUniforms;

#[derive(Debug)]
pub enum RendererError {
    DisplayError(DisplayCreationError),
}

impl From<DisplayCreationError> for RendererError {
    fn from(value: DisplayCreationError) -> RendererError {
        RendererError::DisplayError(value)
    }
}

/// The `Renderer`'s task is to manage the graphical display and render entities as well as the
/// user interface.
pub struct Renderer {
    /// Provides access to the `Display`.
    pub display: Display,
    /// Provides access to the `SceneGraph` for `Entities` with a `Model` component (i.e. a
    /// location in 3-space).
    pub scene_graph: SceneGraph<Model>,
    ready: bool,
    clear_color: (f32, f32, f32, f32),
    draw_params: DrawParameters<'static>,
}

impl Renderer {
    /// Creates a new instance of `Renderer`.
    pub fn new(events_loop: &EventsLoop, scene_graph: SceneGraph<Model>, title: &str, dimensions: &[u32; 2], vsync: bool, msaa: u16, clear_color: &[f32; 4]) -> Result<Self, RendererError> {
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
                .. Default::default()
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
            .. Default::default()
        };

        Ok(Renderer {
            display: display,
            scene_graph: scene_graph,
            ready: false,
            clear_color: (clear_color[0], clear_color[1], clear_color[2], clear_color[3]),
            draw_params: draw_params,
        })
    }
    fn render_entities(&self, entities: &Assembly, target: &mut Frame, params: &DrawParameters) {
        entities.rs1::<Camera>()
            .map(|c| {
                for node in self.scene_graph.iter() {
                    if let Ok(mesh) = entities.borrow_component::<Mesh>(&node.entity) {
                        if let Ok(material) = entities.borrow_component::<Material>(&node.entity) {
                            let uniforms = Uniforms {
                                pvm_matrix: c.matrix * node.component.matrix(),
                            };
                            target.draw(&mesh.vertices, &mesh.indices, &material.shader, &uniforms, params)
                                .expect("Unable to execute the draw call");
                        }
                    }
                }
            })
            .expect("Failed to render entities");
    }
    fn render_user_interface(&self, entities: &Assembly, target: &mut Frame, params: &DrawParameters) {
        entities.rs1::<UiState>()
            .map(|u| {
                for e in u.elements.values() {
                    for p in &e.primitives {
                        let uniforms = UiUniforms {
                            pvm_matrix: e.model.matrix() * p.model.matrix(),
                            font_cache: &u.font_cache_gpu,
                            diff_tex: p.material.diff_tex.as_ref().map(|dt| dt.borrow()),
                            norm_tex: p.material.norm_tex.as_ref().map(|nt| nt.borrow()),
                        };

                        target.draw(&p.mesh.vertices, &p.mesh.indices, &p.material.shader, &uniforms, params)
                            .expect("Unable to execute the draw call");
                    }
                }
            })
            .expect("Failed to render the user interface");
    }
}

impl<F> SystemTrait<EngineEvent, F> for Renderer {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        if self.ready {
            LoopStageFlag::HANDLE_EVENT | LoopStageFlag::RENDER
        } else {
            LoopStageFlag::HANDLE_EVENT
        }
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::READY | EngineEventFlag::RESIZE_WINDOW
    }
    fn handle_event(&mut self, entities: &mut Assembly, _: &mut F, event: &EngineEvent) -> Option<EngineEvent> {
        match *event {
            EngineEvent::Ready => {
                self.ready = true;
                Some(EngineEvent::RendererReady)
            },
            EngineEvent::ResizeWindow(w, h) => {
                entities.ws1::<Camera>()
                    .map(|c| c.set_dimensions([w, h]))
                    .expect("Unable to update the projection matrices.");
                None
            },
            _ => None,
        }
    }
    fn render(&mut self, entities: &Assembly, _: &Duration, _: &Duration) -> Option<EngineEvent> {
        // Update the scene graph.
        self.scene_graph.update(entities, &|pc, cc| pc * cc)
            .expect("Unable to update the scene graph");

        // Create the current frame.
        let mut target = self.display.draw();
        target.clear_color_and_depth(self.clear_color, 1.0);

        // Render all entities.
        self.render_entities(entities, &mut target, &self.draw_params);

        // Render the user interface.
        self.render_user_interface(entities, &mut target, &self.draw_params);

        target.finish()
            .expect("Unable to finalize the current frame");
        None
    }
}
