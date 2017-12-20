use ecs::{Entity, Assembly, ComponentTrait, EcsError};

#[derive(Debug, Fail)]
pub enum GraphError {
    #[fail(display = "The internal state of the scene graph is irreparably out of sync with the assembly: {}", _0)]
    InconsistentState(#[cause] EcsError),
    #[fail(display = "The entity '{:?}' was not found.", _0)]
    EntityNotFound(Entity),
    #[fail(display = "The entity '{:?}' was found more than once.", _0)]
    MultipleEntitiesFound(Entity),
}

impl From<EcsError> for GraphError {
    fn from(value: EcsError) -> GraphError {
        GraphError::InconsistentState(value)
    }
}

/// Each `Node` contains an `Entity` and the corresponding topological component.
#[derive(Clone)]
pub struct Node<C: ComponentTrait + Clone> {
    /// The `Entity` referenced by the current `Node`.
    pub entity: Entity,
    /// The component of the above `Entity` that has a hierarchical or topological dependency.
    pub component: C,
}

impl<C: ComponentTrait + Clone> Node<C> {
    /// Creates a new `Node`.
    pub fn new(entity: Entity, component: C) -> Self {
        Node {
            entity: entity,
            component: component,
        }
    }
    /// Updates the topological component of the `Node` with respect to the local component stored
    /// in the `Assembly`, the parent `Node`'s component, and a merge closure that returns a new
    /// joint component given two input components of the same type.
    pub fn update<F>(&mut self, entities: &Assembly, parent_component: Option<&C>, merge_fn: &F) -> Result<(), GraphError> where for<'r> F: Fn(&'r C, &'r C) -> C {
        let cc = entities.borrow_component::<C>(&self.entity)?;
        self.component = match parent_component {
            Some(ref pc) => merge_fn(pc, cc),
            None => cc.clone(),
        };
        Ok(())
    }
}

/// A `Tree` is an acyclic undirected graph of `Nodes`.
pub enum Tree<C: ComponentTrait + Clone> {
    Leaf(Node<C>),
    Branch(Node<C>, Vec<Tree<C>>),
}

impl<C: ComponentTrait + Clone> Tree<C> {
    /// Creates a new `Tree` from a single root `Node`.
    pub fn new(node: Node<C>) -> Self {
        Tree::Leaf(node)
    }
    /// Recursively updates the entire `Tree` with respect to all referenced components from the
    /// `Assembly`, a closure, and an optional parent component.
    pub fn update<F>(&mut self, entities: &Assembly, parent_component: Option<&C>, merge_fn: &F) -> Result<(), GraphError> where for<'r> F: Fn(&'r C, &'r C) -> C {
        match *self {
            Tree::Leaf(ref mut node) => node.update(entities, parent_component, merge_fn),
            Tree::Branch(ref mut node, ref mut children) => {
                node.update(entities, parent_component, merge_fn)?;
                for c in children {
                    c.update(entities, Some(&node.component), merge_fn)?;
                }
                Ok(())
            },
        }
    }
    /// Obtains a reference to the component that corresponds to the supplied `Entity`.
    pub fn borrow(&self, entity: &Entity) -> Result<&C, GraphError> {
        self.depth_first(entity).map(|t| match *t {
            Tree::Leaf(Node {component: ref c, ..}) => c,
            Tree::Branch(Node {component: ref c, ..}, _) => c,
        })
    }
    /// Performs a depth-first search for the sub-`Tree` where the `Node` defined by `Entity` is
    /// the root `Node`.
    pub fn depth_first(&self, entity: &Entity) -> Result<&Tree<C>, GraphError> {
        use self::GraphError::*;
        match *self {
            Tree::Leaf(Node {entity: ref e, ..}) => {
                if entity == e {
                    Ok(self)
                } else {
                    Err(EntityNotFound(entity.clone()))
                }
            },
            Tree::Branch(Node {entity: ref e, ..}, ref children) => {
                if entity == e {
                    Ok(self)
                } else {
                    let mut sub_trees = Vec::new();
                    for c in children {
                        match c.depth_first(entity) {
                            Ok(sub_tree) => sub_trees.push(sub_tree),
                            Err(MultipleEntitiesFound(ent)) => return Err(MultipleEntitiesFound(ent)),
                            Err(_) => (),
                        }
                    }
                    match sub_trees.len() {
                        0 => Err(EntityNotFound(entity.clone())),
                        1 => Ok(sub_trees.pop().unwrap()),
                        _ => Err(MultipleEntitiesFound(entity.clone())),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Add;
    use super::*;

    #[derive(Clone, PartialEq, Eq)]
    struct TestComponent(u32);

    impl ComponentTrait for TestComponent {}

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
        let mut node = Node::<TestComponent>::new(entity.clone(), component.clone());
        assembly.add_component(&entity, component).unwrap();

        node.update(&assembly, None, &|pc, cc| pc + cc).unwrap();
    }

    #[test]
    fn test_tree() {
        let mut assembly = Assembly::new();

        let entity_a = assembly.create_entity();
        let component_a = TestComponent(10);
        assembly.add_component(&entity_a, component_a.clone()).unwrap();
        let node_a = Node::<TestComponent>::new(entity_a.clone(), component_a.clone());

        let entity_b = assembly.create_entity();
        let component_b = TestComponent(100);
        assembly.add_component(&entity_b, component_b.clone()).unwrap();
        let node_b = Node::<TestComponent>::new(entity_b.clone(), component_b.clone());

        let mut tree = Tree::Branch(node_a, vec![Tree::Leaf(node_b)]);

        tree.update(&assembly, None, &|pc, cc| pc + cc).unwrap();

        let result_a = tree.borrow(&entity_a);
        assert!(result_a.is_ok() && *result_a.unwrap() == component_a);

        let result_b = tree.borrow(&entity_b);
        assert!(result_b.is_ok() && *result_b.unwrap() == &component_a + &component_b);
    }
}
