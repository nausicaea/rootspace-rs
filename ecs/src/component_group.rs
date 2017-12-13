use std::any::{Any, TypeId};
use std::collections::HashMap;

use error::EcsError;

const DOWNCAST_ERROR: &str = "Was unable to downcast the requested component from Any.";

/// All components must implement this trait.
pub trait ComponentTrait: Any {}

/// Groups multiple components of different types together.
#[derive(Default, Debug)]
pub struct ComponentGroup {
    components: HashMap<TypeId, Box<Any>>
}

impl ComponentGroup {
    /// Creates a new, empty group.
    pub fn new() -> Self {
        Default::default()
    }
    /// Inserts a new component into the group. If available, returns the previous component of the
    /// same type.
    pub fn insert<C: ComponentTrait>(&mut self, component: C) -> Option<C> {
        self.components
            .insert(TypeId::of::<C>(), Box::new(component))
            .map(|c| *c.downcast::<C>().expect(DOWNCAST_ERROR))
    }
    /// Removes the component of a particular type from the group an return it.
    pub fn remove<C: ComponentTrait>(&mut self) -> Option<C> {
        self.components
            .remove(&TypeId::of::<C>())
            .map(|c| *c.downcast::<C>().expect(DOWNCAST_ERROR))
    }
    /// Returns `true` if the group contains a component of the specified type.
    pub fn has<C: ComponentTrait>(&self) -> bool {
        self.components
            .contains_key(&TypeId::of::<C>())
    }
    /// Borrows an instance of a component of the specified type.
    pub fn borrow<C: ComponentTrait>(&self) -> Result<&C, EcsError> {
        self.components
            .get(&TypeId::of::<C>())
            .map(|c| c.downcast_ref::<C>().expect(DOWNCAST_ERROR))
            .ok_or_else(|| EcsError::ComponentNotFound(type_names!(C)))
    }
    /// Mutably borrows an instance of a component of the specified type.
    pub fn borrow_mut<C: ComponentTrait>(&mut self) -> Result<&mut C, EcsError> {
        self.components
            .get_mut(&TypeId::of::<C>())
            .map(|c| c.downcast_mut::<C>().expect(DOWNCAST_ERROR))
            .ok_or_else(|| EcsError::ComponentNotFound(type_names!(C)))
    }
}
