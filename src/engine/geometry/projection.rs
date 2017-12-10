use std::ops::{Deref, DerefMut};
use nalgebra::{Scalar, Real, Perspective3};
use ecs::ComponentTrait;

pub struct Projection<N: Scalar + Real> {
    inner: Perspective3<N>,
}

impl<N: Scalar + Real> Projection<N> {
    pub fn new(aspect: N, fov_y: N, z_near: N, z_far: N) -> Projection<N> {
        Projection {
            inner: Perspective3::new(aspect, fov_y, z_near, z_far)
        }
    }
}

impl<N: Scalar + Real> Deref for Projection<N> {
    type Target = Perspective3<N>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<N: Scalar + Real> DerefMut for Projection<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<N: Scalar + Real> ComponentTrait for Projection<N> {}
