use std::collections::HashMap;
use std::time::{Instant, Duration};
use uuid::Uuid;
use ecs::ComponentTrait;
use interface::ui_element::UiElement;

#[derive(Default)]
pub struct UiState {
    pub elements: HashMap<Uuid, UiElement>,
    pub lifetimes: HashMap<Uuid, (Instant, Duration)>
}

impl UiState {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ComponentTrait for UiState {}