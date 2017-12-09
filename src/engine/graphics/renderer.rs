use std::time::Duration;
use glium::{Surface, Display};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::{Api, GlRequest, GlProfile, WindowEvent, Event, EventsLoop, WindowBuilder, ContextBuilder};
use ecs::{LoopStageFlag, SystemTrait, Assembly};
use super::super::event::{EngineEventFlag, EngineEvent};

#[derive(Debug)]
pub enum RendererError {
    DisplayError(DisplayCreationError),
}

impl From<DisplayCreationError> for RendererError {
    fn from(value: DisplayCreationError) -> RendererError {
        RendererError::DisplayError(value)
    }
}

pub struct Renderer {
    pub display: Display,
    events_loop: EventsLoop,
    ready: bool,
    clear_color: (f32, f32, f32, f32),
}

impl Renderer {
    pub fn new(title: &str, dimensions: &[u32; 2], vsync: bool, msaa: u16, clear_color: &[f32; 4]) -> Result<Renderer, RendererError> {
        let events_loop = EventsLoop::new();
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
            events_loop: events_loop,
            ready: false,
            clear_color: (clear_color[0], clear_color[1], clear_color[2], clear_color[3]),
        })
    }
}

impl SystemTrait<EngineEvent> for Renderer {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        if self.ready {
            LoopStageFlag::ALL_STAGES
        } else {
            LoopStageFlag::HANDLE_EVENT
        }
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::READY
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        match *event {
            EngineEvent::Ready => Some(EngineEvent::RendererReady),
            _ => None,
        }
    }
    fn update(&mut self, _: &mut Assembly, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        let mut pd = Vec::new();
        let mut d = Vec::new();

        self.events_loop.poll_events(|ge| {
            match ge {
                Event::WindowEvent {event: we, ..} => match we {
                    WindowEvent::Closed => d.push(EngineEvent::Shutdown),
                    _ => (),
                },
                _ => (),
            }
        });

        if pd.len() > 0 || d.len() > 0 {
            Some((pd, d))
        } else {
            None
        }
    }
    fn render(&mut self, _: &mut Assembly, _: &Duration, _: &Duration) -> Option<EngineEvent> {
        let mut target = self.display.draw();
        target.clear_color_and_depth(self.clear_color, 1.0);

        //target.draw();

        target.finish().unwrap();
        None
    }
}
