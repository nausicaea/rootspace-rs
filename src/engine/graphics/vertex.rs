use super::super::Float;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [Float; 3],
    tex_coord: [Float; 2],
    normal: [Float; 3],
}

implement_vertex!(Vertex, position, tex_coord, normal);
