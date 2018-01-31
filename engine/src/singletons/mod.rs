pub mod factory;
pub mod physics;
pub mod scene_graph;

use ecs::Entity;
use components::model::Model;
use self::factory::ComponentFactory;
use self::physics::PhysicsController;
use self::scene_graph::SceneGraph;

#[derive(Default)]
pub struct Singletons {
    pub factory: ComponentFactory,
    pub physics: PhysicsController,
    pub scene_graph: SceneGraph<Entity, Model>,
}
