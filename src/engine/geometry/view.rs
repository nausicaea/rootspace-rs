use std::ops::{Deref, DerefMut};
use nalgebra::{Scalar, Real, Isometry3, Vector3, Point3};
use ecs::ComponentTrait;

pub struct View<N: Scalar + Real> {
    inner: Isometry3<N>,
}

impl<N: Scalar + Real> View<N> {
    pub fn new(eye: &Point3<N>, target: &Point3<N>, up: &Vector3<N>) -> View<N> {
        View {
            inner: Isometry3::look_at_rh(eye, target, up),
        }
    }
}

impl<N: Scalar + Real> Deref for View<N> {
    type Target = Isometry3<N>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<N: Scalar + Real> DerefMut for View<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<N: Scalar + Real> ComponentTrait for View<N> {}
