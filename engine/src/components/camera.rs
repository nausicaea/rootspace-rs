//! The `camera` module provides the `Camera` component.

use std::f32;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point2, Point3, Unit, Vector3};
use alga::linear::{ProjectiveTransformation, Transformation};
use common::ray::Ray;

/// The `Camera` encapsulates functionality necessary to provide a camera to the `Renderer`.
#[derive(Serialize, Deserialize, Component)]
pub struct Camera {
    /// Provides access to the viewport dimensions.
    pub dimensions: [u32; 2],
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
    pub fn new(
        dims: [u32; 2],
        fov_y: f32,
        z_near: f32,
        z_far: f32,
        eye: &Point3<f32>,
        target: &Point3<f32>,
        up: &Vector3<f32>,
    ) -> Self {
        let projection = Perspective3::new(dims[0] as f32 / dims[1] as f32, fov_y, z_near, z_far);
        let view = Isometry3::look_at_rh(eye, target, up);

        Self {
            dimensions: dims,
            matrix: projection.as_matrix() * view.to_homogeneous(),
            projection: projection,
            view: view,
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
        self.projection
            .project_point(&self.view.transform_point(point))
    }
    /// Transforms a point in normalized device coordinates to world-space.
    pub fn ndc_point_to_world(&self, point: &Point3<f32>) -> Point3<f32> {
        self.view
            .inverse_transform_point(&self.projection.unproject_point(point))
    }
    /// Transforms a point in normalized device coordinates to screen-space.
    pub fn ndc_point_to_screen(&self, point: &Point3<f32>) -> Point2<u32> {
        let w = self.dimensions[0] as f32;
        let h = self.dimensions[1] as f32;

        Point2::new(
            ((w / 2.0) * (point.x + 1.0)).ceil() as u32,
            ((h / 2.0) * (1.0 - point.y)).ceil() as u32,
        )
    }
    /// Transforms a screen point to normalized device coordinates.
    pub fn screen_point_to_ndc(&self, point: &Point2<u32>) -> Point3<f32> {
        let w = self.dimensions[0] as f32;
        let h = self.dimensions[1] as f32;
        let n = self.projection.znear();
        let f = self.projection.zfar();

        Point3::new(
            (2.0 * point.x as f32) / w - 1.0,
            1.0 - (2.0 * point.y as f32) / h,
            ((f + n) / (f - n)).floor(),
        )
    }
    /// Transforms a point in world-space to a screen point.
    pub fn world_point_to_screen(&self, point: &Point3<f32>) -> Point2<u32> {
        self.ndc_point_to_screen(&self.world_point_to_ndc(point))
    }
    /// Transforms a screen point to world-space.
    pub fn screen_point_to_world(&self, point: &Point2<u32>) -> Point3<f32> {
        self.ndc_point_to_world(&self.screen_point_to_ndc(point))
    }
    /// Transforms a screen point to world-space as a ray originating from the camera.
    pub fn screen_point_to_ray(&self, point: &Point2<u32>) -> Option<Ray<f32>> {
        let origin = -self.view.translation.vector;
        let direction = self.screen_point_to_world(point).coords;

        Unit::try_new(direction, f32::EPSILON).map(|d| Ray {
            origin: Point3::from_coordinates(origin),
            direction: d,
        })
    }
    /// Recalculates the projection-view matrix.
    fn recalculate_matrix(&mut self) {
        self.matrix = self.projection.as_matrix() * self.view.to_homogeneous()
    }
}
