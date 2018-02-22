use std::u8;

bitflags! {
    pub struct LoopStageFlag: u8 {
        const HANDLE_EVENT = 0b0001;
        const UPDATE = 0b0010;
        const DYNAMIC_UPDATE = 0b0100;
        const RENDER = 0b1000;
        const ALL_STAGES = u8::MAX;
    }
}

/// The `LoopStage` encodes all stages where `World` will issue calls to all systems. Each enum
/// variant corresponds to a method bound to the `SystemTrait`.
#[derive(Debug, Clone)]
pub enum LoopStage {
    HandleEvent,
    Update,
    DynamicUpdate,
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
            DynamicUpdate => LoopStageFlag::DYNAMIC_UPDATE,
            Render => LoopStageFlag::RENDER,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let lsf: LoopStageFlag = LoopStage::HandleEvent.into();
        assert!(lsf == LoopStageFlag::HANDLE_EVENT);
        let lsf: LoopStageFlag = LoopStage::Update.into();
        assert!(lsf == LoopStageFlag::UPDATE);
        let lsf: LoopStageFlag = LoopStage::DynamicUpdate.into();
        assert!(lsf == LoopStageFlag::DYNAMIC_UPDATE);
        let lsf: LoopStageFlag = LoopStage::Render.into();
        assert!(lsf == LoopStageFlag::RENDER);
    }

    #[test]
    fn test_match_filter() {
        assert!(LoopStage::HandleEvent.match_filter(LoopStageFlag::HANDLE_EVENT));
        assert!(LoopStage::Update.match_filter(LoopStageFlag::UPDATE));
        assert!(LoopStage::DynamicUpdate.match_filter(LoopStageFlag::DYNAMIC_UPDATE));
        assert!(LoopStage::Render.match_filter(LoopStageFlag::RENDER));

        for ls in &[
            LoopStage::HandleEvent,
            LoopStage::Update,
            LoopStage::DynamicUpdate,
            LoopStage::Render,
        ] {
            assert!(ls.match_filter(LoopStageFlag::ALL_STAGES));
        }
    }
}
