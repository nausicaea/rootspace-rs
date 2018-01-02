use ecs::ComponentTrait;
use nalgebra::{Perspective3, Isometry3, Matrix4, Vector3, Point3};
use alga::linear::Transformation;

pub struct Camera {
    projection: Perspective3<f32>,
    view: Isometry3<f32>,
}

impl Camera {
    pub fn new(aspect: f32, fov_y: f32, z_near: f32, z_far: f32, eye: &Point3<f32>, target: &Point3<f32>, up: &Vector3<f32>) -> Self {
        Self {
            projection: Perspective3::new(aspect, fov_y, z_near, z_far),
            view: Isometry3::look_at_rh(eye, target, up),
        }
    }
    pub fn pv_matrix(&self) -> Matrix4<f32> {
        self.projection.as_matrix() * self.view.to_homogeneous()
    }
    pub fn set_aspect(&mut self, aspect: f32) {
        self.projection.set_aspect(aspect)
    }
    pub fn world_point_to_ndc(&self, point: &Point3<f32>) -> Point3<f32> {
        self.projection.project_point(&self.view.transform_point(point))
    }
}

impl ComponentTrait for Camera {}
