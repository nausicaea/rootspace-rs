use components::model::Model;
use components::mesh::Mesh;
use components::material::Material;

/// A `UiPrimitive` encodes all data necessary to render the primitive to the display (vertices,
/// indices, material, uniforms).
pub struct UiPrimitive {
    pub model: Model,
    pub mesh: Mesh,
    pub material: Material,
}

impl UiPrimitive {
    /// Creates a new `UiPrimitive`.
    pub fn new(model: Model, mesh: Mesh, material: Material) -> Self {
        UiPrimitive {
            model: model,
            mesh: mesh,
            material: material,
        }
    }
}

