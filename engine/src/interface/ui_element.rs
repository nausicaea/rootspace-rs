use interface::ui_primitive::UiPrimitive;

/// A `UiElement` stands for a single object in the user interface. It may be composed of one or
/// more `UiPrimitive`s.
pub struct UiElement {
    inner: Vec<UiPrimitive>,
}

impl UiElement {
    pub fn new() -> Self {
        UiElement {
            inner: Vec::new(),
        }
    }
}
