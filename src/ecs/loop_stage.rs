pub const HANDLE_EVENT: u8 = 0b001;
pub const UPDATE: u8 = 0b010;
pub const RENDER: u8 = 0b100;
pub const ALL_STAGES: u8 = 0b111;

/// The `LoopStage` encodes all stages where `World` will issue calls to all systems. Each enum
/// variant corresponds to a method bound to the `SystemTrait`.
#[derive(Debug, Clone)]
pub enum LoopStage {
    HandleEvent,
    Update,
    Render,
}

impl LoopStage {
    /// Returns `true` if the supplied bitmask filter selects the current enum variant.
    pub fn match_filter(&self, filter: u8) -> bool {
        let value: u8 = self.clone().into();
        (value & filter) > 0
    }
}

impl From<LoopStage> for u8 {
    fn from(value: LoopStage) -> u8 {
        use self::LoopStage::*;
        match value {
            HandleEvent => HANDLE_EVENT,
            Update => UPDATE,
            Render => RENDER,
        }
    }
}

