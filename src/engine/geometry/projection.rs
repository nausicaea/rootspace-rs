use std::ops::{Deref, DerefMut};
use nalgebra::Perspective3;
use ecs::ComponentTrait;
use super::super::Float;

/// Provides an abstration for the projection matrix (used to make a camera).
pub struct Projection {
    inner: Perspective3<Float>,
}

impl Projection {
    /// Create a new instance of `Projection` given display aspect ratio, vertical field of view,
    /// and near and far z values.
    pub fn new(aspect: Float, fov_y: Float, z_near: Float, z_far: Float) -> Self {
        Projection {
            inner: Perspective3::new(aspect, fov_y, z_near, z_far)
        }
    }
}

impl Deref for Projection {
    type Target = Perspective3<Float>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Projection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ComponentTrait for Projection {}
