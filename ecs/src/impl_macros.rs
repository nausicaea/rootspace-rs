macro_rules! impl_count {
    ($name:ident, $t:tt) => {
        /// Counts the number of entities with the specified component.
        pub fn $name<$t: ComponentTrait>(&self) -> usize {
            self.entities.values()
                .filter(|g| g.has::<$t>())
                .count()
        }
    };
    ($name:ident, $($t:tt),*) => {
        /// Counts the number of entities with the specified components.
        pub fn $name<$($t: ComponentTrait),*>(&self) -> usize {
            self.entities.values()
                .filter(|g| $(g.has::<$t>())&&*)
                .count()
        }
    };
}

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
                0 => Err(EcsError::ComponentNotFound),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound),
            }
        }
    };
    ($name:ident, $base:ident, $($t:tt),*) => {
        /// Borrows the specified components, ensuring that only a single entity matches the given
        /// conditions.
        pub fn $name<$($t: ComponentTrait),*>(&self) -> Result<(Entity, $(&$t),*), EcsError> {
            let mut components = self.$base::<$($t),*>();

            match components.len() {
                0 => Err(EcsError::ComponentNotFound),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound),
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
                0 => Err(EcsError::ComponentNotFound),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound),
            }
        }
    };
    ($name:ident, $base:ident, $($t:tt),*) => {
        /// Borrows the specified components, ensuring that only a single entity matches the given
        /// conditions (defined by the components and filter).
        pub fn $name<F, $($t: ComponentTrait),*>(&self, filter: F) -> Result<(Entity, $(&$t),*), EcsError> where for<'r> F: FnMut(&'r (Entity, $(&$t),*)) -> bool{
            let mut components = self.$base::<F, $($t),*>(filter);

            match components.len() {
                0 => Err(EcsError::ComponentNotFound),
                1 => Ok(components.pop().unwrap_or_else(|| unreachable!())),
                _ => Err(EcsError::MultipleComponentsFound),
            }
        }
    };
}

