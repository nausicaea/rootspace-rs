use std::collections::HashMap;

/// A `HalfEdge` is a container that characterises one side of an edge connecting two vertices,
/// and lying between two faces. The data structure is based on the concept of a doubly-connected
/// edge list.
#[derive(Debug, Clone)]
pub struct HalfEdge {
    /// Refers to the vertex at the position `VertexIndex` in the vertex array.
    pub vertex: VertexIndex,
    /// Refers to the next `HalfEdge` at the positon `EdgeIndex` in the edge array.
    pub next: Option<EdgeIndex>,
    /// Refers to the previous `HalfEdge` at the position `EdgeIndex` in the edge array.
    pub prev: Option<EdgeIndex>,
    /// Refers to the counterpart `HalfEdge` at the position `EdgeIndex` in the edge array.
    pub opposite: Option<EdgeIndex>,
    /// Refers to the adjacent `Face` at the position `FaceIndex` in the face array.
    pub face: Option<FaceIndex>,
}

impl HalfEdge {
    /// Creates a new, incomplete `HalfEdge` from a `VertexIndex`.
    pub fn from_vertex(vertex: VertexIndex) -> Self {
        Self {
            vertex: vertex,
            next: None,
            prev: None,
            opposite: None,
            face: None,
        }
    }
    /// Returns `true` if all `HalfEdge` properties have been assigned. Cannot check the validity
    /// of the attached indices.
    pub fn is_complete(&self) -> bool {
        self.next.is_some() && self.prev.is_some() && self.opposite.is_some() && self.face.is_some()
    }
}

/// The `Face` characterises a facet enclosed by edges and vertices.
#[derive(Debug, Clone)]
pub struct Face {
    /// Refers to the `EdgeIndex` of first `HalfEdge` in the circle of `HalfEdge`s that encircle
    /// the `Face`.
    pub first_edge: EdgeIndex,
}

impl Face {
    /// Creates a new `Face` from an `EdgeIndex`.
    pub fn from_edge(idx: EdgeIndex) -> Self {
        Self {
            first_edge: idx,
        }
    }
}

/// A `Mesh` contains a set of `Face`s that are connected to each other via `HalfEdge` pairs, each
/// of which references a single `VertexIndex` that in turn references individual vertices in an
/// external data container.
pub struct Mesh {
    /// Holds the set of `HalfEdge`s, indexed by `EdgeIndex`.
    pub edges: HashMap<EdgeIndex, HalfEdge>,
    /// Holds the set of `Face`s, indexed by `FaceIndex`.
    pub faces: HashMap<FaceIndex, Face>,
    /// Specifies the next `EdgeIndex` when a new `HalfEdge` is attached to the `Mesh`.
    next_edge_idx: EdgeIndex,
    /// Speficies the next `FaceIndex` when a new `Face` is attached to the `Mesh`.
    next_face_idx: FaceIndex,
    /// Specifies the maximum number of `HalfEdge`s per `Face`.
    max_edges_per_face: usize,
}

impl Mesh {
    /// Creates a new, empty `Mesh`.
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            faces: HashMap::new(),
            next_edge_idx: EdgeIndex(0),
            next_face_idx: FaceIndex(0),
            max_edges_per_face: 3,
        }
    }
    /// Creates a new `Mesh` from the vertices of a tetrahedron. The order of vertices is assumed
    /// to be: [apex, a, b, c], where {a, b, c} signify the base triangle of the tetrahedron in
    /// counter-clockwise windind order when seen from the apex.
    pub fn from_tetrahedron_vertices(vertices: &[VertexIndex; 4]) -> Result<Self, Error> {
        let mut mesh = Self::new();

        // Add the four triangles of the tetrahedron.
        mesh.add_triangle(vertices[0], vertices[1], vertices[2]);
        mesh.add_triangle(vertices[1], vertices[0], vertices[3]);
        mesh.add_triangle(vertices[2], vertices[3], vertices[0]);
        mesh.add_triangle(vertices[3], vertices[2], vertices[1]);

        // Connect all adjacent edges
        mesh.reconnect_adjacent()?;

        // Ensure that the mesh is closed and complete.
        mesh.verify_mesh()?;

        Ok(mesh)
    }
    /// Adds a new triangle to the `Mesh` and returns its indices.
    pub fn add_triangle(&mut self, a: VertexIndex, b: VertexIndex, c: VertexIndex) -> (FaceIndex, EdgeIndex, EdgeIndex, EdgeIndex) {
        // Create the triangle edges and single face.
        let ea_idx = self.add_free_edge(a);
        let eb_idx = self.add_free_edge(b);
        let ec_idx = self.add_free_edge(c);
        let face_idx = self.add_free_face(ea_idx);

        // Connect the edges with each other and the face
        self.edges.entry(ea_idx).and_modify(|e| {e.next = Some(eb_idx); e.prev = Some(ec_idx); e.face = Some(face_idx)});
        self.edges.entry(eb_idx).and_modify(|e| {e.next = Some(ec_idx); e.prev = Some(ea_idx); e.face = Some(face_idx)});
        self.edges.entry(ec_idx).and_modify(|e| {e.next = Some(ea_idx); e.prev = Some(eb_idx); e.face = Some(face_idx)});

        (face_idx, ea_idx, eb_idx, ec_idx)
    }
    /// Adds a free `HalfEdge` to the mesh and returns its index.
    pub fn add_free_edge(&mut self, vert: VertexIndex) -> EdgeIndex {
        let edge_idx = self.next_edge_idx;
        self.next_edge_idx.increment();
        self.edges.insert(edge_idx, HalfEdge::from_vertex(vert));
        edge_idx
    }
    /// Adds a free `Face` to the mesh and returns its index.
    pub fn add_free_face(&mut self, edge: EdgeIndex) -> FaceIndex {
        let face_idx = self.next_face_idx;
        self.next_face_idx.increment();
        self.faces.insert(face_idx, Face::from_edge(edge));
        face_idx
    }
    /// Iterates through the `HalfEdge`s of the mesh and pairs up all adjacent `HalfEdge`s.
    pub fn reconnect_adjacent(&mut self) -> Result<(), Error> {
        // The connectivity map contains a pair of vertex indices and the connecting edge index.
        let mut connectivity_map: HashMap<(VertexIndex, VertexIndex), EdgeIndex> = HashMap::new();

        // For every `HalfEdge` in the mesh, fill in the connectivity map.
        for (edge_idx, edge) in self.edges.iter() {
            if let Some(next_edge_idx) = edge.next {
                let next_edge = &self.edges[&next_edge_idx];
                connectivity_map.insert((edge.vertex, next_edge.vertex), *edge_idx);
            } else {
                return Err(Error::OpenEdgeLoop);
            }
        }

        // For every entry in the connectivity map, search for the equivalent entry in the opposite
        // direction and fill in the respective fields in each half-edge.
        for (&(ref va, ref vb), ea) in connectivity_map.iter() {
            if let Some(eb) = connectivity_map.get(&(*vb, *va)) {
                self.edges.entry(*ea).and_modify(|e| e.opposite = Some(*eb));
            } else {
                return Err(Error::UnmatchedEdge);
            }
        }

        Ok(())
    }
    pub fn iter_face_edges(&self, face_idx: &FaceIndex) -> EdgeIterator {
        let face = &self.faces[face_idx];
        EdgeIterator::new(self, face.first_edge)
    }
    /// For every face in the mesh, ensure that there is a bidirectional circle of half-edges
    /// around the face that all point to said face. Furthermore, ensure that each half-edge
    /// contains a valid opposite half-edge index.
    pub fn verify_mesh(&self) -> Result<(), Error> {
        for (face_idx, face) in self.faces.iter() {
            if let Some(first_edge) = self.edges.get(&face.first_edge) {
                // Verify the first edge (the one referenced by the face).
                if !first_edge.is_complete() {
                    return Err(Error::IncompleteEdge);
                }
                if face_idx != &first_edge.face.unwrap() {
                    return Err(Error::EdgeWithInvalidFaceReference);
                }
                if !self.edges.contains_key(&first_edge.opposite.unwrap()) {
                    return Err(Error::UnmatchedEdge);
                }
                // Verify all subsequent edges that encircle the face.
                let mut prev_edge_idx = face.first_edge;
                let mut next_edge_idx = first_edge.next.unwrap();
                let mut edge_counter = 1;
                while next_edge_idx != face.first_edge && edge_counter <= self.max_edges_per_face {
                    if let Some(current_edge) = self.edges.get(&next_edge_idx) {
                        if !current_edge.is_complete() {
                            return Err(Error::IncompleteEdge);
                        }
                        if face_idx != &current_edge.face.unwrap() {
                            return Err(Error::EdgeWithInvalidFaceReference);
                        }
                        if !self.edges.contains_key(&current_edge.opposite.unwrap()) {
                            return Err(Error::UnmatchedEdge);
                        }
                        if current_edge.prev.unwrap() != prev_edge_idx {
                            return Err(Error::EdgeWithInvalidEdgeReferences);
                        }

                        prev_edge_idx = next_edge_idx;
                        next_edge_idx = current_edge.next.unwrap();
                        edge_counter += 1;
                    } else {
                        return Err(Error::EdgeNotFound);
                    }
                }

                if first_edge.prev.unwrap() != prev_edge_idx {
                    return Err(Error::EdgeWithInvalidEdgeReferences);
                }

                if !(next_edge_idx == face.first_edge && edge_counter == self.max_edges_per_face) {
                    return Err(Error::InvalidEdgeCount);
                }
            } else {
                return Err(Error::FaceWithInvalidEdgeReference);
            }
        }

        Ok(())
    }
}

/// The `EdgeIterator` allows the iteration over edges in a circle, starting at an initial edge
/// index.
pub struct EdgeIterator<'a> {
    mesh: &'a Mesh,
    first_edge_idx: EdgeIndex,
    edge_idx: Option<EdgeIndex>,
}

impl<'a> EdgeIterator<'a> {
    /// Creates a new `EdgeIterator`.
    pub fn new(mesh: &'a Mesh, first_edge_idx: EdgeIndex) -> Self {
        Self {
            mesh: mesh,
            first_edge_idx: first_edge_idx,
            edge_idx: None,
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = &'a HalfEdge;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have returned to the first edge, stop the iteration.
        if self.edge_idx == Some(self.first_edge_idx) {
            return None;
        }

        // If this is the first time next() is called, the edge index will not have been set yet.
        if self.edge_idx.is_none() {
            self.edge_idx = Some(self.first_edge_idx);
        }

        // Retrieve the current edge and increment the index.
        let current_edge = &self.mesh.edges[&self.edge_idx.unwrap()];
        self.edge_idx = Some(current_edge.next.unwrap());

        // Return a reference to the current edge.
        Some(current_edge)
    }
}

/// An `EdgeIndex` refers to an edge in the Half-Ege mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeIndex(usize);

impl EdgeIndex {
    /// Increments the internal index value.
    pub fn increment(&mut self) {
        self.0 += 1
    }
}

/// A `FaceIndex` refers to a face in the Half-Ege mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceIndex(usize);

impl FaceIndex {
    /// Increments the internal index value.
    pub fn increment(&mut self) {
        self.0 += 1
    }
}

/// A `VertexIndex` refers to a location within the point cloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexIndex(usize);

impl From<usize> for VertexIndex {
    /// Creates a `VertexIndex` from a `usize` value.
    fn from(value: usize) -> Self {
        VertexIndex(value)
    }
}

impl Into<usize> for VertexIndex {
    /// Converts the `VertexIndex` to a `usize` value.
    fn into(self) -> usize {
        self.0
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "The convex hull mesh contains open edge loops")]
    OpenEdgeLoop,
    #[fail(display = "The requested face does not refer to a valid edge")]
    FaceWithInvalidEdgeReference,
    #[fail(display = "The requested edge was not found")]
    EdgeNotFound,
    #[fail(display = "The requested edge has unfilled parameters")]
    IncompleteEdge,
    #[fail(display = "The requested edge does not refer to a valid counterpart")]
    UnmatchedEdge,
    #[fail(display = "The requested edge does not refer to a valid face")]
    EdgeWithInvalidFaceReference,
    #[fail(display = "The requested edge does not refer to valid edges in the next and prev fields")]
    EdgeWithInvalidEdgeReferences,
    #[fail(display = "The requested face is surrounded by an invalid number of edges")]
    InvalidEdgeCount,
}
