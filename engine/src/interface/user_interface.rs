use std::time::Duration;
use glium::Display;
use ecs::{LoopStageFlag, SystemTrait, Assembly, EcsError};
use event::{EngineEventFlag, EngineEvent};
use debugging::description::Description;
use geometry::model::Model;
use interface::ui_state::UiState;

#[derive(Debug, Fail)]
pub enum UiError {
    #[fail(display = "{}", _0)]
    AssemblyError(#[cause] EcsError),
}

impl From<EcsError> for UiError {
    fn from(value: EcsError) -> Self {
        UiError::AssemblyError(value)
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
        let entity_position = entities.rsf2::<_, Description, Model>(|&(d, _)| d.name == target)
            .map(|(_, m)| m.translation.vector)?;

        Ok(())
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
            self.create_speech_bubble(entities, t, c, l).unwrap();
        }
        None
    }
    fn update(&mut self, entities: &mut Assembly, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        self.update_lifetimes(entities).unwrap();
        None
    }
}
