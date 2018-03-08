//! The `ui_state` provides access to the `UiState` component.

use std::collections::HashMap;
use uuid::Uuid;
use glium::Display;
use common::resource_group::{FontCacheGroup, ResourceError};
use common::ui_element::UiElement;
use common::ui_primitive::UiPrimitive;
use common::ui_styles::{SpeechBubble, Tooltip};

/// The `UiState` component encodes information about the user interface.
#[derive(Component)]
pub struct UiState {
    /// Holds all user interface elements, so-called `UiElement`s, indexed by a `Uuid`.
    pub elements: HashMap<Uuid, UiElement>,
    /// Provides access to the font fache.
    pub font_cache: FontCacheGroup,
    /// Provides access to speech-bubble style settings.
    pub speech_bubble: SpeechBubble,
    /// Provides access to tooltip style settings.
    pub tooltip: Tooltip,
    /// If set to `true`, a menu item currently shadows the 3D world.
    pub menu_active: bool,
}

impl UiState {
    /// Creates a new `UiState` component.
    pub fn new(
        display: &Display,
        dimensions: &[u32; 2],
        hi_dpi_factor: f32,
        speech_bubble: SpeechBubble,
        tooltip: Tooltip,
    ) -> Result<Self, ResourceError> {
        let cache = FontCacheGroup::new(display, dimensions, hi_dpi_factor as u32)?;

        Ok(UiState {
            elements: Default::default(),
            font_cache: cache,
            speech_bubble: speech_bubble,
            tooltip: tooltip,
            menu_active: false,
        })
    }
}
