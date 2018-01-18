use alga::linear::Transformation;
use ecs::Assembly;
use common::ray::{Ray, RaycastHit};
use components::model::Model;
use components::bounding_volume::BoundingVolume;

pub struct Physics {
}

impl Default for Physics {
    fn default() -> Self {
        Self {
        }
    }
}

impl Physics {
    /// Returns the first intersection of the ray with any object in the `Assembly` with a
    /// `BoundingVolume` component. Currently no spatial partitioning is performed, thus, this
    /// algorithm is likely to be very slow.
    pub fn raycast(&mut self, entities: &Assembly, ray: &Ray<f32>) -> Option<RaycastHit> {
        for (e, m, b) in entities.r2::<Model, BoundingVolume>() {
            // Decompose the model matrix into a TRS matrix. TODO: This call should be cached somehow.
            let transform = m.decompose();

            // Transform the ray to the local model coordinate system.
            let transformed_ray = ray.inverse_transform(&transform)?;

            // Perform the intersection test.
            if let Some((_, p)) = b.intersect_ray(&transformed_ray) {
                return Some(RaycastHit {
                    target: e,
                    point: m.transform_point(&p),
                });
            }
        }
        None
    }
}
