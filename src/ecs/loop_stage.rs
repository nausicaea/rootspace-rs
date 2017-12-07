use std::u8;

bitflags! {
    pub struct LoopStageFlag: u8 {
        const HANDLE_EVENT = 0b001;
        const UPDATE = 0b010;
        const RENDER = 0b100;
        const ALL_STAGES = u8::MAX;
    }
}

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
        filter.contains(LoopStageFlag::from(self.clone()))
    }
}

impl From<LoopStage> for LoopStageFlag {
    fn from(value: LoopStage) -> LoopStageFlag {
        use self::LoopStage::*;
        match value {
            HandleEvent => LoopStageFlag::HANDLE_EVENT,
            Update => LoopStageFlag::UPDATE,
            Render => LoopStageFlag::RENDER,
        }
    }
}

