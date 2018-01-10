use num_traits::float::Float;
use nalgebra::{zero, Vector3, Unit, Scalar, Real};
use common::half_edge_mesh::{Mesh, VertexIndex};
use common::half_edge_mesh::Error as MError;

/// Generates a convex hull given a set of points in 3-space.
fn convex_hull<N>(vertices: &[Vector3<N>]) -> Result<(), Error> where N: Scalar + Real + Float {
    // If the point cloud contains less than four vertices, a convex hull cannot be created.
    if vertices.len() < 4 {
        return Err(Error::TooFewPoints);
    }

    // Apply the Akl-Toussain heuristic which allows to greatly reduce the search space of the
    // problem.
    let tetrahedron = akl_toussain_heuristic(&vertices)?;

    // Generate the initial tetrahedron mesh.
    let mut mesh = Mesh::from_tetrahedron_vertices(&tetrahedron)?;

    // Create an exclusion vector that contains all indices that are to be excluded from the
    // search.
    let mut excluded_indices = tetrahedron.to_vec();

    // Based on the Akl-Toussain heuristic, also exclude all points that lie within the
    // tetrahedron, as they will never be part of the convex hull anyway.

    Ok(())
}

/// Constructs a tetrahedron from the most extreme points within the point cloud, which allows us
/// to exclude a large amount of points that would not be in the convex hull anyway. On success,
/// this function will return an array of four indices that describe a tetrahedron. The first index
/// corresponds to the apex and the three subsequent indices form the base of the tetrahedron in
/// counter-clockwise order when seen from the apex.
fn akl_toussain_heuristic<N>(vertices: &[Vector3<N>]) -> Result<[VertexIndex; 4], Error> where N: Scalar + Real + Float {
    // Find the two extreme points along each axis of the point cloud.
    let init = [[IndexedPoint::infinity(),
                IndexedPoint::neg_infinity()],
                [IndexedPoint::infinity(),
                IndexedPoint::neg_infinity()],
                [IndexedPoint::infinity(),
                IndexedPoint::neg_infinity()]];
    let extremes = vertices.iter().enumerate().fold(init, |state, (i, p)| {
        let mut next_state = state.clone();

        if p.x < state[0][0].value.x {
            next_state[0][0] = IndexedPoint::new(Some(i), *p);
        } else if p.x > state[0][1].value.x {
            next_state[0][1] = IndexedPoint::new(Some(i), *p);
        }
        if p.y < state[1][0].value.y {
            next_state[1][0] = IndexedPoint::new(Some(i), *p);
        } else if p.y > state[1][1].value.y {
            next_state[1][1] = IndexedPoint::new(Some(i), *p);
        }
        if p.z < state[2][0].value.z {
            next_state[2][0] = IndexedPoint::new(Some(i), *p);
        } else if p.z > state[2][1].value.z {
            next_state[2][1] = IndexedPoint::new(Some(i), *p);
        }

        next_state
    });

    // If any of the extreme values still contain infinities, the mesh must be degenerate.
    if extremes.iter().flat_map(|epair| epair.iter()).any(|ip| !ip.is_valid()) {
        return Err(Error::DegenerateMesh);
    }

    // Determine the most distant point pair of the extremes.
    let init: (IndexedPoint<N>, IndexedPoint<N>, N) = (IndexedPoint::zero(), IndexedPoint::zero(), zero());
    let (a, b, _) = extremes.iter().fold(init, |state, pair| {
        let dist_sq = dist_sq(&pair[0].value, &pair[1].value);
        if dist_sq > state.2 {
            (pair[0].clone(), pair[1].clone(), dist_sq)
        } else {
            state
        }
    });

    // If the most distant points are at the origin, we have a degenerate mesh.
    if !(a.is_valid() && b.is_valid()) {
        return Err(Error::DegenerateMesh);
    }

    // Determine the point `c` that is most distant from the line segment `ab`. Together, they form
    // the initial triangle.
    let init: (IndexedPoint<N>, N) = (IndexedPoint::zero(), zero());
    let (c, _) = extremes.iter().flat_map(|e| e.iter()).fold(init, |state, p| {
        let dist_sq = dist_sq_segment(&a.value, &b.value, &p.value);
        if dist_sq > state.1 {
            (p.clone(), dist_sq)
        } else {
            state
        }
    });

    // If the third point could not be found, all points in the mesh are collinear.
    if !c.is_valid() {
        return Err(Error::CollinearMesh);
    }

    // Determine the point `d` that is most distant from the triangle `abc`.
    let abc_normal = normal(&a.value, &b.value, &c.value).ok_or(Error::DegenerateTriangle)?;
    let aux_dot = abc_normal.dot(&a.value);
    let init: (IndexedPoint<N>, N) = (IndexedPoint::zero(), zero());
    let (d, _) = vertices.iter().enumerate().fold(init, |state, (i, p)| {
        let t = aux_dot - abc_normal.dot(p);
        if t > state.1 {
            (IndexedPoint::new(Some(i), *p), t)
        } else {
            state
        }
    });

    // If the fourth point completing the simplex could not be found, the mesh is coplanar.
    if !d.is_valid() {
        return Err(Error::CoplanarMesh);
    }

    // Return the indices of the initial tetrahedron. The use of unwrap is OK here, because each
    // point was verified beforehand. Furthermore, we have to reorder the points based on the
    // winding order of the base with respect to the apex `d`.
    if abc_normal.dot(&((d.value - a.value).normalize())) > N::default_epsilon() {
        Ok([d.idx.unwrap(), a.idx.unwrap(), b.idx.unwrap(), c.idx.unwrap()])
    } else {
        Ok([d.idx.unwrap(), b.idx.unwrap(), a.idx.unwrap(), c.idx.unwrap()])
    }
}

/// Calculates the squared euclidean distance of two vectors.
fn dist_sq<N>(a: &Vector3<N>, b: &Vector3<N>) -> N where N: Scalar + Real {
    let ab = b - a;
    ab.dot(&ab)
}

/// Calculates the minimum distance of a point `p` to a line segment `ab`.
fn dist_sq_segment<N>(a: &Vector3<N>, b: &Vector3<N>, p: &Vector3<N>) -> N where N: Scalar + Real {
    let l2_sq = dist_sq(a, b);

    if l2_sq <= N::default_epsilon() {
        return dist_sq(a, p);
    }

    let ab_n = (b - a).normalize();
    let ap = p - a;
    let d = ap - ab_n * ap.dot(&ab_n);

    d.dot(&d)
}

/// Calculates the incentre of a triangle.
fn incentre<N>(a: &Vector3<N>, b: &Vector3<N>, c: &Vector3<N>) -> Vector3<N> where N: Scalar + Real {
    let a_norm = a.norm();
    let b_norm = b.norm();
    let c_norm = c.norm();
    let perimeter = a_norm + b_norm + c_norm;

    a * (a_norm / perimeter) + b * (b_norm / perimeter) + c * (c_norm / perimeter)
}

/// Calculates the normal of the specified triangle. Will fail if the triangle is degenerate.
fn normal<N>(a: &Vector3<N>, b: &Vector3<N>, c: &Vector3<N>) -> Option<Unit<Vector3<N>>> where N: Scalar + Real {
    let ab = b - a;
    let ac = c - a;
    let n = ab.cross(&ac);

    Unit::try_new(n, N::default_epsilon())
}

/// Returns `true` if the winding order of the specified triangle is counter-clockwise.
fn ccw<N>(p: &Vector3<N>, q: &Vector3<N>, r: &Vector3<N>) -> bool where N: Scalar + Real {
    (q.x - p.x) * (r.y - p.y) > (r.x - p.x) * (q.y - p.y)
}

/// An `IndexedPoint` stores both the value of the point as `Vector3` and the index into the
/// originating point cloud. `IndexedPoint` makes code cleaner, where we could just use tuples.
/// Furthermore, the lack of `Copy` makes it clearer when data is copied.
#[derive(Debug, Clone)]
struct IndexedPoint<N> where N: Scalar + Real + Float {
    pub idx: Option<VertexIndex>,
    pub value: Vector3<N>,
}

impl<N> IndexedPoint<N> where N: Scalar + Real + Float {
    /// Creates a new `IndexedPoint` with a valid index.
    pub fn new(idx: Option<usize>, value: Vector3<N>) -> Self {
        IndexedPoint {
            idx: idx.map(|i| From::from(i)),
            value: value,
        }
    }
    /// Creates a new, zero-value `IndexedPoint`.
    pub fn zero() -> Self {
        IndexedPoint {
            idx: None,
            value: zero(),
        }
    }
    /// Creates a new `IndexedPoint` with positive infinite value.
    pub fn infinity() -> Self {
        IndexedPoint {
            idx: None,
            value: Vector3::new(N::infinity(), N::infinity(), N::infinity()),
        }
    }
    /// Creates a new `IndexedPoint` with negative infinite value.
    pub fn neg_infinity() -> Self {
        IndexedPoint {
            idx: None,
            value: Vector3::new(N::neg_infinity(), N::neg_infinity(), N::neg_infinity()),
        }
    }
    /// Returns `true` if the `IndexedPoint` does not contain infinities and the index is set to a
    /// value.
    pub fn is_valid(&self) -> bool {
        self.idx.is_some() && !self.value.iter().any(|c| c.is_infinite())
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "The supplied point cloud contains less than four points")]
    TooFewPoints,
    #[fail(display = "The supplied point cloud is degenerate")]
    DegenerateMesh,
    #[fail(display = "The supplied point cloud is collinear")]
    CollinearMesh,
    #[fail(display = "The supplied point cloud is coplanar")]
    CoplanarMesh,
    #[fail(display = "The triangle is degenerate")]
    DegenerateTriangle,
    #[fail(display = "{}", _0)]
    MeshError(#[cause] MError),
}

impl From<MError> for Error {
    fn from(value: MError) -> Self {
        Error::MeshError(value)
    }
}
