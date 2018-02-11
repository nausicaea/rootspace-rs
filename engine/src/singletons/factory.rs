use std::collections::hash_map::{HashMap, DefaultHasher};
use std::hash::{Hash, Hasher};
use glium::Display;
use common::resource_group::{ShaderGroup, TextureGroup};
use components::material::{Material, MaterialError};

/// `ComponentFactory` provides a way to create components and cache them for reuse later. Due to
/// the internal use of `Rc`, componens created thus are immutable. This allows to save a lot of
/// time with shader compilation once the cache can be serialized to disk or when objects are
/// reused frequently.
#[derive(Default)]
pub struct ComponentFactory {
    /// A cache of `Material` components.
    materials: HashMap<u64, Material>,
}

impl ComponentFactory {
    /// Creates a new `ComponentFactory`.
    pub fn new() -> Self {
        Default::default()
    }
    /// Creates a new `Material` component or returns a cached instance with the specified
    /// parameters.
    pub fn new_material(&mut self, display: &Display, shaders: &ShaderGroup, textures: &TextureGroup) -> Result<Material, FactoryError> {
        let hash = self.calculate_material_hash(shaders, textures);

        if self.materials.contains_key(&hash) {
            Ok(self.materials.get(&hash).unwrap_or_else(|| unreachable!()).clone())
        } else {
            let material = Material::new(display, shaders.clone(), textures.clone())?;
            self.materials.insert(hash, material.clone());
            Ok(material)
        }
    }
    fn calculate_material_hash(&self, shaders: &ShaderGroup, textures: &TextureGroup) -> u64 {
        let mut s = DefaultHasher::new();
        shaders.hash(&mut s);
        textures.hash(&mut s);
        s.finish()
    }
}

/// Operations of the `ComponentFactory` may fail with the following errors.
#[derive(Debug, Fail)]
pub enum FactoryError {
    #[fail(display = "{}", _0)]
    MaterialCreationError(#[cause] MaterialError),
}

impl From<MaterialError> for FactoryError {
    /// Converts a `MaterialError` to a `FactoryError`.
    fn from(value: MaterialError) -> Self {
        FactoryError::MaterialCreationError(value)
    }
}
