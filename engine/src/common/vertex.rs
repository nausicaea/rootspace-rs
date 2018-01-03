/// Describes the collection of physical coordinates, texture coordinates and vertex normals.
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    /// Creates a new `Vertex`.
    pub fn new(pos: [f32; 3], uv: [f32; 2], norm: [f32; 3]) -> Self {
        Vertex {
            position: pos,
            tex_coord: uv,
            normal: norm,
        }
    }
}

implement_vertex!(Vertex, position, tex_coord, normal);
