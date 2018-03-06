pub mod factory;
pub mod physics;
pub mod hierarchy;

use ecs::Entity;
use components::model::Model;
use self::factory::ComponentFactory;
use self::physics::PhysicsController;
use self::hierarchy::Hierarchy;

#[derive(Default)]
pub struct Singletons {
    pub factory: ComponentFactory,
    pub physics: PhysicsController,
    pub hierarchy: Hierarchy<Entity, Model>,
}
