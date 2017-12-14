use std::time::Duration;
use glium::Display;
use alga::linear::Transformation;
use nalgebra::Point3;
use rusttype::gpu_cache::CacheWriteErr;
use rusttype::gpu_cache::CacheWriteErr::*;
use ecs::{LoopStageFlag, SystemTrait, Assembly, EcsError};
use utilities::layout_paragraph_cached;
use event::{EngineEventFlag, EngineEvent};
use debugging::description::Description;
use geometry::projection::Projection;
use geometry::view::View;
use geometry::model::Model;
use interface::ui_state::UiState;
use interface::ui_primitive::{UiPrimitive, UiPrimitiveError};

#[derive(Debug, Fail)]
pub enum UiError {
    #[fail(display = "{}", _0)]
    AssemblyError(#[cause] EcsError),
    #[fail(display = "{}", _0)]
    CacheError(String),
    #[fail(display = "{}", _0)]
    PrimitiveError(#[cause] UiPrimitiveError),
}

impl From<EcsError> for UiError {
    fn from(value: EcsError) -> Self {
        UiError::AssemblyError(value)
    }
}

impl From<CacheWriteErr> for UiError {
    fn from(value: CacheWriteErr) -> Self {
        use self::UiError::*;

        match value {
            GlyphTooLarge => CacheError("At least one of the queued glyphs is too big to fit into
                                        the cache, even if all other glyphs are removed".into()),
            NoRoomForWholeQueue => CacheError("Not all of the requested glyphs can fit into the
                                              cache, even if the cache is completely cleared before
                                              the attempt".into()),
        }
    }
}

impl From<UiPrimitiveError> for UiError {
    fn from(value: UiPrimitiveError) -> Self {
        UiError::PrimitiveError(value)
    }
}

/// The `UserInterface` is responsible for managing the state associated with the user interface.
/// It also processes events that relate to the UI.
pub struct UserInterface {
    display: Display,
}

impl UserInterface {
    /// Creates a new `UserInterface` system.
    pub fn new(display: &Display) -> Self {
        UserInterface {
            display: display.clone(),
        }
    }
    fn create_speech_bubble(&self, entities: &mut Assembly, target: &str, content: &str, lifetime: u64) -> Result<(), UiError> {
        let world_position = entities.rsf2::<_, Description, Model>(|&(d, _)| d.name == target)
            .map(|(_, m)| Point3::from_coordinates(m.translation.vector))?;

        let ndc_position = entities.rs2::<Projection, View>()
            .map(|(p, v)| p.project_point(&v.transform_point(&world_position)))?;

        entities.ws1::<UiState>()
            .map_err(|e| UiError::AssemblyError(e))
            .and_then(|u| {
                let (glyphs, text_dims_px) = layout_paragraph_cached(&mut u.font_cache_cpu,
                                                                     &u.font_cache_gpu,
                                                                     &u.common.font,
                                                                     u.common.font_scale,
                                                                     u.speech_bubble.width,
                                                                     content)?;

                Ok(())
            })
    }
    /// Checks the lifetimes of the registered `UiElement`s and removes those with expired
    /// lifetimes.
    fn update_lifetimes(&self, entities: &mut Assembly) -> Result<(), EcsError> {
        entities.ws1::<UiState>()
            .map(|u| {
                if !u.lifetimes.is_empty() {
                    let to_delete = u.lifetimes.iter()
                        .filter(|&(_, l)| l.0.elapsed() >= l.1)
                        .map(|(i, _)| i)
                        .cloned()
                        .collect::<Vec<_>>();

                    to_delete.iter()
                        .for_each(|i| {
                            u.elements.remove(i);
                            u.lifetimes.remove(i);
                        });
                }
            })
    }
}

impl SystemTrait<EngineEvent> for UserInterface {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT | LoopStageFlag::UPDATE
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::SPEECH_BUBBLE
    }
    fn handle_event(&mut self, entities: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        if let EngineEvent::SpeechBubble(ref t, ref c, l) = *event {
            self.create_speech_bubble(entities, t, c, l).unwrap_or_else(|e| warn!("{}", e));
        }
        None
    }
    fn update(&mut self, entities: &mut Assembly, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        self.update_lifetimes(entities).unwrap();
        None
    }
}
