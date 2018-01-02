//! The `camera` module provides the `Camera` component.

use ecs::ComponentTrait;
use nalgebra::{Perspective3, Isometry3, Matrix4, Vector3, Point3, Point2};
use alga::linear::{Transformation, ProjectiveTransformation};

/// The `Camera` encapsulates functionality necessary to provide a camera to the `Renderer`.
pub struct Camera {
    /// Provides access to the projection-view matrix. It is recalculated with changes to the
    /// `Camera`.
    pub matrix: Matrix4<f32>,
    /// Provides access to the projection data (not a matrix, but constituents).
    projection: Perspective3<f32>,
    /// Provides access to the view data (not a matrix, but constituents).
    view: Isometry3<f32>,
    /// Provides access to the viewport dimensions.
    dimensions: [u32; 2],
}

impl Camera {
    /// Creates a new instance of `Camera`.
    pub fn new(dims: [u32; 2], fov_y: f32, z_near: f32, z_far: f32, eye: &Point3<f32>, target: &Point3<f32>, up: &Vector3<f32>) -> Self {
        let projection = Perspective3::new(dims[0] as f32 / dims[1] as f32, fov_y, z_near, z_far);
        let view = Isometry3::look_at_rh(eye, target, up);
        Self {
            matrix: projection.as_matrix() * view.to_homogeneous(),
            projection: projection,
            view: view,
            dimensions: dims,
        }
    }
    /// Changes the aspect ratio of the `Camera` projection matrix.
    pub fn set_dimensions(&mut self, dims: [u32; 2]) {
        self.projection.set_aspect(dims[0] as f32 / dims[1] as f32);
        self.dimensions = dims;
        self.recalculate_matrix()
    }
    /// Transforms a point in world-space to normalized device coordinates.
    pub fn world_point_to_ndc(&self, point: &Point3<f32>) -> Point3<f32> {
        self.projection.project_point(&self.view.transform_point(point))
    }
    /// Transforms a point in normalized device coordinates to world-space.
    pub fn ndc_point_to_world(&self, point: &Point3<f32>) -> Point3<f32> {
        self.view.inverse_transform_point(&self.projection.unproject_point(point))

    }
    /// Transforms a point in normalized device coordinates to screen-space.
    pub fn ndc_point_to_screen(&self, point: &Point3<f32>) -> Point2<f32> {
        let w = self.dimensions[0] as f32;
        let h = self.dimensions[1] as f32;
        Point2::new((w / 2.0) * (point.x + 1.0),
                    (h / 2.0) * (point.y + 1.0))
    }
    /// Transforms a screen point to normalized device coordinates.
    pub fn screen_point_to_ndc(&self, point: &Point2<f32>) -> Point3<f32> {
        let w = self.dimensions[0] as f32;
        let h = self.dimensions[1] as f32;
        let n = self.projection.znear();
        let f = self.projection.zfar();
        Point3::new((2.0 * point.x) / w - 1.0,
                    (2.0 * point.y) / h - 1.0,
                    (n + f) / (n - f))
    }
    /// Transforms a point in world-space to a screen point.
    pub fn world_point_to_screen(&self, point: &Point3<f32>) -> Point2<f32> {
        self.ndc_point_to_screen(&self.world_point_to_ndc(point))
    }
    /// Transforms a screen point to world-space.
    pub fn screen_point_to_world(&self, point: &Point2<f32>) -> Point3<f32> {
        self.ndc_point_to_world(&self.screen_point_to_ndc(point))
    }
    /// Recalculates the projection-view matrix.
    fn recalculate_matrix(&mut self) {
        self.matrix = self.projection.as_matrix() * self.view.to_homogeneous()
    }
}

impl ComponentTrait for Camera {}
