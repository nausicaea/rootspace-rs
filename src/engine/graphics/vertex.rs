use super::super::Float;

/// Describes the collection of physical coordinates, texture coordinates and vertex normals.
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [Float; 3],
    tex_coord: [Float; 2],
    normal: [Float; 3],
}

impl Vertex {
    /// Creates a new `Vertex`.
    pub fn new(pos: [Float; 3], uv: [Float; 2], norm: [Float; 3]) -> Self {
        Vertex {
            position: pos,
            tex_coord: uv,
            normal: norm,
        }
    }
}

implement_vertex!(Vertex, position, tex_coord, normal);
