/// The `TooltipData` component allows objects to display information about themselves upon hovering
/// the mouse cursor over the object in 3-space.
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct TooltipData {
    pub text: String,
}

impl TooltipData {
    /// Creates a new `TooltipData` component.
    pub fn new(text: &str) -> Self {
        TooltipData {
            text: text.into(),
        }
    }
}
