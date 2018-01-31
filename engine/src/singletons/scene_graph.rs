use std::collections::HashMap;
use std::hash::Hash;
use daggy::{Dag, NodeIndex, Walker};
use daggy::petgraph::graph::{Node, DefaultIx};

/// Given a set of identifying keys and corresponding data, `SceneGraph` allows users to establish
/// hierarchical relationships between individual instances of the data type.
pub struct SceneGraph<K: Clone + Default + Eq + Hash, V: Clone + Default> {
    /// Holds the key of the root node.
    root_key: K,
    /// Provides an indexing relationship between keys and `NodeIndex` instances that in turn index
    /// into the directed acyclic graph (`Dag`).
    index: HashMap<K, NodeIndex>,
    /// Holds the directed acyclic graph of `SceneNode`s.
    graph: Dag<SceneNode<K, V>, ()>,
}

impl<K: Clone + Default + Eq + Hash, V: Clone + Default> Default for SceneGraph<K, V> {
    /// Creates a default `SceneGraph` with just a root node.
    fn default() -> Self {
        let root_node: SceneNode<K, V> = Default::default();
        let root_key = root_node.key.clone();

        let mut dag = Dag::new();
        let root_idx = dag.add_node(root_node);

        let mut index = HashMap::new();
        index.insert(root_key.clone(), root_idx);

        SceneGraph {
            root_key: root_key,
            index: index,
            graph: dag,
        }
    }
}

impl<K: Clone + Default + Eq + Hash, V: Clone + Default> SceneGraph<K, V> {
    /// Creates a new `SceneGraph`.
    pub fn new() -> Self {
        Default::default()
    }
    /// Deletes the `SceneNode` defined by the specified key.
    pub fn remove(&mut self, key: &K) -> Result<(), GraphError> {
        if key == &self.root_key {
            return Err(GraphError::CannotRemoveRootNode);
        }

        let node_idx = self.get_index(key)?;
        self.graph.remove_node(node_idx);
        self.rebuild_index();
        Ok(())
    }
    /// Inserts a `SceneNode` as child of the root `SceneNode`.
    pub fn insert(&mut self, child: K, data: V) -> Result<(), GraphError> {
        let parent = self.root_key.clone();
        self.insert_child(&parent, child, data)
    }
    /// Inserts a `SceneNode` as child of another `SceneNode` identified by its key.
    pub fn insert_child(&mut self, parent: &K, child: K, data: V) -> Result<(), GraphError> {
        let parent_idx = self.get_index(parent)?;
        let child_node = SceneNode::new(child.clone(), data);
        let (_, child_idx) = self.graph.add_child(parent_idx, (), child_node);
        self.index.insert(child, child_idx);
        Ok(())
    }
    /// Returns `true` if the specified key is represented within the `SceneGraph`.
    pub fn has(&self, key: &K) -> bool {
        self.index.contains_key(key)
    }
    /// Recursively updates the entire `SceneGraph`. Refer to `SceneNode.update` for more
    /// information.
    pub fn update<F>(&mut self, merge_fn: &F) -> Result<(), GraphError>
            where for<'r> F: Fn(&'r K, &'r V) -> V {

        // Obtain the index of the root node.
        let root_idx = self.get_index(&self.root_key)?;

        // Recursively update all children of the root node. Never update the root node itself,
        // because it's invisible to the outside and should not be represented in the assembly.
        update_recursive(&mut self.graph, root_idx, merge_fn)
    }
    /// Borrows the data from the `SceneNode` identified by the specified key.
    pub fn borrow(&self, key: &K) -> Result<&V, GraphError> {
        let node_idx = self.get_index(key)?;
        self.graph.node_weight(node_idx)
            .map(|n| &n.data)
            .ok_or(GraphError::KeyNotFound)
    }
    /// Mutably borrows the data from the `SceneNode` defined by the specified key.
    pub fn borrow_mut(&mut self, key: &K) -> Result<&mut V, GraphError> {
        let node_idx = self.get_index(key)?;
        self.graph.node_weight_mut(node_idx)
            .map(|n| &mut n.data)
            .ok_or(GraphError::KeyNotFound)
    }
    /// Returns an iterator over all `SceneNode`s in the `SceneGraph`.
    pub fn iter(&self) -> GraphIter<K, V> {
        GraphIter::new(self.graph.raw_nodes())
    }
    /// Returns the `NodeIndex` for a particular key.
    fn get_index(&self, key: &K) -> Result<NodeIndex, GraphError> {
        self.index.get(key)
            .cloned()
            .ok_or(GraphError::KeyNotFound)
    }
    /// Rebuilds the `Key`-`SceneNode` index from the underlying `Graph`.
    fn rebuild_index(&mut self) {
        self.index.clear();
        for idx in self.graph.graph().node_indices() {
            let node = self.graph.node_weight(idx).unwrap_or_else(|| unreachable!());
            self.index.insert(node.key.clone(), idx);
        }
    }
}

/// Each `SceneNode` consists of an identifying key and the associated data.
#[derive(Default, Clone)]
pub struct SceneNode<K, V: Clone + Default> {
    /// Provides access to the identifying key.
    pub key: K,
    /// Provides access to the hierarchical data.
    pub data: V,
}

impl<K, V: Clone + Default> SceneNode<K, V> {
    /// Creates a new `SceneNode`.
    pub fn new(key: K, data: V) -> Self {
        SceneNode {
            key: key,
            data: data,
        }
    }
    /// Given the parent node's data, update the current node's data with the supplied closure.
    /// This allows users to establish hierarchical relationships between instances of a type.
    /// As arguments, the closure will receive the current node's key and a reference to its parent
    /// node's data.
    pub fn update<F>(&mut self, parent_data: &V, merge_fn: &F) where for<'r> F: Fn(&'r K, &'r V) -> V {
        self.data = merge_fn(&self.key, parent_data)
    }
}

/// Provides the ability to iterate over all `SceneNode`s stored within a `SceneGraph`.
pub struct GraphIter<'a, K: 'a, V: 'a + Clone + Default> {
    index: usize,
    data: &'a [Node<SceneNode<K, V>, DefaultIx>],
}

impl<'a, K: 'a, V: 'a + Clone + Default> GraphIter<'a, K, V> {
    /// Creates a new `SceneGraph`.
    pub fn new(data: &'a [Node<SceneNode<K, V>, DefaultIx>]) -> Self {
        GraphIter {
            index: 0,
            data: data,
        }
    }
}

impl<'a, K: 'a, V: 'a + Clone + Default> Iterator for GraphIter<'a, K, V> {
    type Item = &'a SceneNode<K, V>;

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

fn update_single<K, V, F>(graph: &mut Dag<SceneNode<K, V>, ()>, node_idx: NodeIndex, parent_data: &V, merge_fn: &F) -> Result<(), GraphError>
        where V: Clone + Default,
        for<'r> F: Fn(&'r K, &'r V) -> V {

    // Obtain a mutable reference to the current node.
    graph.node_weight_mut(node_idx)
        .ok_or(GraphError::NodeNotFound)
        .map(|n| n.update(parent_data, merge_fn))
}

fn update_recursive<K, V, F>(graph: &mut Dag<SceneNode<K, V>, ()>, parent_idx: NodeIndex, merge_fn: &F) -> Result<(), GraphError>
        where V: Clone + Default,
        for<'r> F: Fn(&'r K, &'r V) -> V {

    // Obtain a reference to the parent data.
    let parent_data = graph.node_weight(parent_idx)
        .map(|n| n.data.clone())
        .ok_or(GraphError::NodeNotFound)?;

    // Update all children of the current node.
    let mut child_walker = graph.children(parent_idx);
    while let Some(idx) = child_walker.next_node(graph) {
        // Update each child.
        update_single(graph, idx, &parent_data, merge_fn)?;

        // Update each child's children.
        update_recursive(graph, idx, merge_fn)?;
    }

    Ok(())
}

#[derive(Debug, Fail)]
pub enum GraphError {
    #[fail(display = "The key was not found.")]
    KeyNotFound,
    #[fail(display = "The key was found more than once.")]
    MultipleKeysFound,
    #[fail(display = "The root node may not be removed.")]
    CannotRemoveRootNode,
    #[fail(display = "The specified node was not found.")]
    NodeNotFound,
}
