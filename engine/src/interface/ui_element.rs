use geometry::model::Model;
use interface::ui_primitive::UiPrimitive;

/// A `UiElement` stands for a single object in the user interface. It may be composed of one or
/// more `UiPrimitive`s.
pub struct UiElement {
    pub model: Model,
    pub primitives: Vec<UiPrimitive>,
}

impl UiElement {
    pub fn new(model: Model, primitives: Vec<UiPrimitive>) -> Self {
        UiElement {
            model: model,
            primitives: primitives,
        }
    }
}
