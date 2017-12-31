use std::collections::HashMap;
use daggy::{Dag, NodeIndex, Walker};
use daggy::petgraph::graph::{Node, DefaultIx};
use ecs::{Entity, Assembly, ComponentTrait, EcsError};

#[derive(Debug, Fail)]
pub enum GraphError {
    #[fail(display = "The internal state of the scene graph is irreparably out of sync with the assembly: {}", _0)]
    InconsistentState(#[cause] EcsError),
    #[fail(display = "The entity '{:?}' was not found.", _0)]
    EntityNotFound(Entity),
    #[fail(display = "The entity '{:?}' was found more than once.", _0)]
    MultipleEntitiesFound(Entity),
    #[fail(display = "The root node may not be removed.")]
    CannotRemoveRootNode,
    #[fail(display = "The specified node was not found.")]
    NodeNotFound,
}

impl From<EcsError> for GraphError {
    fn from(value: EcsError) -> GraphError {
        GraphError::InconsistentState(value)
    }
}

/// Each `SceneNode` contains an `Entity` and the corresponding topological component.
#[derive(Clone)]
pub struct SceneNode<C: ComponentTrait + Clone> {
    /// The `Entity` referenced by the current `SceneNode`.
    pub entity: Entity,
    /// The component of the above `Entity` that has a hierarchical or topological dependency.
    pub component: C,
}

impl<C: ComponentTrait + Clone> SceneNode<C> {
    /// Creates a new `SceneNode`.
    pub fn new(entity: Entity, component: C) -> Self {
        SceneNode {
            entity: entity,
            component: component,
        }
    }
    /// Updates the topological component of the `SceneNode` with respect to the local component stored
    /// in the `Assembly`, the parent `SceneNode`'s component, and a merge closure that returns a new
    /// joint component given two input components of the same type. More specifically, the
    /// `merge_fn` parameter requires a closure that receives a reference to the parent node's
    /// topological component and a reference to the current node's local component, and must
    /// provide a new topological component for the current node.
    pub fn update<F>(&mut self, entities: &Assembly, parent_component: Option<&C>, merge_fn: &F) -> Result<(), GraphError> where for<'r> F: Fn(&'r C, &'r C) -> C {
        let cc = entities.borrow_component::<C>(&self.entity)?;
        self.component = match parent_component {
            Some(pc) => merge_fn(pc, cc),
            None => cc.clone(),
        };
        Ok(())
    }
}

/// Provides the ability to iterate over all `SceneNode`s stored within a `SceneGraph`.
pub struct GraphIter<'a, C: ComponentTrait + Clone> {
    index: usize,
    data: &'a [Node<SceneNode<C>, DefaultIx>],
}

impl<'a, C: ComponentTrait + Clone> GraphIter<'a, C> {
    /// Creates a new `SceneGraph`.
    pub fn new(data: &'a [Node<SceneNode<C>, DefaultIx>]) -> Self {
        GraphIter {
            index: 0,
            data: data,
        }
    }
}

impl<'a, C: ComponentTrait + Clone> Iterator for GraphIter<'a, C> {
    type Item = &'a SceneNode<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let idx = self.index;
            self.index += 1;
            Some(&self.data[idx].weight)
        } else {
            None
        }
    }
}

pub struct SceneGraph<C: ComponentTrait + Clone> {
    root_entity: Entity,
    index: HashMap<Entity, NodeIndex>,
    graph: Dag<SceneNode<C>, ()>,
}

impl<C: ComponentTrait + Clone> SceneGraph<C> {
    /// Creates a new `SceneGraph`.
    pub fn new(root_node: SceneNode<C>) -> Self {
        let root_entity = root_node.entity.clone();

        let mut dag = Dag::new();
        let root_idx = dag.add_node(root_node);

        let mut index = HashMap::new();
        index.insert(root_entity.clone(), root_idx);

        SceneGraph {
            root_entity: root_entity,
            index: index,
            graph: dag,
        }
    }
    /// Deletes the `SceneNode` defined by the specified `Entity`.
    pub fn remove(&mut self, entity: &Entity) -> Result<(), GraphError> {
        if entity == &self.root_entity {
            return Err(GraphError::CannotRemoveRootNode);
        }

        let node_idx = self.get_index(entity)?;
        self.graph.remove_node(node_idx);
        self.rebuild_index();
        Ok(())
    }
    /// Inserts a `SceneNode` as child of the root `SceneNode`.
    pub fn insert(&mut self, node: SceneNode<C>) -> Result<(), GraphError> {
        let ent = self.root_entity.clone();
        self.insert_child(&ent, node)
    }
    /// Inserts a `SceneNode` as child of another `SceneNode` defined by an `Entity`.
    pub fn insert_child(&mut self, entity: &Entity, node: SceneNode<C>) -> Result<(), GraphError> {
        let node_idx = self.get_index(entity)?;
        let new_entity = node.entity.clone();
        let (_, nidx) = self.graph.add_child(node_idx, (), node);
        self.index.insert(new_entity, nidx);
        Ok(())
    }
    /// Returns `true` if the specified `Entity` is represented within the `SceneGraph`.
    pub fn has(&self, entity: &Entity) -> bool {
        self.get_index(entity).is_ok()
    }
    /// Recursively updates the entire `SceneGraph` given the `Assembly` and a component merger
    /// closure. Refer to `SceneNode.update` for more information.
    pub fn update<F>(&mut self, entities: &Assembly, merge_fn: &F) -> Result<(), GraphError> where for<'r> F: Fn(&'r C, &'r C) -> C {
        // Obtain the index of the root node.
        let root_idx = self.get_index(&self.root_entity)?;

        // Update the root node.
        update_single(&mut self.graph, root_idx, None, entities, merge_fn)?;

        // Recursively update all children.
        update_recursive(&mut self.graph, root_idx, entities, merge_fn)
    }
    /// Borrows the component from the `SceneNode` defined by the specified `Entity`.
    pub fn borrow(&self, entity: &Entity) -> Result<&C, GraphError> {
        let node_idx = self.get_index(entity)?;
        self.graph.node_weight(node_idx)
            .map(|n| &n.component)
            .ok_or_else(|| GraphError::EntityNotFound(entity.clone()))
    }
    /// Mutably borrows the component from the `SceneNode` defined by the specified `Entity`.
    pub fn borrow_mut(&mut self, entity: &Entity) -> Result<&mut C, GraphError> {
        let node_idx = self.get_index(entity)?;
        self.graph.node_weight_mut(node_idx)
            .map(|n| &mut n.component)
            .ok_or_else(|| GraphError::EntityNotFound(entity.clone()))
    }
    /// Returns an iterator over all `SceneNode`s in the `SceneGraph`.
    pub fn iter(&self) -> GraphIter<C> {
        GraphIter::new(self.graph.raw_nodes())
    }
    /// Returns the `NodeIndex` for a particular `Entity`.
    fn get_index(&self, entity: &Entity) -> Result<NodeIndex, GraphError> {
        self.index.get(entity)
            .cloned()
            .ok_or_else(|| GraphError::EntityNotFound(entity.clone()))
    }
    /// Rebuilds the `Entity`-`SceneNode` index from the underlying `Graph`.
    fn rebuild_index(&mut self) {
        self.index.clear();
        for idx in self.graph.graph().node_indices() {
            let node = self.graph.node_weight(idx).unwrap_or_else(|| unreachable!());
            self.index.insert(node.entity.clone(), idx);
        }
    }
}

fn update_single<C, F>(graph: &mut Dag<SceneNode<C>, ()>, node_idx: NodeIndex, parent: Option<&C>, entities: &Assembly, merge_fn: &F) -> Result<(), GraphError> where C: ComponentTrait + Clone, for<'r> F: Fn(&'r C, &'r C) -> C {
    // Obtain a mutable reference to the current node.
    graph.node_weight_mut(node_idx)
        .ok_or(GraphError::NodeNotFound)
        .and_then(|n| n.update(entities, parent, merge_fn))
}

fn update_recursive<C, F>(graph: &mut Dag<SceneNode<C>, ()>, parent_idx: NodeIndex, entities: &Assembly, merge_fn: &F) -> Result<(), GraphError> where C: ComponentTrait + Clone, for<'r> F: Fn(&'r C, &'r C) -> C {
    // Obtain a reference to the parent component.
    let parent = graph.node_weight(parent_idx)
        .map(|n| n.component.clone())
        .ok_or(GraphError::NodeNotFound)?;

    // Update all children of the current node.
    let mut child_walker = graph.children(parent_idx);
    while let Some(idx) = child_walker.next_node(graph) {
        // Update each child.
        update_single(graph, idx, Some(&parent), entities, merge_fn)?;

        // Update each child's children.
        update_recursive(graph, idx, entities, merge_fn)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::ops::Add;
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestComponent(u32);

    impl ComponentTrait for TestComponent {}

    impl<'a> Add<&'a TestComponent> for TestComponent {
        type Output = TestComponent;

        fn add(self, other: &'a TestComponent) -> Self::Output {
            TestComponent(self.0 + other.0)
        }
    }

    impl<'a> Add<Self> for &'a TestComponent {
        type Output = TestComponent;

        fn add(self, other: Self) -> Self::Output {
            TestComponent(self.0 + other.0)
        }
    }

    #[test]
    fn test_scene_node() {
        let mut assembly = Assembly::new();

        let entity = assembly.create_entity();
        let component = TestComponent(10);
        let mut node = SceneNode::<TestComponent>::new(entity.clone(), component.clone());
        assembly.add_component(&entity, component).unwrap();

        node.update(&assembly, None, &|pc, cc| pc + cc).unwrap();
    }

    #[test]
    fn test_scene_graph() {
        let mut assembly = Assembly::new();

        let entity_a = assembly.create_entity();
        let component_a = TestComponent(1);
        assembly.add_component(&entity_a, component_a.clone()).unwrap();
        let node_a = SceneNode::<TestComponent>::new(entity_a.clone(), component_a.clone());

        let entity_b = assembly.create_entity();
        let component_b = TestComponent(10);
        assembly.add_component(&entity_b, component_b.clone()).unwrap();
        let node_b = SceneNode::<TestComponent>::new(entity_b.clone(), component_b.clone());

        let entity_c = assembly.create_entity();
        let component_c = TestComponent(20);
        assembly.add_component(&entity_c, component_c.clone()).unwrap();
        let node_c = SceneNode::<TestComponent>::new(entity_c.clone(), component_c.clone());

        let entity_d = assembly.create_entity();
        let component_d = TestComponent(100);
        assembly.add_component(&entity_d, component_d.clone()).unwrap();
        let node_d = SceneNode::<TestComponent>::new(entity_d.clone(), component_d.clone());

        let mut tree = SceneGraph::new(node_a);

        tree.insert_child(&entity_a, node_b).unwrap();
        tree.insert_child(&entity_a, node_c).unwrap();
        tree.insert_child(&entity_b, node_d).unwrap();

        tree.update(&assembly, &|pc, cc| pc + cc).unwrap();

        let result_a = tree.borrow(&entity_a);
        assert!(result_a.is_ok() && *result_a.unwrap() == component_a);

        let result_b = tree.borrow(&entity_b);
        assert!(result_b.is_ok() && *result_b.unwrap() == &component_a + &component_b);

        let result_c = tree.borrow(&entity_c);
        assert!(result_c.is_ok() && *result_c.unwrap() == &component_a + &component_c);

        let result_d = tree.borrow(&entity_d);
        assert!(result_d.is_ok() && *result_d.unwrap() == &component_a + &component_b + &component_d);
    }
}
