use nalgebra::{Vector3, Isometry3, Affine3, Matrix4};
use ecs::ComponentTrait;

/// Provides an abstraction for the model matrix for each 3D object.
pub struct Model {
    pub isometry: Isometry3<f32>,
    pub scale: Affine3<f32>,
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
            isometry: isometry,
            scale: scale,
        }
    }
}

impl ComponentTrait for Model {}
