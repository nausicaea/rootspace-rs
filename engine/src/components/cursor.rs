use std::collections::HashMap;
use nalgebra::Point2;
use glium::glutin::{ElementState, MouseButton};

/// The `Cursor` stores information about the pointing device in use by the operating system.
#[derive(Component)]
pub struct Cursor {
    /// Holds the current position of the `Cursor` in screen-space coordinates (pixel values).
    pub position: Point2<u32>,
    /// Holds the current state of the mouse buttons.
    pub buttons: HashMap<MouseButton, ElementState>,
}

impl Default for Cursor {
    /// Provides a default instance of `Cursor`.
    fn default() -> Self {
        Cursor {
            position: Point2::new(0, 0),
            buttons: Default::default(),
        }
    }
}

impl Cursor {
    /// Creates a new `Cursor` component.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Indicates the direction of the derivative of a square wave function, as is the case for mouse
/// button presses. Thus, when a mouse button press goes from released to pressed, it's flank is
/// `Down`, and `Up` in the other direction.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FlankDirection {
    /// When a binary function goes from 0 to 1, it's flank goes `Up`.
    Up,
    /// When a binary function goes from 1 to 0, it's flank goes `Down`.
    Down,
}
