use std::collections::HashMap;
use uuid::Uuid;
use ecs::ComponentTrait;
use super::ui_element::UiElement;

pub struct UiState {
    elements: HashMap<Uuid, UiElement>,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            elements: HashMap::new(),
        }
    }
}

impl ComponentTrait for UiState {}
