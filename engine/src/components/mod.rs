//! The `components` module provides access to components in the Entity-Component-System sense of
//! the word: These components are designed to hold primarily data. In principle, no behaviour
//! should be encoded within components.

pub mod description;
pub mod tooltip;
pub mod camera;
pub mod model;
pub mod material;
pub mod mesh;
pub mod ui_state;
pub mod bounding_volume;
pub mod cursor;
pub mod render_mode;
