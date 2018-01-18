use nalgebra::Point3;
use alga::linear::Transformation;
use ecs::{Entity, Assembly};
use common::ray::Ray;
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
            // Transform the ray to the local model coordinate system.
            let transformed_ray = ray.inverse_transform(&m.decompose())?;

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

pub struct RaycastHit {
    pub target: Entity,
    pub point: Point3<f32>,
}
