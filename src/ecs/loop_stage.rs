use std::u8;

pub type LoopStageFlag = u8;

pub const HANDLE_EVENT: LoopStageFlag = 0b001;
pub const UPDATE: LoopStageFlag = 0b010;
pub const RENDER: LoopStageFlag = 0b100;
pub const ALL_STAGES: LoopStageFlag = u8::MAX;

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
    pub fn match_filter(&self, filter: LoopStageFlag) -> bool {
        (LoopStageFlag::from(self.clone()) & filter) > 0
    }
}

impl From<LoopStage> for LoopStageFlag {
    fn from(value: LoopStage) -> LoopStageFlag {
        use self::LoopStage::*;
        match value {
            HandleEvent => HANDLE_EVENT,
            Update => UPDATE,
            Render => RENDER,
        }
    }
}

