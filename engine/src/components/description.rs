//! The `description` module provides access to the `Description` component.

use ecs::ComponentTrait;

/// The `Description` component encodes information that describes a particular entity.
#[derive(Clone)]
pub struct Description {
    /// Holds the name of the connected entity or object.
    pub name: String,
}

impl Description {
    /// Creates a new `Description` component.
    pub fn new(name: &str) -> Self {
        Description {
            name: name.into(),
        }
    }
}

impl ComponentTrait for Description {}
