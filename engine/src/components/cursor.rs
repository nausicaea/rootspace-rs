use nalgebra::Point2;
use glium::glutin::ElementState;

#[derive(Component)]
pub struct Cursor {
    pub position: Point2<u32>,
    pub left_button: ElementState,
    pub right_button: ElementState,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            position: Point2::new(0, 0),
            left_button: ElementState::Released,
            right_button: ElementState::Released,
        }
    }
}

impl Cursor {
    pub fn new() -> Self {
        Default::default()
    }
}
