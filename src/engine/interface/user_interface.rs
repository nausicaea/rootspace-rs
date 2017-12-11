use std::time::Duration;
use glium::Display;
use ecs::{LoopStageFlag, SystemTrait, Assembly, EcsError};
use super::super::event::{EngineEventFlag, EngineEvent};
use super::ui_state::UiState;

#[derive(Debug, Fail)]
pub enum UiError {
    #[fail(display = "{}", _0)]
    AssemblyError(#[cause] EcsError),
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
    /// Checks the lifetimes of the registered `UiElement`s and removes those with expired
    /// lifetimes.
    fn update_lifetimes(&self, entities: &mut Assembly) -> Result<(), EcsError> {
        entities.ws1::<UiState>()
            .map(|u| {
                if u.lifetimes.len() > 0 {
                    let to_delete = u.lifetimes.iter()
                        .filter(|&(i, l)| l.0.elapsed() >= l.1)
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
        EngineEventFlag::empty()
    }
    fn handle_event(&mut self, entities: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        None
    }
    fn update(&mut self, entities: &mut Assembly, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        self.update_lifetimes(entities).unwrap();
        None
    }
}
