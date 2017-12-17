use std::borrow::Borrow;
use std::time::Duration;
use glium;
use glium::{Surface, Display, DrawParameters};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::{Api, GlRequest, GlProfile, EventsLoop, WindowBuilder, ContextBuilder};
use ecs::{LoopStageFlag, SystemTrait, Assembly};
use event::{EngineEventFlag, EngineEvent};
use geometry::projection::Projection;
use geometry::view::View;
use geometry::model::Model;
use graphics::mesh::Mesh;
use graphics::material::Material;
use graphics::uniforms::Uniforms;
use interface::ui_state::UiState;
use interface::ui_uniforms::UiUniforms;

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
    ready: bool,
    clear_color: (f32, f32, f32, f32),
}

impl Renderer {
    /// Creates a new instance of `Renderer`.
    pub fn new(events_loop: &EventsLoop, title: &str, dimensions: &[u32; 2], vsync: bool, msaa: u16, clear_color: &[f32; 4]) -> Result<Self, RendererError> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dimensions[0], dimensions[1]);
        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_vsync(vsync)
            .with_multisampling(msaa);
        let display = Display::new(window, context, events_loop)?;

        Ok(Renderer {
            display: display,
            ready: false,
            clear_color: (clear_color[0], clear_color[1], clear_color[2], clear_color[3]),
        })
    }
}

impl SystemTrait<EngineEvent> for Renderer {
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
    fn handle_event(&mut self, entities: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        match *event {
            EngineEvent::Ready => {
                self.ready = true;
                Some(EngineEvent::RendererReady)
            },
            EngineEvent::ResizeWindow(w, h) => {
                entities.ws1::<Projection>()
                    .map(|p| p.set_aspect(w as f32 / h as f32))
                    .unwrap();
                None
            },
            _ => None,
        }
    }
    fn render(&mut self, entities: &Assembly, _: &Duration, _: &Duration) -> Option<EngineEvent> {
        let mut target = self.display.draw();
        target.clear_color_and_depth(self.clear_color, 1.0);

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

        // Render all entities.
        entities.rs2::<Projection, View>()
            .map(|(p, v)| {
                let pv = p.as_matrix() * v.to_homogeneous();
                for (mo, me, ma) in entities.r3::<Model, Mesh, Material>() {
                    let uniforms = Uniforms {
                        pvm_matrix: pv * mo.to_homogeneous(),
                    };

                    target.draw(&me.vertices, &me.indices, &ma.shader, &uniforms, &draw_params).unwrap();
                }
            })
            .unwrap();

        // Redner the user interface.
        entities.rs1::<UiState>()
            .map(|u| {
                for e in u.elements.values() {
                    for p in &e.primitives {
                        let uniforms = UiUniforms {
                            pvm_matrix: e.model.to_homogeneous() * p.model.to_homogeneous(),
                            font_cache: &u.font_cache_gpu,
                            diff_tex: p.material.diff_tex.as_ref().map(|dt| dt.borrow()),
                            norm_tex: p.material.norm_tex.as_ref().map(|nt| nt.borrow()),
                        };

                        target.draw(&p.mesh.vertices, &p.mesh.indices, &p.material.shader, &uniforms, &draw_params).unwrap();
                    }
                }
            })
            .unwrap();

        target.finish().unwrap();
        None
    }
}
