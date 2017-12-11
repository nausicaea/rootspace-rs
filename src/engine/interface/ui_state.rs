use std::collections::HashMap;
use std::time::{Instant, Duration};
use uuid::Uuid;
use ecs::ComponentTrait;
use super::ui_element::UiElement;

pub struct UiState {
    pub elements: HashMap<Uuid, UiElement>,
    pub lifetimes: HashMap<Uuid, (Instant, Duration)>
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            elements: HashMap::new(),
            lifetimes: HashMap::new(),
        }
    }
}

impl ComponentTrait for UiState {}
