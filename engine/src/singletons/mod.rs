pub mod factory;
pub mod physics;

use self::factory::ComponentFactory;
use self::physics::PhysicsController;

#[derive(Default)]
pub struct Singletons {
    pub factory: ComponentFactory,
    pub physics: PhysicsController,
}
