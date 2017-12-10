use std::time::Duration;
use glium::{Surface, Display};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::{Api, GlRequest, GlProfile, EventsLoop, WindowBuilder, ContextBuilder};
use ecs::{LoopStageFlag, SystemTrait, Assembly};
use super::super::event::{EngineEventFlag, EngineEvent};
use super::super::geometry::projection::Projection;
use super::super::geometry::view::View;

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
        let display = Display::new(window, context, &events_loop)?;

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
        EngineEventFlag::READY
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        match *event {
            EngineEvent::Ready => {
                self.ready = true;
                Some(EngineEvent::RendererReady)
            },
            _ => None,
        }
    }
    fn render(&mut self, entities: &mut Assembly, _: &Duration, _: &Duration) -> Option<EngineEvent> {
        let mut target = self.display.draw();
        target.clear_color_and_depth(self.clear_color, 1.0);

        // Render all entities.
        entities.rs2::<Projection, View>()
            .map(|(p, v)| {
                let pv = p.as_matrix() * v.to_homogeneous();
                // entities.r4::<Description, ..>().iter()
                // target.draw();
            })
            .unwrap();

        target.finish().unwrap();
        None
    }
}
