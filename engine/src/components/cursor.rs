use std::collections::HashMap;
use nalgebra::Point2;
use glium::glutin::{MouseButton, ElementState};

#[derive(Component)]
pub struct Cursor {
    pub position: Point2<u32>,
    pub buttons: HashMap<MouseButton, ElementState>,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            position: Point2::new(0, 0),
            buttons: Default::default(),
        }
    }
}

impl Cursor {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FlankDirection {
    Up,
    Down,
}
