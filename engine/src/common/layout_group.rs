use nalgebra::Vector2;

#[derive(Debug, Clone, PartialEq)]
pub struct MarginGroup {
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
}

impl MarginGroup {
    pub fn new(right: f32, left: f32, top: f32, bottom: f32) -> Self {
        MarginGroup {
            right,
            left,
            top,
            bottom,
        }
    }
    pub fn screen_to_ndc(&self, dimensions: &Vector2<f32>) -> Self {
        MarginGroup {
            right: self.right / dimensions.x,
            left: self.left / dimensions.x,
            top: self.top / dimensions.y,
            bottom: self.bottom / dimensions.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub center: Vector2<f32>,
    pub dimensions: Vector2<f32>,
}
