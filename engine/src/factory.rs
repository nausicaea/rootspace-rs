use std::collections::hash_map::{HashMap, DefaultHasher};
use std::hash::{Hash, Hasher};
use std::path::Path;
use glium::Display;
use components::material::{Material, MaterialError};

#[derive(Debug, Fail)]
pub enum FactoryError {
    #[fail(display = "{}", _0)]
    MaterialCreationError(#[cause] MaterialError),
}

impl From<MaterialError> for FactoryError {
    fn from(value: MaterialError) -> Self {
        FactoryError::MaterialCreationError(value)
    }
}

/// `ComponentFactory` provides a way to create components and cache them for reuse later. Due to
/// the internal use of `Rc`, componens created thus are immutable.
#[derive(Default)]
pub struct ComponentFactory {
    materials: HashMap<u64, Material>,
}

impl ComponentFactory {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_material(&mut self, display: &Display, vs: &Path, fs: &Path, gs: Option<&Path>, dt: Option<&Path>, nt: Option<&Path>) -> Result<Material, FactoryError> {
        let hash = self.calculate_material_hash(vs, fs, gs, dt, nt);

        if self.materials.contains_key(&hash) {
            Ok(self.materials.get(&hash).unwrap_or_else(|| unreachable!()).clone())
        } else {
            let material = Material::new(display, vs, fs, gs, dt, nt)?;
            self.materials.insert(hash, material.clone());
            Ok(material)
        }
    }
    fn calculate_material_hash(&self, vs: &Path, fs: &Path, gs: Option<&Path>, dt: Option<&Path>, nt: Option<&Path>) -> u64 {
        let mut s = DefaultHasher::new();
        vs.hash(&mut s);
        fs.hash(&mut s);
        gs.hash(&mut s);
        dt.hash(&mut s);
        nt.hash(&mut s);
        s.finish()
    }
}
