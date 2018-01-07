//! The `bounding_volume` module provides access to the `BoundingVolume` component.

use std::f32;
use alga::linear::Transformation;
use nalgebra::{Point3, Vector3, Affine3};
use glium::buffer::ReadError;
use ecs::ComponentTrait;
use components::mesh::Mesh;
use common::vertex::Vertex;
use common::ray::Ray;

/// The `BoundingVolume` component describes simplified volumes of entities or objects to use for
/// collision detection.
#[derive(Debug, Clone)]
pub enum BoundingVolume {
    /// Defines an oriented bounding-box (OBB).
    Obb {center: Point3<f32>, extents: Vector3<f32>, base: [Vector3<f32>; 3]},
    /// Defines a discrete oriented polytope (k-DOP).
    Dop(Vec<(Vector3<f32>, f32, f32)>),
    /// Defines a sphere.
    Sphere {center: Point3<f32>, square_radius: f32},
}

impl BoundingVolume {
    /// Creates an optimal oriented bounding-box from a set of vertices (`Vertex`) by
    /// determining the minimum and maximum extents of the `Vertex` positions.
    pub fn new_obb(vertices: &[Vertex]) -> Self {
        unimplemented!()
    }
    /// Creates an optimal discrete oriented polytope with `k = 8` in a similar fashion as with
    /// the AABB (which can also be seen as 6-DOP).
    pub fn new_dop8(vertices: &[Vertex]) -> Self {
        let normals = [
            Vector3::new(1.0, 1.0, 1.0).normalize(),
            Vector3::new(1.0, 1.0, -1.0).normalize(),
            Vector3::new(1.0, -1.0, 1.0).normalize(),
            Vector3::new(-1.0, 1.0, 1.0).normalize(),
        ];

        let mut dop_data = Vec::new();
        normals.into_iter()
            .for_each(|n| {
                // Iterate through all vertices and grab both minima and maxima.
                let init = (f32::INFINITY, -f32::INFINITY);

                let (min, max) = vertices.iter()
                    .fold(init, |s, v| {
                        let mut next_s = s;
                        let d = Vector3::new(v.position[0], v.position[1], v.position[2]).dot(n);

                        if d < s.0 {
                            next_s.0 = d
                        } else if d > s.1 {
                            next_s.1 = d
                        }
                        next_s
                    });
                dop_data.push((*n, min, max));
            });

        BoundingVolume::Dop(dop_data)
    }
    /// Creates a near-optimal spherical bounding volume from a set of vertices (`Vertex`) by first
    /// calculating the minimum and maximum extents of the vertices (same as the AABB case), and
    /// subsequently calculating the sphere radius as the largest center-vertex distance.
    pub fn new_sphere(vertices: &[Vertex]) -> Self {
        // Iterate through all vertices and grab both minima and maxima.
        let init = (Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            Vector3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY));

        let (min, max) = vertices.iter()
            .fold(init, |s, v| {
                let mut next_s = s;
                let p = Vector3::new(v.position[0], v.position[1], v.position[2]);

                if p.x < s.0.x {
                    next_s.0.x = p.x
                } else if p.x > s.1.x {
                    next_s.1.x = p.x
                }
                if p.y < s.0.y {
                    next_s.0.y = p.y
                } else if p.y > s.1.y {
                    next_s.1.y = p.y
                }
                if p.z < s.0.z {
                    next_s.0.z = p.z
                } else if p.z > s.1.z {
                    next_s.1.z = p.z
                }
                next_s
            });

        // Calculate the bounding volume center.
        let center = (min + max) / 2.0;

        // Calculate the radius of the sphere in a second pass as the largest center-vertex
        // distance.
        let square_radius = vertices.iter()
            .fold(0.0, |s, v| {
                let p = Vector3::new(v.position[0], v.position[1], v.position[2]);
                let d = p - center;
                let next_s = d.dot(&d);

                if next_s > s {
                    next_s
                } else {
                    s
                }
            });

        BoundingVolume::Sphere {
            center: Point3::from_coordinates(center),
            square_radius: square_radius,
        }
    }
    /// Creates an optimal axis-aligned bounding-box from the supplied mesh.
    pub fn from_mesh_obb(mesh: &Mesh) -> Result<Self, ReadError> {
        let vertex_data = mesh.vertices.read()?;
        Ok(Self::new_obb(&vertex_data))
    }
    /// Creates an optimal discrete oriented polytope with `k = 8` from the supplied mesh.
    pub fn from_mesh_dop8(mesh: &Mesh) -> Result<Self, ReadError> {
        let vertex_data = mesh.vertices.read()?;
        Ok(Self::new_dop8(&vertex_data))
    }
    /// Creates a near-optimal spherical bounding volume from the supplied mesh.
    pub fn from_mesh_sphere(mesh: &Mesh) -> Result<Self, ReadError> {
        let vertex_data = mesh.vertices.read()?;
        Ok(Self::new_sphere(&vertex_data))
    }
    pub fn as_transformed(&self, transform: &Affine3<f32>) -> Self {
        match *self {
            BoundingVolume::Sphere {ref center, ref square_radius} => {
                BoundingVolume::Sphere {
                    center: transform.transform_point(center),
                    square_radius: *square_radius,
                }
            },
            _ => unimplemented!(),
        }
    }
    /// Performs an intersection test of the `BoundingVolume` against the supplied `Ray`.
    /// Optionally returns a tuple of `Ray` position and intersection point.
    pub fn intersect_ray(&self, ray: &Ray<f32>) -> Option<(f32, Point3<f32>)> {
        match *self {
            BoundingVolume::Sphere {ref center, ref square_radius} => {
                let l = center.coords - ray.origin.coords;
                let s = l.dot(&ray.direction);
                let l_square = l.dot(&l);

                if s < 0.0 && l_square > *square_radius {
                    return None;
                }

                let m_square = l_square - s.powi(2);

                if m_square > *square_radius {
                    return None;
                }

                let q = (square_radius - m_square).sqrt();

                let t = if l_square > *square_radius {
                    s - q
                } else {
                    s + q
                };

                Some((t, ray.at(t)))
            },
            BoundingVolume::Obb {ref center, ref extents, ref base} => {
                let epsilon = 0.001;
                let mut t_min = -f32::INFINITY;
                let mut t_max = f32::INFINITY;
                let p = center.coords - ray.origin.coords;
                for i in 0..3 {
                    let e = base[i].dot(&p);
                    let f = base[i].dot(&ray.direction);
                    if f.abs() > epsilon {
                        let mut t_1 = (e + extents[i]) / f;
                        let mut t_2 = (e - extents[i]) / f;

                        if t_1 > t_2 {
                            let tmp = t_1;
                            t_1 = t_2;
                            t_2 = tmp;
                        }

                        if t_1 > t_min {
                            t_min = t_1;
                        }

                        if t_2 < t_max {
                            t_max = t_2;
                        }

                        if t_min > t_max || t_max < 0.0 {
                            return None;
                        }
                    } else if (-e - extents[i]) > 0.0 || (-e + extents[i]) < 0.0 {
                        return None;
                    }
                }

                if t_min > 0.0 {
                    Some((t_min, ray.at(t_min)))
                } else {
                    Some((t_max, ray.at(t_max)))
                }
            },
            _ => unimplemented!(),
        }
    }
}

impl ComponentTrait for BoundingVolume {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obb() {
        let min = [-0.5, -0.5];
        let max = [0.5, 0.5];
        let vertices = vec![
            Vertex::new([min[0], max[1], 0.0], [0.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([min[0], min[1], 0.0], [0.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], min[1], 0.0], [1.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], max[1], 0.0], [1.0, 1.0], [0.0, 0.0, 1.0]),
        ];

        match BoundingVolume::new_obb(&vertices) {
            BoundingVolume::Obb {center: c, extents: r, base: b} => {
                assert!(c == Point3::origin(), "Got {:?} instead", c);
                assert!(r == Vector3::new(0.5, 0.5, 0.0), "Got {:?} instead", r);
                assert!(b[0] == Vector3::x(), "Got {:?} instead", b[0]);
                assert!(b[1] == Vector3::y(), "Got {:?} instead", b[1]);
                assert!(b[2] == Vector3::z(), "Got {:?} instead", b[2]);
            },
            bv => panic!("Expected an OBB enum variant, got {:?} instead", bv),
        }
    }
    #[test]
    fn test_dop8() {
        let min = [-0.5, -0.5];
        let max = [0.5, 0.5];
        let vertices = vec![
            Vertex::new([min[0], max[1], 0.0], [0.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([min[0], min[1], 0.0], [0.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], min[1], 0.0], [1.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], max[1], 0.0], [1.0, 1.0], [0.0, 0.0, 1.0]),
        ];

        match BoundingVolume::new_dop8(&vertices) {
            BoundingVolume::Dop(d) => {
                assert!(d.len() == 4, "Got {:?} instead", d.len());
                assert!(d[0].0 == Vector3::new(1.0, 1.0, 1.0).normalize());
                assert!(d[0].1 == -0.57735026, "Got {:?} instead", d[0].1);
                assert!(d[0].2 == 0.57735026, "Got {:?} instead", d[0].2);
                assert!(d[1].0 == Vector3::new(1.0, 1.0, -1.0).normalize());
                assert!(d[1].1 == -0.57735026, "Got {:?} instead", d[1].1);
                assert!(d[1].2 == 0.57735026, "Got {:?} instead", d[1].2);
                assert!(d[2].0 == Vector3::new(1.0, -1.0, 1.0).normalize());
                assert!(d[2].1 == -0.57735026, "Got {:?} instead", d[2].1);
                assert!(d[2].2 == 0.57735026, "Got {:?} instead", d[2].2);
                assert!(d[3].0 == Vector3::new(-1.0, 1.0, 1.0).normalize());
                assert!(d[3].1 == -0.57735026, "Got {:?} instead", d[3].1);
                assert!(d[3].2 == 0.0, "Got {:?} instead", d[3].2);
            },
            bv => panic!("Expected a DOP enum variant, got {:?} instead", bv),
        }
    }
    #[test]
    fn test_sphere() {
        let min = [-0.5, -0.5];
        let max = [0.5, 0.5];
        let vertices = vec![
            Vertex::new([min[0], max[1], 0.0], [0.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([min[0], min[1], 0.0], [0.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], min[1], 0.0], [1.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], max[1], 0.0], [1.0, 1.0], [0.0, 0.0, 1.0]),
        ];

        match BoundingVolume::new_sphere(&vertices) {
            BoundingVolume::Sphere {center: c, square_radius: r} => {
                assert!(c == Point3::origin(), "Got {:?} instead", c);
                assert!(r == 0.5, "Got {:?} instead", r);
            },
            bv => panic!("Expected a sphere enum variant, got {:?} instead", bv),
        }
    }
}
