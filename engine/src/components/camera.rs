//! The `camera` module provides the `Camera` component.

use ecs::ComponentTrait;
use nalgebra::{Perspective3, Isometry3, Matrix4, Vector3, Point3};
use alga::linear::Transformation;

/// The `Camera` encapsulates functionality necessary to provide a camera to the `Renderer`.
pub struct Camera {
    /// Provides access to the projection-view matrix. It is recalculated with changes to the
    /// `Camera`.
    pub matrix: Matrix4<f32>,
    /// Provides access to the projection data (not a matrix, but constituents).
    projection: Perspective3<f32>,
    /// Provides access to the view data (not a matrix, but constituents).
    view: Isometry3<f32>,
}

impl Camera {
    /// Creates a new instance of `Camera`.
    pub fn new(aspect: f32, fov_y: f32, z_near: f32, z_far: f32, eye: &Point3<f32>, target: &Point3<f32>, up: &Vector3<f32>) -> Self {
        let projection = Perspective3::new(aspect, fov_y, z_near, z_far);
        let view = Isometry3::look_at_rh(eye, target, up);
        Self {
            matrix: projection.as_matrix() * view.to_homogeneous(),
            projection: projection,
            view: view,
        }
    }
    /// Changes the aspect ratio of the `Camera` projection matrix.
    pub fn set_aspect(&mut self, aspect: f32) {
        self.projection.set_aspect(aspect);
        self.recalculate_matrix()
    }
    /// Transforms a point in world-space to normalized device coordinates.
    pub fn world_point_to_ndc(&self, point: &Point3<f32>) -> Point3<f32> {
        self.projection.project_point(&self.view.transform_point(point))
    }
    /// Recalculates the projection-view matrix.
    fn recalculate_matrix(&mut self) {
        self.matrix = self.projection.as_matrix() * self.view.to_homogeneous()
    }
}

impl ComponentTrait for Camera {}
