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
            .ok_or_else(|| EcsError::ComponentNotFound)
    }
    /// Mutably borrows an instance of a component of the specified type.
    pub fn borrow_mut<C: ComponentTrait>(&mut self) -> Result<&mut C, EcsError> {
        self.components
            .get_mut(&TypeId::of::<C>())
            .map(|c| c.downcast_mut::<C>().expect(DOWNCAST_ERROR))
            .ok_or_else(|| EcsError::ComponentNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(PartialEq, Eq)]
    struct Component(u32);

    impl ComponentTrait for Component {}

    #[test]
    fn test_instantiation() {
        let _cg = ComponentGroup::new();
        let _cg: ComponentGroup = Default::default();
    }

    #[test]
    fn test_insert() {
        let mut cg = ComponentGroup::new();

        assert!(cg.insert(Component(0)).is_none());
        let c = cg.insert(Component(1));
        assert!(c.is_some() && c.unwrap() == Component(0));
    }

    #[test]
    fn test_remove() {
        let mut cg = ComponentGroup::new();

        assert!(cg.remove::<Component>().is_none());
        assert!(cg.insert(Component(0)).is_none());
        let c = cg.remove::<Component>();
        assert!(c.is_some() && c.unwrap() == Component(0));
        assert!(cg.remove::<Component>().is_none());
    }

    #[test]
    fn test_has() {
        let mut cg = ComponentGroup::new();

        assert!(!cg.has::<Component>());
        assert!(cg.insert(Component(0)).is_none());
        assert!(cg.has::<Component>());
        assert!(cg.remove::<Component>().is_some());
        assert!(!cg.has::<Component>());
    }

    #[test]
    fn test_borrow() {
        let mut cg = ComponentGroup::new();

        assert!(cg.borrow::<Component>().is_err());
        assert!(cg.insert(Component(0)).is_none());
        let c = cg.borrow::<Component>();
        assert!(c.is_ok() && c.unwrap() == &Component(0));
    }

    #[test]
    fn test_borrow_mut() {
        let mut cg = ComponentGroup::new();

        assert!(cg.borrow_mut::<Component>().is_err());
        assert!(cg.insert(Component(0)).is_none());
        let c = cg.borrow_mut::<Component>().map(|x| {x.0 += 1; x});
        assert!(c.is_ok() && c.unwrap() == &mut Component(1));
    }
}
