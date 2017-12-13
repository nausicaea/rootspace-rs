use std::fmt;
use std::fmt::Display;

/// An `Entity` is nothing more than a unique identifier that stands for an object in the `World`.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    /// Creates a new, initial `Entity`.
    pub fn new() -> Self {
        Default::default()
    }
    /// Increments the internal ID.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entity({})", self.0)
    }
}