use nalgebra::{Point3, Vector3, Scalar};

/// Characterises a ray (a line segment with an origin and direction).
pub struct Ray<N> where N: Scalar {
    pub origin: Point3<N>,
    pub direction: Vector3<N>,
}

