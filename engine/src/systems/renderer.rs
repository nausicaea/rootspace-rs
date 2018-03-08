use std::borrow::Borrow;
use std::time::Duration;
use glium;
use glium::{Display, DrawParameters, Frame, Surface};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::{Api, ContextBuilder, EventsLoop, GlProfile, GlRequest, WindowBuilder};
use ecs::{Assembly, DispatchEvents, LoopStageFlag, SystemTrait};
use event::{EngineEvent, EngineEventFlag};
use singletons::Singletons;
use components::camera::Camera;
use components::mesh::Mesh;
use components::material::Material;
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
    fn render_entities(
        &self,
        entities: &Assembly,
        aux: &mut Singletons,
        target: &mut Frame,
        params: &DrawParameters,
    ) {
        // Update the scene graph.
        aux.scene_graph
            .update(&|entity, parent_component| {
                let current_component = entities
                    .borrow_component(entity)
                    .expect("The scene graph is irreparably out of sync with the assembly");
                parent_component * current_component
            })
            .expect("Unable to update the scene graph");

        entities
            .rs1::<Camera>()
            .map(|(_, c)| {
                for node in aux.scene_graph.iter() {
                    if let Ok(mesh) = entities.borrow_component::<Mesh>(&node.key) {
                        if let Ok(material) = entities.borrow_component::<Material>(&node.key) {
                            let uniforms = Uniforms {
                                pvm_matrix: c.matrix * node.data.matrix(),
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
                    }
                }
            })
            .expect("Failed to render entities");
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
                aux.ui_hierarchy
                    .update(&|id, parent_component| {
                        let current_component = u.elements.get(id).expect("The requested entity was not found.");
                        parent_component * &current_component.model
                    })
                    .expect("Unable to update the UI scene graph.");

                for e in u.elements.values() {
                    for p in &e.primitives {
                        let uniforms = UiUniforms {
                            pvm_matrix: e.model.matrix() * p.model.matrix(),
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
            })
            .expect("Failed to render the user interface");
    }
}

impl SystemTrait<EngineEvent, Singletons> for Renderer {
    /// The `Renderer` depends on the presence of exactly one `Camera` component.
    fn verify_requirements(&self, entities: &Assembly) -> bool {
        entities.count1::<Camera>() == 1
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

        // Render all entities.
        self.render_entities(entities, aux, &mut target, &self.draw_params);

        // Render the user interface.
        self.render_user_interface(entities, aux, &mut target, &self.draw_params);

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
