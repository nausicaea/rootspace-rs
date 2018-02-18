use std::time::{Instant, Duration};
use glium::Display;
use nalgebra::{Point3, Vector2};
use uuid::Uuid;
use ecs::{SystemTrait, Assembly, EcsError, LoopStageFlag, DispatchEvents};
use event::{EngineEvent, EngineEventFlag};
use singletons::Singletons;
use common::ui_element::{UiElement, UiElementError as RootUiElementError};
use components::camera::Camera;
use components::description::Description;
use components::model::Model;
use components::ui_state::UiState;

pub struct SpeechBubbleController {
    display: Display,
}

impl SpeechBubbleController {
    /// Creates a new `SpeechBubbleController`.
    pub fn new(display: &Display) -> Self {
        SpeechBubbleController {
            display: display.clone(),
        }
    }
    /// Creates a new speech-bubble `UiElement` and attaches it to the `UiState`.
    fn create_speech_bubble(&self, entities: &mut Assembly, aux: &mut Singletons, target: &str, content: &str, lifetime: u64) -> Result<(), SpeechBubbleError> {
        // Attempt to find the entity named in `target` and retreive its world position.
        let entity_pos_world = entities.rsf2::<_, Description, Model>(|&(_, d, _)| d.name == target)
            .map(|(_, _, m)| Point3::from_coordinates(*m.translation()))
            .map_err(|e| SpeechBubbleError::EntityNameNotFound(target.into(), e))?;

        // Project the entity position to normalized device coordinates (this requires the camera
        // entity).
        let (entity_pos_ndc, dimensions) = entities.rs1::<Camera>()
            .map(|(_, c)| (c.world_point_to_ndc(&entity_pos_world), Vector2::new(c.dimensions[0] as f32, c.dimensions[1] as f32)))
            .expect("Could not access the Camera component");

        // Obtain a mutable reference to the `UiState`.
        let (_, ui_state) = entities.ws1::<UiState>()
            .expect("Could not access the UiState component");

        // Create the text box
        let element = UiElement::create_textbox(&self.display, &mut aux.factory,
                                                &mut ui_state.font_cache,
                                                &ui_state.speech_bubble.margin,
                                                &ui_state.speech_bubble.font,
                                                &ui_state.speech_bubble.rect_shaders,
                                                &ui_state.speech_bubble.rect_textures,
                                                &ui_state.speech_bubble.text_shaders,
                                                &entity_pos_ndc.coords,
                                                &ui_state.speech_bubble.relative_position_offset,
                                                &dimensions, ui_state.speech_bubble.text_width,
                                                &content)?;

        // Create and register the element.
        let id = Uuid::new_v4();
        ui_state.elements.insert(id, element);
        ui_state.lifetimes.insert(id, (Instant::now(), Duration::new(lifetime, 0)));

        Ok(())
    }
    /// Checks the lifetimes of the registered `UiElement`s and removes those with expired
    /// lifetimes.
    fn update_lifetimes(&self, entities: &mut Assembly) {
        entities.ws1::<UiState>()
            .map(|(_, u)| {
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
            .expect("Could not access the UiState component")
    }
}

impl SystemTrait<EngineEvent, Singletons> for SpeechBubbleController {
    /// The `SpeechBubbleController` depends on the presence of exactly one `UiState` and exactly one
    /// `Camera` component.
    fn verify_requirements(&self, entities: &Assembly) -> bool {
        entities.count1::<UiState>() == 1 && entities.count1::<Camera>() == 1
    }
    /// `SpeechBubbleController` subscribes to the `handle_event` and update calls.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT | LoopStageFlag::UPDATE
    }
    /// `SpeechBubbleController` subscribes to the `SpeechBubble` event.
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::SPEECH_BUBBLE
    }
    fn handle_event(&mut self, entities: &mut Assembly, aux: &mut Singletons, event: &EngineEvent) -> DispatchEvents<EngineEvent> {
        match *event {
            EngineEvent::SpeechBubble(ref t, ref c, l) => {
                self.create_speech_bubble(entities, aux, t, c, l)
                    .unwrap_or_else(|e| warn!("Could not create a speech bubble: {}", e))
            },
            _ => (),
        }
        (None, None)
    }
    fn update(&mut self, entities: &mut Assembly, _: &mut Singletons, _: &Duration, _: &Duration) -> DispatchEvents<EngineEvent> {
        self.update_lifetimes(entities);
        (None, None)
    }
}

#[derive(Debug, Fail)]
pub enum SpeechBubbleError {
    #[fail(display = "The entity name '{}' could not be uniquely identified.", _0)]
    EntityNameNotFound(String, #[cause] EcsError),
    #[fail(display = "{}", _0)]
    UiElementError(#[cause] RootUiElementError),
}

impl From<RootUiElementError> for SpeechBubbleError {
    fn from(value: RootUiElementError) -> Self {
        SpeechBubbleError::UiElementError(value)
    }
}
