use std::ops::{Deref, DerefMut};
use nalgebra::{Isometry3, Vector3, Point3};
use ecs::ComponentTrait;

/// Provides an abstraction for the view matrix (used to make a camera).
#[derive(Clone)]
pub struct View {
    inner: Isometry3<f32>,
}

impl View {
    /// Given a position, a target and an up direction, create a new instance of `View`.
    pub fn new(eye: &Point3<f32>, target: &Point3<f32>, up: &Vector3<f32>) -> Self {
        View {
            inner: Isometry3::look_at_rh(eye, target, up),
        }
    }
}

impl Deref for View {
    type Target = Isometry3<f32>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for View {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ComponentTrait for View {}
