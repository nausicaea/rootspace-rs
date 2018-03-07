use std::time::Duration;
use num_traits::float::Float;
use alga::linear::Transformation;
use ecs::Assembly;
use common::ray::{ObjectHit, Ray};
use components::model::Model;
use components::bounding_volume::BoundingVolume;

/// The `PhysicsController` provides means to perform collision detection and other physics
/// operations.
#[derive(Default)]
pub struct PhysicsController;

impl PhysicsController {
    /// Returns the first intersection of the ray with any object in the `Assembly` with a
    /// `BoundingVolume` component. Currently no spatial partitioning is performed,
    /// thus, this algorithm is likely to be very slow.
    pub fn raycast(&mut self, entities: &Assembly, ray: &Ray<f32>) -> Option<ObjectHit<f32>> {
        for (e, m, b) in entities.r2::<Model, BoundingVolume>() {
            // Transform the ray to the local model coordinate system.
            let transformed_ray = ray.inverse_transform(m.decomposed())?;

            // Perform the intersection test.
            if let Some((_, p)) = b.intersect_ray(&transformed_ray) {
                return Some(ObjectHit {
                    target: e,
                    point: m.matrix().transform_point(&p),
                });
            }
        }
        None
    }
}

/// Returns the floating point representation of a `Duration`. Internally, `Duration` uses `u64` to
/// represent seconds and nanoseconds. But large 64-bit integers are not necessarily representable
/// as floating point number. Beware.
pub fn duration_as_float<F: Float>(duration: Duration) -> Option<F> {
    let seconds = F::from(duration.as_secs())?;
    let nanos = F::from(duration.subsec_nanos())?;
    let conv = F::from(1e-9)?;

    Some(nanos.mul_add(conv, seconds))
}
