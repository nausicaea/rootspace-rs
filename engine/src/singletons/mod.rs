use uuid::Uuid;
use ecs::Entity;
use common::factory::ComponentFactory;
use common::physics::PhysicsController;
use common::hierarchy::Hierarchy;
use components::model::Model;

#[derive(Default)]
pub struct Singletons {
    pub factory: ComponentFactory,
    pub physics: PhysicsController,
    pub scene_graph: Hierarchy<Entity, Model>,
    pub ui_hierarchy: Hierarchy<Uuid, Model>,
}
