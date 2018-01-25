use alga::linear::Transformation;
use ecs::{Entity, Assembly};
use common::ray::{Ray, ObjectHit};
use components::model::Model;
use components::bounding_volume::BoundingVolume;

/// The `PhysicsController` provides means to perform collision detection and other physics
/// operations.
#[derive(Default)]
pub struct PhysicsController;

impl PhysicsController {
    /// Returns the first intersection of the ray with any object in the `Assembly` with a
    /// `BoundingVolume` component. Currently no spatial partitioning or caching is performed,
    /// thus, this algorithm is likely to be very slow.
    pub fn raycast(&mut self, entities: &Assembly, ray: &Ray<f32>) -> Option<ObjectHit<f32>> {
        for (e, m, b) in entities.r2::<Model, BoundingVolume>() {
            // Decompose the model matrix into a TRS matrix. TODO: This call should be cached somehow.
            let transform = m.decompose();

            // Transform the ray to the local model coordinate system.
            let transformed_ray = ray.inverse_transform(&transform)?;

            // Perform the intersection test.
            if let Some((_, p)) = b.intersect_ray(&transformed_ray) {
                return Some(ObjectHit {
                    target: e,
                    point: m.transform_point(&p),
                });
            }
        }
        None
    }
    /// Given the currently active target `Entity`, performs a ray intersection and determines
    /// whether the same object is still active, whether a new object has been hit, or whether no
    /// intersection was found.
    pub fn stateful_raycast(&mut self, entities: &Assembly, ray: &Ray<f32>, current_target: &Option<Entity>) -> StatefulHit {
        if let Some(hit) = self.raycast(entities, ray) {
            if let Some(ref tgt) = *current_target {
                if &hit.target != tgt {
                    StatefulHit::NewHit(hit)
                } else {
                    StatefulHit::RepeatHit(hit)
                }
            } else {
                StatefulHit::NewHit(hit)
            }
        } else {
            StatefulHit::NoHit
        }
    }
}

/// Describes the states of a ray-object intersection given knowledge about previous intersections.
pub enum StatefulHit {
    /// No object was hit by the `Ray`.
    NoHit,
    /// A new object was hit by the `Ray`.
    NewHit(ObjectHit<f32>),
    /// The Same object was hit by the `Ray`, but the `ObjectHit` data have changed.
    RepeatHit(ObjectHit<f32>),
}
