//! The `model` module provides access to the `Model` component.

use std::ops::Mul;
use nalgebra::{Vector3, Isometry3, Affine3, Matrix4, UnitQuaternion};
use common::affine_transform::AffineTransform;

/// `Model` provides an abstraction for the model matrix for each 3D object.
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct Model {
    /// Provides access to the model matrix (an affine matrix).
    inner: Affine3<f32>,
    /// Provides access to the decomposed form of the model matrix.
    decomp: AffineTransform<f32>,
}

impl Default for Model {
    /// Creates a new `Model` component equivalent to an identity matrix.
    fn default() -> Self {
        Model::identity()
    }
}

impl Model {
    /// Creates a new `Model` component from a translation, axis-angle vector, and non-uniform
    /// scale.
    pub fn new(translation: Vector3<f32>, axisangle: Vector3<f32>, scale: Vector3<f32>) -> Self {
        let isometry = Isometry3::new(translation, axisangle);
        let scale_matrix = Affine3::from_matrix_unchecked(Matrix4::new(scale.x, 0.0, 0.0, 0.0,
                                                                       0.0, scale.y, 0.0, 0.0,
                                                                       0.0, 0.0, scale.z, 0.0,
                                                                       0.0, 0.0, 0.0, 1.0));
        Model {
            inner: isometry * scale_matrix,
            decomp: AffineTransform::from_parts(isometry.translation, isometry.rotation, scale),
        }
    }
    /// Creates a new `Model` component equivalent to an identity matrix.
    pub fn identity() -> Self {
        Model {
            inner: Affine3::identity(),
            decomp: AffineTransform::identity(),
        }
    }
    /// Returns a reference to the affine model matrix.
    pub fn matrix(&self) -> &Matrix4<f32> {
        self.inner.matrix()
    }
    /// Returns a reference to the decomposed affine model matrix, where the translational,
    /// rotational and non-uniform scaling components are accessible.
    pub fn decomposed(&self) -> &AffineTransform<f32> {
        &self.decomp
    }
    /// Returns a reference to the translational component of the model matrix.
    pub fn translation(&self) -> &Vector3<f32> {
        &self.decomp.translation.vector
    }
    /// Sets a new translational component and recalculates the compound affine matrix.
    pub fn set_translation(&mut self, value: Vector3<f32>) {
        if self.decomp.translation.vector != value {
            self.decomp.translation.vector = value;
            self.inner = self.decomp.into();
        }
    }
    /// Returns a reference to the rotational component of the model matrix.
    pub fn rotation(&self) -> &UnitQuaternion<f32> {
        &self.decomp.rotation
    }
    /// Sets a new rotational component and recalculates the compound affine matrix.
    pub fn set_rotation(&mut self, value: UnitQuaternion<f32>) {
        if self.decomp.rotation != value {
            self.decomp.rotation = value;
            self.inner = self.decomp.into();
        }
    }
    /// Returns a reference to the non-uniform scale component of the model matrix.
    pub fn scale(&self) -> &Vector3<f32> {
        &self.decomp.scale
    }
    /// Sets a new non-uniform scale component and recalculates the compound affine matrix.
    pub fn set_scale(&mut self, value: Vector3<f32>) {
        if self.decomp.scale != value {
            self.decomp.scale = value;
            self.inner = self.decomp.into();
        }
    }
}

impl<'a> Mul for &'a Model {
    type Output = Model;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = self.inner * rhs.inner;
        Model {
            inner: product,
            decomp: product.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_getters_and_setters() {
        let mut model = Model::identity();

        let translation = *model.translation();
        model.set_translation(translation + Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(model.translation(), &Vector3::new(1.0, 2.0, 3.0));

        let rotation = *model.rotation();
        model.set_rotation(rotation * UnitQuaternion::identity());
        assert_eq!(model.rotation(), &UnitQuaternion::identity());

        let scale = *model.scale();
        model.set_scale(scale.component_mul(&Vector3::new(2.0, 3.0, 4.0)));
        assert_eq!(model.scale(), &Vector3::new(2.0, 3.0, 4.0));
    }
}
