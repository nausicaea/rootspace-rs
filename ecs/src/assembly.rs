use std::collections::HashMap;

use error::EcsError;
use entity::Entity;
use component_group::{ComponentTrait, ComponentGroup};

/// Implements methods that collect all entities' components under the condition that all specified
/// types must be present in each entity.
macro_rules! impl_read {
    ($name:ident, $t:tt) => {
        /// Borrows all instances of the specified component.
        pub fn $name<$t: ComponentTrait>(&self) -> Vec<(Entity, &$t)> {
            self.entities.iter()
                .filter(|&(_, ref g)| g.has::<$t>())
                .map(|(e, g)| (e.clone(), g.borrow::<$t>().unwrap_or_else(|_| unreachable!())))
                .collect()
        }
    };
    ($name:ident, $($t:tt),*) => {
        /// Borrows from all entities that have all specified components.
        pub fn $name<$($t: ComponentTrait),*>(&self) -> Vec<(Entity, $(&$t),*)> {
            self.entities.iter()
                .filter(|&(_, g)| $(g.has::<$t>())&&*)
                .map(|(e, g)| (e.clone(), $(g.borrow::<$t>().unwrap_or_else(|_| unreachable!())),*))
                .collect()
        }
    };
}

/// Implements methods that collect all entities' components under the condition that all
/// specified types must be present in each entity. Additionally accepts a filter function to
/// filter components by their contents.
macro_rules! impl_read_filtered {
    ($name:ident, $t:tt) => {
        /// Borrows all instances of the specified component, if their values pass the specified
        /// filter.
        pub fn $name<F, $t: ComponentTrait>(&self, filter: F) -> Vec<(Entity, &$t)> where for<'r> F: FnMut(&'r (Entity, &$t)) -> bool {
            self.entities.iter()
                .filter(|&(_, ref g)| g.has::<$t>())
                .map(|(e, g)| (e.clone(), g.borrow::<$t>().unwrap_or_else(|_| unreachable!())))
                .filter(filter)
                .collect()
        }
    };
    ($name:ident, $($t:tt),*) => {
        /// Borrows from all entities that have all specified components and whose values pass the
        /// specified filter.
        pub fn $name<F, $($t: ComponentTrait),*>(&self, filter: F) -> Vec<(Entity, $(&$t),*)> where for<'r> F: FnMut(&'r (Entity, $(&$t),*)) -> bool {
            self.entities.iter()
                .filter(|&(_, ref g)| $(g.has::<$t>())&&*)
                .map(|(e, g)| (e.clone(), $(g.borrow::<$t>().unwrap_or_else(|_| unreachable!())),*))
                .filter(filter)
                .collect()
        }
    };
}

/// Implements methods that ensure only a single entity matches the bounds given by the components.
/// Errors otherwise.
macro_rules! impl_read_single {
    ($name:ident, $base:ident, $t:tt) => {
        /// Borrows the specified component, ensuring that only a single entity matches the given
        /// conditions.
        pub fn $name<$t: ComponentTrait>(&self) -> Result<(Entity, &$t), EcsError> {
            let mut components = self.$base::<$t>();

            match components.len() {
                0 => Err(EcsError::ComponentNotFound(type_names!($t))),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound(type_names!($t))),
            }
        }
    };
    ($name:ident, $base:ident, $($t:tt),*) => {
        /// Borrows the specified components, ensuring that only a single entity matches the given
        /// conditions.
        pub fn $name<$($t: ComponentTrait),*>(&self) -> Result<(Entity, $(&$t),*), EcsError> {
            let mut components = self.$base::<$($t),*>();

            match components.len() {
                0 => Err(EcsError::ComponentNotFound(type_names!($($t),*))),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound(type_names!($($t),*))),
            }
        }
    };
}

/// Implements methods that ensure only a single entity matches the bounds given by the components
/// and the specified filter. Errors otherwise.
macro_rules! impl_read_single_filtered {
    ($name:ident, $base:ident, $t:tt) => {
        /// Borrows the specified component, ensuring that only a single entity matches the given
        /// conditions (defined by the component and filter).
        pub fn $name<F, $t: ComponentTrait>(&self, filter: F) -> Result<(Entity, &$t), EcsError> where for<'r> F: FnMut(&'r (Entity, &$t)) -> bool {
            let mut components = self.$base::<F, $t>(filter);

            match components.len() {
                0 => Err(EcsError::ComponentNotFound(type_names!($t))),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound(type_names!($t))),
            }
        }
    };
    ($name:ident, $base:ident, $($t:tt),*) => {
        /// Borrows the specified components, ensuring that only a single entity matches the given
        /// conditions (defined by the components and filter).
        pub fn $name<F, $($t: ComponentTrait),*>(&self, filter: F) -> Result<(Entity, $(&$t),*), EcsError> where for<'r> F: FnMut(&'r (Entity, $(&$t),*)) -> bool{
            let mut components = self.$base::<F, $($t),*>(filter);

            match components.len() {
                0 => Err(EcsError::ComponentNotFound(type_names!($($t),*))),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound(type_names!($($t),*))),
            }
        }
    };
}

/// Defines a collection of `Entity`s and their components.
#[derive(Default, Debug)]
pub struct Assembly {
    base_entity: Entity,
    entities: HashMap<Entity, ComponentGroup>,
}

impl Assembly {
    /// Creates a new, empty `Assembly`.
    pub fn new() -> Self {
        Default::default()
    }
    /// Creates a new `Entity` and registers it with the `Assembly`.
    pub fn create_entity(&mut self) -> Entity {
        let e = self.base_entity.clone();
        self.base_entity.increment();
        self.entities.insert(e.clone(), ComponentGroup::new());
        e
    }
    /// Deletes the specified `Entity` from the `Assembly` and may return a `ComponentGroup`.
    pub fn destroy_entity(&mut self, entity: &Entity) -> Option<ComponentGroup> {
        self.entities.remove(entity)
    }
    /// Returns `true` if the specified `Entity` is known to the `Assembly`.
    pub fn has_entity(&self, entity: &Entity) -> bool {
        self.entities.contains_key(entity)
    }
    /// Adds a component to the group assigned to the specified `Entity`.
    pub fn add_component<C>(&mut self, entity: &Entity, component: C) -> Result<Option<C>, EcsError> where C: ComponentTrait {
        self.entities.get_mut(entity)
            .ok_or_else(|| EcsError::EntityNotFound(entity.clone()))
            .map(|g| g.insert(component))
    }
    /// Removes the component of the secified type from the group assigned to the specified `Entity`.
    pub fn remove_component<C>(&mut self, entity: &Entity) -> Result<Option<C>, EcsError> where C: ComponentTrait {
        self.entities.get_mut(entity)
            .ok_or_else(|| EcsError::EntityNotFound(entity.clone()))
            .map(|g| g.remove::<C>())
    }
    /// Checks whether the supplied `Entity` has the specified component type.
    pub fn has_component<C>(&self, entity: &Entity) -> Result<bool, EcsError> where C: ComponentTrait {
        self.entities.get(entity)
            .ok_or_else(|| EcsError::EntityNotFound(entity.clone()))
            .map(|g| g.has::<C>())
    }
    /// Borrows a single component from the specified `Entity`.
    pub fn borrow_component<C>(&self, entity: &Entity) -> Result<&C, EcsError> where C: ComponentTrait {
        self.entities.get(entity)
            .ok_or_else(|| EcsError::EntityNotFound(entity.clone()))
            .and_then(|g| g.borrow::<C>())
    }
    /// Mutably borrows a single component from the specified `Entity`.
    pub fn borrow_component_mut<C>(&mut self, entity: &Entity) -> Result<&mut C, EcsError> where C: ComponentTrait {
        self.entities.get_mut(entity)
            .ok_or_else(|| EcsError::EntityNotFound(entity.clone()))
            .and_then(|g| g.borrow_mut::<C>())
    }
    impl_read!(r1, A);
    impl_read!(r2, A, B);
    impl_read!(r3, A, B, C);
    impl_read!(r4, A, B, C, D);
    impl_read_filtered!(rf1, A);
    impl_read_filtered!(rf2, A, B);
    impl_read_filtered!(rf3, A, B, C);
    impl_read_filtered!(rf4, A, B, C, D);
    impl_read_single!(rs1, r1, A);
    impl_read_single!(rs2, r2, A, B);
    impl_read_single!(rs3, r3, A, B, C);
    impl_read_single!(rs4, r4, A, B, C, D);
    impl_read_single_filtered!(rsf1, rf1, A);
    impl_read_single_filtered!(rsf2, rf2, A, B);
    impl_read_single_filtered!(rsf3, rf3, A, B, C);
    impl_read_single_filtered!(rsf4, rf4, A, B, C, D);
    /// Provides mutable access to all instances of the specified component type.
    pub fn w1<C: ComponentTrait>(&mut self) -> Vec<(Entity, &mut C)> {
        self.entities.iter_mut()
            .filter(|&(_, ref g)| g.has::<C>())
            .map(|(e, g)| (e.clone(), g.borrow_mut::<C>().unwrap_or_else(|_| unreachable!())))
            .collect()
    }
    /// Provides mutable access to all entities' components that match the specified type and
    /// supplied filter.
    pub fn wf1<F, C: ComponentTrait>(&mut self, filter: F) -> Vec<(Entity, &mut C)> where for<'r> F: FnMut(&'r (Entity, &mut C)) -> bool {
        self.entities.iter_mut()
            .filter(|&(_, ref g)| g.has::<C>())
            .map(|(e, g)| (e.clone(), g.borrow_mut::<C>().unwrap_or_else(|_| unreachable!())))
            .filter(filter)
            .collect()
    }
    /// Ensures that only a single entity matches the bounds given by the specified component
    /// type and filter. Errors otherwise. Mutable version.
    pub fn ws1<C: ComponentTrait>(&mut self) -> Result<(Entity, &mut C), EcsError> {
        let mut components = self.w1::<C>();

        match components.len() {
            0 => Err(EcsError::ComponentNotFound(type_names!(C))),
            1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
            _ => Err(EcsError::MultipleComponentsFound(type_names!(C))),
        }
    }
    /// Ensures that only a single entity matches the bounds given by the specified component
    /// type and filter. Errors otherwise. Mutable version.
    pub fn wsf1<F, C: ComponentTrait>(&mut self, filter: F) -> Result<(Entity, &mut C), EcsError> where for<'r> F: FnMut(&'r (Entity, &mut C)) -> bool {
        let mut components = self.wf1::<F, C>(filter);

        match components.len() {
            0 => Err(EcsError::ComponentNotFound(type_names!(C))),
            1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
            _ => Err(EcsError::MultipleComponentsFound(type_names!(C))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instatiation() {
        let _a = Assembly::new();
        let _a: Assembly = Default::default();
    }

    #[test]
    fn test_create_entity() {
        let mut a = Assembly::new();

        let e = a.create_entity();
        let f = a.create_entity();

        assert!(e != f);
    }

    #[test]
    fn test_destroy_entity() {
        use entity::Entity;

        let mut a = Assembly::new();

        let e = Entity::new();
        assert!(a.destroy_entity(&e).is_none());

        let f = a.create_entity();
        assert!(a.destroy_entity(&f).is_some());
    }

    #[test]
    fn test_has_entity() {
        use entity::Entity;

        let mut a = Assembly::new();

        let e = Entity::new();
        assert!(!a.has_entity(&e));

        let f = a.create_entity();
        assert!(a.has_entity(&f));
    }
}
