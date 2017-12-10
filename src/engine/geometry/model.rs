use std::ops::{Deref, DerefMut};
use nalgebra::{Vector3, Isometry3};
use ecs::ComponentTrait;
use super::super::Float;

/// Provides an abstraction for the model matrix for each 3D object.
pub struct Model {
    inner: Isometry3<Float>,
}

impl Model {
    /// Creates a new `Model` component from a translation and axis-angle vector.
    pub fn new(translation: &Vector3<Float>, axisangle: &Vector3<Float>) -> Self {
        Model {
            inner: Isometry3::new(translation.clone(), axisangle.clone()),
        }
    }
}

impl Deref for Model {
    type Target = Isometry3<Float>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ComponentTrait for Model {}
