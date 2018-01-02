//! The `model` module provides access to the `Model` component.

use std::ops::{Deref, DerefMut, Mul};
use nalgebra::{Vector3, Isometry3, Affine3, Matrix4};
use ecs::ComponentTrait;

/// `Model` provides an abstraction for the model matrix for each 3D object.
#[derive(Clone)]
pub struct Model {
    /// Provides access to the model matrix (an affine TRS matrix).
    inner: Affine3<f32>,
}

impl Model {
    /// Creates a new `Model` component from a translation and axis-angle vector.
    pub fn new(translation: Vector3<f32>, axisangle: Vector3<f32>, scale: Vector3<f32>) -> Self {
        let isometry = Isometry3::new(translation, axisangle);
        let scale = Affine3::from_matrix_unchecked(Matrix4::new(scale.x, 0.0, 0.0, 0.0,
                                                                0.0, scale.y, 0.0, 0.0,
                                                                0.0, 0.0, scale.z, 0.0,
                                                                0.0, 0.0, 0.0, 1.0));
        Model {
            inner: isometry * scale,
        }
    }
    /// Creates a new `Model` component equivalent to an identity matrix.
    pub fn identity() -> Self {
        Model {
            inner: Affine3::identity(),
        }
    }
}

impl Deref for Model {
    type Target = Affine3<f32>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Mul for &'a Model {
    type Output = Model;

    fn mul(self, rhs: Self) -> Self::Output {
        Model {
            inner: self.inner * rhs.inner,
        }
    }
}

impl ComponentTrait for Model {}
