use nalgebra::{Point3, Vector3, Scalar};

/// A `Ray` characterises a ray (a line segment with an origin, direction and infinite length in
/// that direction).
#[derive(Debug, Clone)]
pub struct Ray<N> where N: Scalar {
    /// Specifies the origin of the `Ray`.
    pub origin: Point3<N>,
    /// Specifies the direction of the `Ray`.
    pub direction: Vector3<N>,
}

