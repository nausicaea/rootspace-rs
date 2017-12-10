use std::ops::{Deref, DerefMut};
use nalgebra::{Isometry3, Vector3, Point3};
use ecs::ComponentTrait;
use super::super::Float;

pub struct View {
    inner: Isometry3<Float>,
}

impl View {
    pub fn new(eye: &Point3<Float>, target: &Point3<Float>, up: &Vector3<Float>) -> Self {
        View {
            inner: Isometry3::look_at_rh(eye, target, up),
        }
    }
}

impl Deref for View {
    type Target = Isometry3<Float>;

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
