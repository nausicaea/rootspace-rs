use nalgebra::{Point3, Vector3};

/// Characterises a ray (a line segment with an origin and direction).
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

