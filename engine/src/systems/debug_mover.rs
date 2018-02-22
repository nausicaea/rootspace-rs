use std::time::Duration;
use nalgebra::{zero, Vector3};
use ecs::{Assembly, DispatchEvents, Entity, LoopStageFlag, SystemTrait};
use event::EngineEvent;
use singletons::Singletons;
use components::description::Description;
use components::model::Model;

/// Continuously moves the target entity on a circle in 3-space.
pub struct DebugMover {
    pub target_name: String,
    pub target: Option<Entity>,
    pub target_position: Vector3<f32>,
}

impl DebugMover {
    /// Creates a new `DebugMover` system.
    pub fn new(target_name: &str) -> Self {
        DebugMover {
            target_name: target_name.into(),
            target: None,
            target_position: zero(),
        }
    }
}

impl SystemTrait<EngineEvent, Singletons> for DebugMover {
    /// `DebugMover` has no requirements.
    fn verify_requirements(&self, _: &Assembly) -> bool {
        true
    }
    /// `DebugMover` registers only the update call.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::UPDATE
    }
    fn update(
        &mut self,
        entities: &mut Assembly,
        _: &mut Singletons,
        time: &Duration,
        _: &Duration,
    ) -> DispatchEvents<EngineEvent> {
        if self.target.is_none() {
            let (target, target_position) = entities
                .rsf2::<_, Description, Model>(|&(_, d, _)| d.name == self.target_name)
                .map(|(e, _, m)| (Some(e), *m.translation()))
                .unwrap();

            self.target = target;
            self.target_position = target_position;
        }

        if let Some(ref target) = self.target {
            entities
                .borrow_component_mut::<Model>(target)
                .map(|m| {
                    let r = 1.0;
                    let w = 1.0;
                    let t = time.as_secs() as f32 + time.subsec_nanos() as f32 * 1e-9;
                    let translation = self.target_position
                        + Vector3::new(0.0, r * (w * t).cos(), r * (w * t).sin());
                    m.set_translation(translation);
                })
                .unwrap();
        }
        (None, None)
    }
}
