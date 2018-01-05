use nalgebra::{Point3, Vector3, Scalar, Unit, Real};

/// A `Ray` characterises a ray (a line segment with an origin, direction and infinite length in
/// that direction).
#[derive(Debug, Clone)]
pub struct Ray<N> where N: Scalar + Real {
    /// Specifies the origin of the `Ray`.
    pub origin: Point3<N>,
    /// Specifies the direction of the `Ray`.
    pub direction: Unit<Vector3<N>>,
}

impl<N> Ray<N> where N: Scalar + Real {
    /// Creates a new `Ray`.
    pub fn new(origin: Point3<N>, direction: Unit<Vector3<N>>) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
    pub fn at(&self, position: N) -> Point3<N> {
        self.origin + self.direction.as_ref() * position
    }
}
