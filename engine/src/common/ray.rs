use std::fmt;
use nalgebra::{Point3, Real, Scalar, Unit, Vector3};
use ecs::Entity;
use common::affine_transform::AffineTransform;

/// A `Ray` characterises a ray (a line segment with an origin, direction and infinite length in
/// that direction).
#[derive(Debug, Clone, PartialEq)]
pub struct Ray<N>
where
    N: Scalar + Real,
{
    /// Specifies the origin of the `Ray`.
    pub origin: Point3<N>,
    /// Specifies the direction of the `Ray`.
    pub direction: Unit<Vector3<N>>,
}

impl<N> Ray<N>
where
    N: Scalar + Real,
{
    /// Creates a new `Ray`.
    pub fn new(origin: Point3<N>, direction: Unit<Vector3<N>>) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
    /// Extends the `Ray` to the specified position and returns the resulting point.
    pub fn at(&self, position: N) -> Point3<N> {
        self.origin + self.direction.as_ref() * position
    }
    /// Transforms the `Ray` into a new coordinate system determined by an `AffineTransform`
    /// matrix.
    pub fn transform(&self, transform: &AffineTransform<N>) -> Option<Self> {
        let new_origin = transform.transform_point(&self.origin);
        let new_direction = Unit::try_new(
            transform.transform_vector(&self.direction),
            N::default_epsilon(),
        )?;

        Some(Ray {
            origin: new_origin,
            direction: new_direction,
        })
    }
    /// Applies the inverse of the supplied `AffineTransform` matrix to the `Ray`.
    pub fn inverse_transform(&self, transform: &AffineTransform<N>) -> Option<Self> {
        let new_origin = transform.inverse_transform_point(&self.origin);
        let new_direction = Unit::try_new(
            transform.inverse_transform_vector(&self.direction),
            N::default_epsilon(),
        )?;

        Some(Ray {
            origin: new_origin,
            direction: new_direction,
        })
    }
}

impl<N> fmt::Display for Ray<N>
where
    N: Scalar + Real,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ray {{origin: {}, direction: {}}}",
            self.origin,
            self.direction.unwrap()
        )
    }
}

/// When performing raycasting, the `ObjectHit` structure indicates a positive intersection test
/// between a `Ray` and a `BoundingVolume`.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectHit<N>
where
    N: Scalar + Real,
{
    /// Names the `Entity` that was hit by the `Ray`.
    pub target: Entity,
    /// Names the point in world space closest to the `Ray` origin at which the intersection took
    /// place.
    pub point: Point3<N>,
}

impl<N> fmt::Display for ObjectHit<N>
where
    N: Scalar + Real,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ObjectHit {{target: {}, point: {}}}",
            self.target, self.point
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_at() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let d = Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0));
        let r = Ray::new(o, d);

        assert!(r.at(0.0) == o);
        assert!(r.at(1.0) == Point3::new(1.0, 0.0, 0.0));
    }
    #[test]
    fn test_transform() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let d = Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0));
        let r = Ray::new(o, d);

        let s = r.transform(&AffineTransform::identity()).unwrap();

        assert!(s == r, "Got {:?} instead", s);
    }
    #[test]
    fn test_inverse_transform() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let d = Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0));
        let r = Ray::new(o, d);

        let s = r.inverse_transform(&AffineTransform::identity()).unwrap();

        assert!(s == r, "Got {:?} instead", s);
    }
}
