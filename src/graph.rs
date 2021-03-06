use std::fmt;
use std::iter;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Range};
use std::slice;
use std::cmp;

use Direction::{Incoming, Outgoing};

// Index into the NodeIndex and EdgeIndex arrays
/// Edge direction
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    /// Edges from a node.
    Outgoing = 0,
    /// Edges to a node.
    Incoming = 1,
}
impl Direction {
    /// Return the opposite of `Direction`
    pub fn opposite(&self) -> Direction {
        match *self {
            Outgoing => Incoming,
            Incoming => Outgoing,
        }
    }
    /// Return `0` for `Outgoing` and `1` for `Incoming`
    pub fn index(&self) -> usize {
        // (*self as usize) & 0x1
        match *self {
            Outgoing => 0,
            Incoming => 1,
        }
    }
}
const DIRECTIONS: [Direction; 2] = [Outgoing, Incoming];

/// Marker type for directed graphs
pub struct Directed;
/// Marker type for undirected graphs
pub struct Undirected;
/// Edge type: determines whether a graph has directed edges or not
pub trait EdgeType {
    /// whether the graph has directed edges or not
    fn is_directed() -> bool;
}
impl EdgeType for Directed {
    fn is_directed() -> bool {
        true
    }
}
impl EdgeType for Undirected {
    fn is_directed() -> bool {
        false
    }
}

/// Convert an element like `(i, j)` or `(i, j, w)` into a triple
/// of source, target, edge weight.
///
/// For `Graph::from_edges`
pub trait IntoWeightedEdge<E> {
    /// The node-ID type (Ix)
    type NodeId;
    /// Transform the tuple to a valid (source, target, weight) triple.
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E);
}
impl<E, Ix> IntoWeightedEdge<E> for (Ix, Ix)
where
    E: Default,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = self;
        (s, t, E::default())
    }
}
impl<'a, E, Ix> IntoWeightedEdge<E> for (Ix, Ix, &'a E)
where
    E: Clone,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (a, b, c) = self;
        (a, b, c.clone())
    }
}
impl<'a, E, Ix> IntoWeightedEdge<E> for &'a (Ix, Ix)
where
    Ix: Copy,
    E: Default,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = *self;
        (s, t, E::default())
    }
}
impl<'a, E, Ix> IntoWeightedEdge<E> for &'a (Ix, Ix, E)
where
    Ix: Copy,
    E: Clone,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        self.clone()
    }
}

#[derive(Debug, PartialEq)]
enum Pair<T> {
    None,
    One(T),
    Both(T, T),
}
fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Pair<&mut T> {
    if cmp::max(a, b) >= slc.len() {
        Pair::None
    } else if a == b {
        Pair::One(&mut slc[a])
    } else {
        // safe, because a and b are in bounds and distinct
        unsafe {
            let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
            let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
            Pair::Both(ar, br)
        }
    }
}

#[cfg(test)]
#[test]
fn test_index_twice() {
    let mut arr = [5, 7, 3, 9];
    assert_eq!(index_twice(&mut arr, 1, 3), Pair::Both(&mut 7, &mut 9));
    assert_eq!(index_twice(&mut arr, 1, 1), Pair::One(&mut 7));
    assert_eq!(index_twice(&mut arr, 2, 7), Pair::None);
}

/// The default integer type for graph indices.
/// `u32` is the default to reduce the size of the graph's data and improve
/// performance in the common case.
///
/// Used for node and edge indices in `Graph`
pub type DefaultIx = u32;

/// Trait for the unsigned integer type used for node and edge indices
pub trait IndexType: Copy + Default + Ord + fmt::Debug + 'static {
    /// Construct a new `IndexType` from an `usize`.
    fn new(x: usize) -> Self;
    /// Index for internal data structure access.
    fn index(&self) -> usize;
    /// The types max-value
    fn max() -> Self;
}
macro_rules! impl_index_type {
    ($t:ty, $i:ident) => {
        impl IndexType for $t {
            fn new(x: usize) -> Self {
                x as $t
            }
            fn index(&self) -> usize {
                *self as usize
            }
            fn max() -> Self {
                ::std::$i::MAX
            }
        }
    };
    ($a:tt) => {
        impl_index_type!($a, $a);
    };
}
impl_index_type!(usize);
impl_index_type!(u32);
impl_index_type!(u16);
impl_index_type!(u8);

/// Node identifier
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeIndex<Ix = DefaultIx>(Ix);
impl<Ix: IndexType> NodeIndex<Ix> {
    /// Construct a new `NodeIndex`.
    pub fn new(x: usize) -> Self {
        NodeIndex(IndexType::new(x))
    }
    /// Internal index (edge-endpoints and directions are internally
    /// represented as 2-ary arrays
    pub fn index(self) -> usize {
        self.0.index()
    }
    /// Represents an invalid `NodeIndex`
    pub fn end() -> Self {
        NodeIndex(IndexType::max())
    }
    fn _into_edge(self) -> EdgeIndex<Ix> {
        EdgeIndex(self.0)
    }
}

/// Edge identifier
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeIndex<Ix = DefaultIx>(Ix);
impl<Ix: IndexType> EdgeIndex<Ix> {
    /// Construct a new `EdgeIndex`.
    pub fn new(x: usize) -> Self {
        EdgeIndex(IndexType::new(x))
    }
    /// Internal index (edge-endpoints and directions are internally
    /// represented as 2-ary arrays
    pub fn index(self) -> usize {
        self.0.index()
    }
    /// An invalid `EdgeIndex` used to denote absence of an edge,
    /// for example to end an adjacency list
    pub fn end() -> Self {
        EdgeIndex(IndexType::max())
    }
    fn _into_node(self) -> NodeIndex<Ix> {
        NodeIndex(self.0)
    }
}

/// The graph's node type.
#[derive(Debug)]
pub struct Node<N, Ix = DefaultIx> {
    /// Associated node data
    pub data: N,
    /// Next edge in outgoing and incoming edge lists
    next: [EdgeIndex<Ix>; 2],
}
impl<N, Ix: IndexType> Node<N, Ix> {
    /// Accessor for data structure internals:
    /// the first edge in the given direction.
    pub fn next_edge(&self, dir: Direction) -> EdgeIndex<Ix> {
        self.next[dir.index()]
    }
}
impl<N, Ix> Clone for Node<N, Ix>
where
    N: Clone,
    Ix: Copy,
{
    fn clone(&self) -> Self {
        Node {
            data: self.data.clone(),
            next: self.next, // EdgeIndex is Copy
        }
    }
}
type NodeList<N, Ix> = Vec<Node<N, Ix>>;

/// The graph's edge type.
#[derive(Debug)]
pub struct Edge<E, Ix = DefaultIx> {
    /// Associated edge data
    pub weight: E,
    /// Next edge in outgoing and incoming edge lists
    next: [EdgeIndex<Ix>; 2],
    /// Start and End node index
    node: [NodeIndex<Ix>; 2],
}
impl<E, Ix: IndexType> Edge<E, Ix> {
    /// Accessor for data structure internatls: the next edge for the given direction
    pub fn next_edge(&self, dir: Direction) -> EdgeIndex<Ix> {
        self.next[dir.index()]
    }
    /// Return the source node index
    pub fn source(&self) -> NodeIndex<Ix> {
        self.node[0]
    }
    /// Return the target node index
    pub fn target(&self) -> NodeIndex<Ix> {
        self.node[1]
    }
}
impl<E, Ix> Clone for Edge<E, Ix>
where
    E: Clone,
    Ix: Copy,
{
    fn clone(&self) -> Self {
        Edge {
            weight: self.weight.clone(),
            next: self.next, // EdgeIndex is Copy
            node: self.node, // NodeIndex is Copy
        }
    }
}

type EdgeList<E, Ix> = Vec<Edge<E, Ix>>;

/// `Graph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation
///
/// `Graph` is parameterized over:
///
/// - Associated data `N` for nodes and `E` for edges (*weight*).
///   The associated data can be of an arbitrary type.
/// - Edge type `Ty` that determines whether the grap edges are directed or undirected.
/// - Index type `Ix`, which determines the maximum size of the graph.
///
/// The graph uses **O(|V| + |E|)** space and allows fast node and edge insert,
/// efficient graph search and graph algorithms.
/// It implements **O(e')** edge lookup and edge and node removals, where **e'**
/// is some local measure of edge count.
///
/// Here is an example of building a graph with directed edges:
/// ```
/// use graphs::*;
///
/// let mut deps = Graph::<&str, &str>::new();
/// let pg = deps.add_node("petgraph");
/// let fb = deps.add_node("fixedbitset");
/// let qc = deps.add_node("quickcheck");
/// let rand = deps.add_node("rand");
/// let libc = deps.add_node("libc");
/// deps.extend_with_edges(&[
///     (pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc),
/// ]);
/// ```
///
/// ### Graph Indices
///
/// The graph maintains indices for nodes and edges, and node data and edge
/// weights may be accessed mutably. Indices range in a compact interval,
/// for example for *n* nodes indices are 0 to *n* - 1 inclusive.
///
/// `NodeIndex` and `EdgeIndex` are types that act as references to nodes and edges,
/// but these are only stable across certain operations.
/// **Adding nodes or edges keeps indices stable.
/// Removing nodes or edges may shift other indices.**
/// Removing a node will force the last node to shift its index to
/// take its place. Similarly, removing an edge shifts the index of the last edge.
///
/// The `Ix` parameter is `u32` by default. The goal is that you can ignore this
/// parameter completely unless you need a very big graph -- then you can use `usize`.
///
/// ### Pros and Cons
///
/// * The fact that the node and edge indices in the graph each are numbered in compact
///   intervals (from 0 to *n* - 1 for *n* nodes) simplifies some graph algorithms.
///
/// * You can select graph index integer type after the size of the graph. A smaller
///   size may have better performance.
///
/// * Using indices allows mutation while traversing the graph.
///
/// * You can create several graphs using equal node indices but with different
///   weights or different edges.
///
/// * The `Graph` is a regular rust collection and is `Send` and `Sync`
///   (as long as associated data `N` and `E` are).
///
/// * Some indices shift during node or edge removal, so that is a drawback
///   of removing elements. Indices don't allow as much compile time checking
///   as references.
#[derive(Debug)]
pub struct Graph<N, E, Ty = Directed, Ix = DefaultIx> {
    nodes: Vec<Node<N, Ix>>,
    edges: Vec<Edge<E, Ix>>,
    ty: PhantomData<Ty>,
}

/// A `Graph` with directed edges.
///
/// For example, an edge from *1* to *2* is distinct from
/// an edge from *2* to *1*
pub type DiGraph<N, E, Ix = DefaultIx> = Graph<N, E, Directed, Ix>;
/// A `Graph` with undirected edges.
///
/// For example, an edge from *1* to *2* is equivalent to
/// an edge from *2* to *1*
pub type UnGraph<N, E, Ix = DefaultIx> = Graph<N, E, Undirected, Ix>;

impl<N, E> Graph<N, E, Directed> {
    /// Create a new `Graph` with directed edges.
    ///
    /// This is a convenience method. Use `Graph::with_capacity` or `Graph::default`
    /// for a constructor that is generic in all the type parameters of `Graph`
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            ty: PhantomData,
        }
    }
}
impl<N, E> Graph<N, E, Undirected> {
    /// Create a new `Graph` with undirected edges.
    ///
    /// This is a convenience method. Use `Graph::with_capacity` or `Graph::default`
    /// for a constructor that is generic in all the type parameters of `Graph`
    pub fn new_undirected() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            ty: PhantomData,
        }
    }
}
impl<N, E, Ty: EdgeType, Ix: IndexType> Graph<N, E, Ty, Ix> {
    /// Create a new `Graph` with estimated capacity.
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: Vec::with_capacity(nodes),
            edges: Vec::with_capacity(edges),
            ty: PhantomData,
        }
    }
    /// Return the number of nodes (vertices) in the graph.
    ///
    /// Computes in **O(1)** time.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// Return the number of edges in the graph.
    ///
    /// Computes in **O(1)** time.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// Whether the graph has directed edges or not.
    #[inline]
    pub fn is_directed(&self) -> bool {
        Ty::is_directed()
    }
    /// Add a node with associated data `weight` to the graph.
    ///
    /// Computes in **O(1)** time.
    ///
    /// Return the index of the new node.
    ///
    /// **Panics** if the graph is at the maximum number of nodes for
    /// its index type (N/A if usize)
    pub fn add_node(&mut self, data: N) -> NodeIndex<Ix> {
        let new_node = Node {
            data,
            next: [EdgeIndex::end(), EdgeIndex::end()],
        };
        let node_idx = NodeIndex::new(self.nodes.len());
        assert!(NodeIndex::end() != node_idx);
        self.nodes.push(new_node);
        node_idx
    }
    /// Access the data for node `a`.
    ///
    /// Also available with indexing syntax: `&graph[a]`.
    pub fn node_data(&self, a: NodeIndex<Ix>) -> Option<&N> {
        self.nodes.get(a.index()).map(|n| &n.data)
    }
    /// Access the data for node `a`, mutably.
    ///
    /// Also available with indexing syntax: `&mut graph[a]`.
    pub fn node_data_mut(&mut self, a: NodeIndex<Ix>) -> Option<&mut N> {
        self.nodes.get_mut(a.index()).map(|n| &mut n.data)
    }
    /// Add an edge from `a` to `b` to the graph, with its associated data `weight`.
    ///
    /// Return the index of the new edge.
    ///
    /// Computes in **O(1)** time.
    ///
    /// **Panics** if any of the nodes don't exist.<br>
    /// **Panics** if the graph is at the maximum number of edges for its index
    /// type (N/A if usize).
    ///
    /// **Note:** `Graph` allows adding parallel ("duplicate") edges. If you want
    /// to avoid this, use [`.update_edge(a,b,weight)`](#method.update_edge) instead.
    pub fn add_edge(&mut self, a: NodeIndex<Ix>, b: NodeIndex<Ix>, weight: E) -> EdgeIndex<Ix> {
        let edge_idx = EdgeIndex::new(self.edges.len());
        assert!(EdgeIndex::end() != edge_idx);
        let mut edge = Edge {
            weight,
            node: [a, b],
            next: [EdgeIndex::end(), EdgeIndex::end()],
        };
        match index_twice(&mut self.nodes, a.index(), b.index()) {
            Pair::None => panic!("Graph::add_edge(): node indices out of bound"),
            Pair::One(an) => {
                edge.next = an.next;
                an.next[0] = edge_idx;
                an.next[1] = edge_idx;
            }
            Pair::Both(an, bn) => {
                edge.next = [an.next[0], bn.next[1]];
                an.next[0] = edge_idx;
                bn.next[1] = edge_idx;
            }
        }
        self.edges.push(edge);
        edge_idx
    }
    /// Add or update an edge from `a` to `b`.
    /// If the edge already exists, its weight is updated.
    ///
    /// Return the index of the affected edge.
    ///
    /// Computes in **O(e')** time, where **e'** is the number of edges
    /// connected to `a` (and `b`, if the graph edges are undirected).
    ///
    /// **Panics** if any of the nodes don't exist.
    pub fn update_edge(&mut self, a: NodeIndex<Ix>, b: NodeIndex<Ix>, weight: E) -> EdgeIndex<Ix> {
        if let Some(ix) = self.find_edge(a, b) {
            if let Some(ed) = self.edge_weight_mut(ix) {
                *ed = weight;
                return ix;
            }
        }
        self.add_edge(a, b, weight)
    }
    /// Access the weight for edge `e`.
    ///
    /// Also available with indexing syntax: `&graph[e]`.
    pub fn edge_weight(&self, e: EdgeIndex<Ix>) -> Option<&E> {
        self.edges.get(e.index()).map(|ed| &ed.weight)
    }
    /// Access the weight for edge `e`, mutably.
    ///
    /// Also available wth indexing syntax: `&mut graph[e]`.
    pub fn edge_weight_mut(&mut self, e: EdgeIndex<Ix>) -> Option<&mut E> {
        self.edges.get_mut(e.index()).map(|ed| &mut ed.weight)
    }
    /// Access the source and target nodes for `e`.
    pub fn edge_endpoints(&self, e: EdgeIndex<Ix>) -> Option<(NodeIndex<Ix>, NodeIndex<Ix>)> {
        self.edges
            .get(e.index())
            .map(|ed| (ed.source(), ed.target()))
    }
    /// Remove `a` from the graph if it exists, and return its weight.
    /// If it doesn't exist in the graph, return `None`.
    ///
    /// Apart from `a`, this invalidates the last node index in the graph (that node will
    /// adopt the removed node index). Edge indices are invalidated as they would be following
    /// the removal of each edge with an endpoint in `a`.
    ///
    /// Computes in **O(e')** time, where **e'** is the number of affected adges,
    /// including `n` calls to `.remove_edge()` where *n* is the number of edges
    /// with an endpoint in `a`, and including the edges with an edpoint in the displaced node.
    pub fn remove_node(&mut self, a: NodeIndex<Ix>) -> Option<N> {
        self.nodes.get(a.index())?;
        for d in &DIRECTIONS {
            let k = d.index();
            loop {
                let next = self.nodes[a.index()].next[k];
                if next == EdgeIndex::end() {
                    break;
                }
                let ret = self.remove_edge(next);
                debug_assert!(ret.is_some());
                let _ = ret;
            }
        }

        let node = self.nodes.swap_remove(a.index());

        let swap_edges = match self.nodes.get(a.index()) {
            None => return Some(node.data),
            Some(ed) => ed.next,
        };

        let old_index = NodeIndex::new(self.nodes.len());
        let new_index = a;

        for &d in &DIRECTIONS {
            let k = d.index();
            let mut edges = edges_walker_mut(&mut self.edges, swap_edges[k], d);
            while let Some(curedge) = edges.next_edge() {
                debug_assert!(curedge.node[k] == old_index);
                curedge.node[k] = new_index;
            }
        }
        Some(node.data)
    }
    /// For edge `e` with endpoints `edge_node`, replace links to it,
    /// with links to `edge_next`.
    fn change_edge_links(
        &mut self,
        edge_node: [NodeIndex<Ix>; 2],
        e: EdgeIndex<Ix>,
        edge_next: [EdgeIndex<Ix>; 2],
    ) {
        for &d in &DIRECTIONS {
            let k = d.index();
            let node = match self.nodes.get_mut(edge_node[k].index()) {
                Some(r) => r,
                None => {
                    debug_assert!(
                        false,
                        "Edge's endpoint  dir={:?} index={:?} not found",
                        d, edge_node[k]
                    );
                    return;
                }
            };
            let fst = node.next[k];
            if fst == e {
                // println!("Updating first edge 0 for node {}, set to {}", edge_node[0], edge_next[0]);
                node.next[k] = edge_next[k];
            } else {
                let mut edges = edges_walker_mut(&mut self.edges, fst, d);
                while let Some(curedge) = edges.next_edge() {
                    if curedge.next[k] == e {
                        curedge.next[k] = edge_next[k];
                        break;
                    }
                }
            }
        }
    }
    /// Remove an edge and return its edge weight, or `None` if it didn't exist.
    ///
    /// Apart from `e`, this invalidates the last index in the graph
    /// (that edge will adopt the removed edge index)
    ///
    /// Computes in **O(e')** time, where **e'** is the size of four particular
    /// edge lists, the vertices of `e` and the vertices of another affected edge.
    pub fn remove_edge(&mut self, e: EdgeIndex<Ix>) -> Option<E> {
        let (edge_node, edge_next) = match self.edges.get(e.index()) {
            None => return None,
            Some(x) => (x.node, x.next),
        };
        self.change_edge_links(edge_node, e, edge_next);
        self.remove_edge_adjust_indices(e)
    }
    fn remove_edge_adjust_indices(&mut self, e: EdgeIndex<Ix>) -> Option<E> {
        let edge = self.edges.swap_remove(e.index());
        let swap = match self.edges.get(e.index()) {
            None => return Some(edge.weight),
            Some(ed) => ed.node,
        };
        let swapped_e = EdgeIndex::new(self.edges.len());
        self.change_edge_links(swap, swapped_e, [e, e]);
        Some(edge.weight)
    }
    /// Return an iterator of all nodes with an edge starting from `a`.
    ///
    /// - `Directed`: Outgoing edges from `a`.
    /// - `Undirected`: All edges from or to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `NodeIndex<Ix>`.
    ///
    /// Use [`.neighbors(a).detach()`][1] to get a neighbor walker that does
    /// not borrow from the graph.
    ///
    /// [1]: struct.Neighbors.html#method.detach
    pub fn neighbors(&self, a: NodeIndex<Ix>) -> Neighbors<E, Ix> {
        self.neighbors_directed(a, Outgoing)
    }
    /// Return an iterator of all neighbors that have an edge between them and `a`,
    /// in the specified direction.
    /// If the graph's edges are undirected, this is equivalent to *.neighbors(a)*.
    ///
    /// - `Directed`, `Outgoing`: All edges from `a`.
    /// - `Directed`, `Incoming`: All edges to `a`.
    /// - `Undirected`: All edges from or to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `NodeIndex<Ix>`.
    ///
    /// For a `Directed` graph, neighbors are listed in reverse order of their addition
    /// to the graph, so the most recently added edge's neighbor is listed first.
    /// The order in an `Undirected` graph is arbitrary.
    ///
    /// Use [`.neighbors_directed(a, dir).detach()`][1] to get a neighbor walker that
    /// doesn't borrow from the graph.
    ///
    /// [1]: struct.Neighbors.html#method.detach
    pub fn neighbors_directed(&self, a: NodeIndex<Ix>, dir: Direction) -> Neighbors<E, Ix> {
        let mut iter = self.neighbors_undirected(a);
        if self.is_directed() {
            let k = dir.index();
            iter.next[1 - k] = EdgeIndex::end();
            iter.skip_start = NodeIndex::end();
        }
        iter
    }
    /// Return an iterator of all neighbors that have an edge between them and `a`, in either direction.
    /// If the graph`s edges are undirected, this is equivalent to *.neighbors(a)*.
    ///
    /// - 'Directed' and 'Undirected': All edges from or to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `NodeIndex<Ix>`.
    ///
    /// Use [`.neighbors_undirected(a).detach()`][1] to get a neighbor walker
    /// that doesn't borrow from the graph.
    ///
    /// [1]: struct.Neighbors.html#method.detach
    pub fn neighbors_undirected(&self, a: NodeIndex<Ix>) -> Neighbors<E, Ix> {
        Neighbors {
            skip_start: a,
            edges: &self.edges,
            next: match self.nodes.get(a.index()) {
                None => [EdgeIndex::end(), EdgeIndex::end()],
                Some(n) => n.next,
            },
        }
    }
    /// Return an iterator of all edges of `a`.
    ///
    /// `Directed`: Outgoing edges from `a`.
    /// `Undirected`: All edges connected to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `EdgeReference<E, Ix>`.
    pub fn edges(&self, a: NodeIndex<Ix>) -> Edges<E, Ty, Ix> {
        self.edges_directed(a, Outgoing)
    }
    /// Return an iterator of all edges of `a`, in the specified direction.
    ///
    /// `Directed`, `Outgoing`: All edges from `a`.
    /// `Directed`, `Incoming`: All edges to `a`.
    /// `Undirected`: All edges connected to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `EdgeReference<E, Ix>`.
    pub fn edges_directed(&self, a: NodeIndex<Ix>, dir: Direction) -> Edges<E, Ty, Ix> {
        let mut iter = self.edges_undirected(a);
        if self.is_directed() {
            iter.direction = Some(dir);
        }
        if self.is_directed() && dir == Incoming {
            iter.next.swap(0, 1);
        }
        iter
    }
    /// Return an iterator of all edges of `a`.
    ///
    /// `Directed` and `Undirected`: All edges connected to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `EdgeReference<E, Ix>`.
    pub fn edges_undirected(&self, a: NodeIndex<Ix>) -> Edges<E, Ty, Ix> {
        Edges {
            skip_start: a,
            edges: &self.edges,
            direction: None,
            next: match self.nodes.get(a.index()) {
                None => [EdgeIndex::end(), EdgeIndex::end()],
                Some(n) => n.next,
            },
            ty: PhantomData,
        }
    }

    /// Lookup if there is an edge from `a` to `b`.
    ///
    /// Computes in **O(e')** time, where **e'** is the number of edges connected
    /// to `a` (and `b` if the graph edges are undirected).
    pub fn contains_edge(&self, a: NodeIndex<Ix>, b: NodeIndex<Ix>) -> bool {
        self.find_edge(a, b).is_some()
    }
    /// Lookup an edge from `a` to `b`.
    ///
    /// Computes in **O(e')** time, where **e'** is the number of edges connected
    /// to `a` (and `b` if the graph edges are undirected).
    pub fn find_edge(&self, a: NodeIndex<Ix>, b: NodeIndex<Ix>) -> Option<EdgeIndex<Ix>> {
        if !self.is_directed() {
            self.find_edge_undirected(a, b).map(|(ix, _)| ix)
        } else {
            match self.nodes.get(a.index()) {
                None => None,
                Some(node) => self.find_edge_directed_from_node(node, b),
            }
        }
    }
    fn find_edge_directed_from_node(
        &self,
        node: &Node<N, Ix>,
        b: NodeIndex<Ix>,
    ) -> Option<EdgeIndex<Ix>> {
        let mut edix = node.next[0];
        while let Some(edge) = self.edges.get(edix.index()) {
            if edge.node[1] == b {
                return Some(edix);
            }
            edix = edge.next[0];
        }
        None
    }
    /// Lookup an edge from `a` to `b`, in either direction.
    ///
    /// If the graph is undirected, then this is equivalent to `.find_edge()`.
    ///
    /// Return the edge index and its directionality, with `Outgoing` meaning
    /// from `a` to `b` and `Incoming` the reverse,
    /// or `None` if the edge does not exist.
    pub fn find_edge_undirected(
        &self,
        a: NodeIndex<Ix>,
        b: NodeIndex<Ix>,
    ) -> Option<(EdgeIndex<Ix>, Direction)> {
        match self.nodes.get(a.index()) {
            None => None,
            Some(node) => self.find_edge_undirected_from_node(node, b),
        }
    }
    fn find_edge_undirected_from_node(
        &self,
        node: &Node<N, Ix>,
        b: NodeIndex<Ix>,
    ) -> Option<(EdgeIndex<Ix>, Direction)> {
        for &d in &DIRECTIONS {
            let k = d.index();
            let mut edix = node.next[k];
            while let Some(edge) = self.edges.get(edix.index()) {
                if edge.node[1 - k] == b {
                    return Some((edix, d));
                }
                edix = edge.next[k];
            }
        }
        None
    }

    /// Return an iterator over the source nodes of the graph/ the nodes
    /// without edges to them
    ///
    /// Wrapper for `.externals(Incoming)`
    ///
    /// For a graph with undirected edges, this equals `.sink_nodes()` in
    /// returning an iterator over all nodes with no edges to or from them.
    pub fn source_nodes(&self) -> Externals<N, Ty, Ix> {
        self.externals(Incoming)
    }
    /// Return an iterator over the sink nodes of the graph/ the nodes
    /// without edges from them
    ///
    /// Wrapper for `.externals(Outgoing)`
    ///
    /// For a graph with undirected edges, this equals `.source_nodes()` in
    /// returning an iterator over all nodes with no edges to or from them.
    pub fn sink_nodes(&self) -> Externals<N, Ty, Ix> {
        self.externals(Outgoing)
    }
    /// Return an iterator over either the nodes without edges
    /// to them (`Incoming`) or from them (`Outgoing`).
    ///
    /// An *internal* node has both incoming and outgoing edges.
    /// The nodes in `.externals(Incoming)` are the source nodes and
    /// `.externals(Outgoing)` are the sinks of the graph.
    ///
    /// For a graph with undirected edges, both the sinks and the sources are
    /// just nodes without edges.
    ///
    /// The whole iteration computes in **O(|V|)** time.
    pub fn externals(&self, dir: Direction) -> Externals<N, Ty, Ix> {
        Externals {
            iter: self.nodes.iter().enumerate(),
            dir,
            ty: PhantomData,
        }
    }
    /// Return an iterator over the node indices of the graph.
    pub fn node_indices(&self) -> NodeIndices<Ix> {
        NodeIndices {
            r: 0..self.node_count(),
            ty: PhantomData,
        }
    }
    /// Return an iterator yielding mutable access to all node weights.
    ///
    /// The order in which weighs are yielded
    /// matches the order of their node indices.
    pub fn node_weights_mut(&mut self) -> NodeWeightsMut<N, Ix> {
        NodeWeightsMut {
            nodes: self.nodes.iter_mut()
        }
    }
    /// Return an iterator over the edge indices of the graph.
    pub fn edge_indices(&self) -> EdgeIndices<Ix> {
        EdgeIndices {
            r: 0..self.edge_count(),
            ty: PhantomData,
        }
    }
    /// Create an iterator over all edges, in indexed order.
    ///
    /// Iterator element type is `EdgeReference<E, Ix>`.
    pub fn edge_references(&self) -> EdgeReferences<E, Ix> {
        EdgeReferences {
            iter: self.edges.iter().enumerate()
        }
    }
    /// Return an iterator yielding mutable access to all edge weights.
    ///
    /// The order in which weights are yielded
    /// matches the order of their edge indices
    pub fn edge_weights_mut(&mut self) -> EdgeWeightsMut<E, Ix> {
        EdgeWeightsMut {
            edges: self.edges.iter_mut()
        }
    }

    // Remaining methods are of the more internal flavour, read-only access to
    // the data structure`s internals.

    /// Access the internal node array.
    pub fn raw_nodes(&self) -> &[Node<N, Ix>] {
        &self.nodes
    }
    /// Access the internal edge array.
    pub fn raw_edges(&self) -> &[Edge<E, Ix>] {
        &self.edges
    }
    /// Convert the graph into a vector of nodes and a vector of edges.
    pub fn into_nodes_edges(self) -> (NodeList<N, Ix>, EdgeList<E, Ix>) {
        // pub fn into_nodes_edges(self) -> (Vec<Node<N, Ix>>, Vec<Edge<E, Ix>>) {
        (self.nodes, self.edges)
    }
    /// Accessor for data structure internals: the first edge in the given direction.
    pub fn first_edge(&self, a: NodeIndex<Ix>, dir: Direction) -> Option<EdgeIndex<Ix>> {
        match self.nodes.get(a.index()) {
            None => None,
            Some(node) => {
                let edix = node.next[dir.index()];
                if edix == EdgeIndex::end() {
                    None
                } else {
                    Some(edix)
                }
            }
        }
    }
    /// Accessor for data structure internals: the next edge in the given direction.
    pub fn next_edge(&self, a: NodeIndex<Ix>, dir: Direction) -> Option<EdgeIndex<Ix>> {
        match self.edges.get(a.index()) {
            None => None,
            Some(node) => {
                let edix = node.next[dir.index()];
                if edix == EdgeIndex::end() {
                    None
                } else {
                    Some(edix)
                }
            }
        }
    }

    // pub fn index_twice_mut<T, U>(&mut self, i: T, j: U) -> (&mut <Self as Index<T>>::Output, &mut <Self as Index<U>>::Output) {}

    /// Reverse the direction of all edges.
    pub fn reverse(&mut self) {
        for edge in &mut self.edges {
            edge.node.swap(0, 1);
            edge.next.swap(0, 1);
        }
        for node in &mut self.nodes {
            node.next.swap(0, 1);
        }
    }
    /// Remove all nodes and edges.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
    /// Remove all edges.
    pub fn clear_edges(&mut self) {
        self.edges.clear();
        for node in &mut self.nodes {
            node.next = [EdgeIndex::end(), EdgeIndex::end()];
        }
    }
    /// Return the current node and edge capacity of the graph.
    pub fn capacity(&self) -> (usize, usize) {
        (self.nodes.capacity(), self.edges.capacity())
    }
    /// Reserves capacity for at least `additional` more nodes to be inserted
    /// in the graph. Graph may reserve more space to avoid frequent reallocations.
    ///
    /// **Panics** if the new capacity overflows `usize`.
    pub fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }
    /// Reserves capacity for at least `additional` more edges to be inserted
    /// in the graph. Graph may reserve more space to avoid frequent reallocations.
    ///
    /// **Panics** if the new capacity overflows `usize`.
    pub fn reserve_edges(&mut self, additional: usize) {
        self.edges.reserve(additional);
    }
    /// Reserves the minimum capacity for exactly `additional` more nodes to be
    /// inserted in the graph. Does nothing if the capacity is already sufficient.
    ///
    /// Prefer `reserve_nodes` if future insertions are expected.
    ///
    /// **Panics** if the new capacity overflows `usize`.
    pub fn reserve_exact_nodes(&mut self, additional: usize) {
        self.nodes.reserve_exact(additional);
    }
    /// Reserves the minimum capacity for exactly `additional` more edges to be
    /// inserted in the graph. Does nothing if the capacity is already sufficient.
    ///
    /// Prefer `reserve_edges` if future insertions are expected.
    ///
    /// **Panics** if the new capacity overflows `usize`.
    pub fn reserve_exact_edges(&mut self, additional: usize) {
        self.edges.reserve_exact(additional);
    }
    /// Shrinks the capacity of the underlying nodes collection as much as possible.
    pub fn shrink_to_fit_nodes(&mut self) {
        self.nodes.shrink_to_fit();
    }
    /// Shrinks the capacity of the underlying edges collection as much as possible.
    pub fn shrink_to_fit_edges(&mut self) {
        self.edges.shrink_to_fit();
    }
    /// Shrinks the capacity of the graph as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.nodes.shrink_to_fit();
        self.edges.shrink_to_fit();
    }
    // TODO:
    // pub fn retain_nodes<F>(&mut self, mut visit: F) where F: FnMut(Frozen<Self>, NodeIndex<Ix>) -> bool {}
    // pub fn retain_edges<F>(&mut self, mut visit: F) where F: FnMut(Frozen<Self>, EdgeIndex<Ix>) -> bool {}

    /// Create a new `Graph` from an iterable of edges.
    ///
    /// Node weights `N` are set to default values. Edge weights `E` may
    /// either be specified in the list, or they are filled with default
    /// values.
    ///
    /// Nodes are inserted automatically to match the edges.
    pub fn from_edges<I>(iterable: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoWeightedEdge<E>,
        <I::Item as IntoWeightedEdge<E>>::NodeId: Into<NodeIndex<Ix>>,
        N: Default,
    {
        let mut g = Self::with_capacity(0, 0);
        g.extend_with_edges(iterable);
        g
    }
    /// Extend the graph from an iterable of edges.
    ///
    /// Node weights $N$ are set to default values. Edge weights $E$
    /// may either be specified in the list, or they are filled with
    ///
    /// default values.
    /// Nodes are inserted automatically to match the edges.
    pub fn extend_with_edges<I>(&mut self, iterable: I)
    where
        I: IntoIterator,
        I::Item: IntoWeightedEdge<E>,
        <I::Item as IntoWeightedEdge<E>>::NodeId: Into<NodeIndex<Ix>>,
        N: Default,
    {
        let iter = iterable.into_iter();
        let (low, _) = iter.size_hint();
        self.edges.reserve(low);

        for elt in iter {
            let (source, target, weight) = elt.into_weighted_edge();
            let (source, target) = (source.into(), target.into());
            let nx = cmp::max(source, target);
            while nx.index() >= self.node_count() {
                self.add_node(N::default());
            }
            self.add_edge(source, target, weight);
        }
    }
    // pub fn map ...
    // pub fn filter_map ...

    /// Convert the graph into either undirected or directed. No edge adjustments
    /// are done, so you may want to go over the result to remove or add edges.
    ///
    /// Computes in **O(1)** time.
    pub fn into_edge_type<NewTy: EdgeType>(self) -> Graph<N, E, NewTy, Ix> {
        Graph {
            nodes: self.nodes,
            edges: self.edges,
            ty: PhantomData,
        }
    }
}
// * GRAPH TRAIT IMPLs * //
impl<N, E, Ty, Ix: IndexType> Clone for Graph<N, E, Ty, Ix>
where
    N: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        Graph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            ty: self.ty,
        }
    }
}
impl<N, E, Ty, Ix> Default for Graph<N, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    fn default() -> Self {
        Self::with_capacity(0, 0)
    }
}
/// Index the `Graph` by `NodeIndex` to access node data.
///
/// **Panics** if the node doesn't exist.
impl<N, E, Ty, Ix> Index<NodeIndex<Ix>> for Graph<N, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    type Output = N;
    fn index(&self, index: NodeIndex<Ix>) -> &N {
        &self.nodes[index.index()].data
    }
}
/// Index the `Graph` by `NodeIndex` to access node data.
///
/// **Panics** if the node doesn't exist.
impl<N, E, Ty, Ix> IndexMut<NodeIndex<Ix>> for Graph<N, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    fn index_mut(&mut self, index: NodeIndex<Ix>) -> &mut N {
        &mut self.nodes[index.index()].data
    }
}
/// Index the `Graph` by `EdgeIndex` to access edge weights.
///
/// **Panics** if the edge doesn't exist.
impl<N, E, Ty, Ix> Index<EdgeIndex<Ix>> for Graph<N, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    type Output = E;
    fn index(&self, index: EdgeIndex<Ix>) -> &E {
        &self.edges[index.index()].weight
    }
}
/// Index the `Graph` by `EdgeIndex` to access edge weights.
///
/// **Panics** if the edge doesn't exist.
impl<N, E, Ty, Ix> IndexMut<EdgeIndex<Ix>> for Graph<N, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    fn index_mut(&mut self, index: EdgeIndex<Ix>) -> &mut E {
        &mut self.edges[index.index()].weight
    }
}
// impl<'a, N, E, Ty, Ix> IntoNodeReferences for &'a Graph<N, E, Ty, Ix>
// where
// Ty: EdgeType,
// Ix: IndexType,
// {
// }

//* NODES *//
/// An iterator over either the nodes without edges to them or from them.
pub struct Externals<'a, N: 'a, Ty, Ix: IndexType = DefaultIx> {
    iter: iter::Enumerate<slice::Iter<'a, Node<N, Ix>>>,
    dir: Direction,
    ty: PhantomData<Ty>,
}
impl<'a, N: 'a, Ty, Ix> Iterator for Externals<'a, N, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    type Item = NodeIndex<Ix>;
    fn next(&mut self) -> Option<Self::Item> {
        let k = self.dir.index();
        loop {
            match self.iter.next() {
                None => return None,
                Some((index, node)) => {
                    if node.next[k] == EdgeIndex::end()
                        && (Ty::is_directed() || node.next[1 - k] == EdgeIndex::end())
                    {
                        return Some(NodeIndex::new(index));
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}
/// Iterator yielding mutable access to all node weights.
pub struct NodeWeightsMut<'a, N: 'a, Ix: IndexType = DefaultIx> {
    nodes: slice::IterMut<'a, Node<N, Ix>>,
}
impl<'a, N, Ix> Iterator for NodeWeightsMut<'a, N, Ix>
where
    Ix: IndexType,
{
    type Item = &'a mut N;
    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|node| &mut node.data)
    }
}
/// Iterator over the neighbors of a node.
///
/// Iterator element type is `NodeIndex<Ix>`.
///
/// Created with [`.neighbors()`][1], [`.neighbors_directed()`][2] or
/// [`.neighbors_undirected()`][3].
///
/// [1]: struct.Graph.html#method.neighbors
/// [2]: struct.Graph.html#method.neighbors_directed
/// [3]: struct.Graph.html#method.neighbors_undirected
pub struct Neighbors<'a, E: 'a, Ix: 'a = DefaultIx> {
    skip_start: NodeIndex<Ix>,
    edges: &'a [Edge<E, Ix>],
    next: [EdgeIndex<Ix>; 2],
}
impl<'a, E, Ix> Iterator for Neighbors<'a, E, Ix>
where
    Ix: IndexType,
{
    type Item = NodeIndex<Ix>;
    fn next(&mut self) -> Option<NodeIndex<Ix>> {
        // first any outgoing edges
        match self.edges.get(self.next[0].index()) {
            None => {}
            Some(edge) => {
                self.next[0] = edge.next[0];
                return Some(edge.node[1]);
            }
        }

        // then incoming edges
        // For an "undirected" iterator (traverse both incoming and
        // outgoing edge lists), make sure we don't double count
        // selfloops by skipping them in the incoming list.
        while let Some(edge) = self.edges.get(self.next[1].index()) {
            self.next[1] = edge.next[1];
            if edge.node[0] != self.skip_start {
                return Some(edge.node[0]);
            }
        }
        None
    }
}
impl<'a, E, Ix> Clone for Neighbors<'a, E, Ix>
where
    Ix: IndexType,
{
    fn clone(&self) -> Self {
        Neighbors {
            skip_start: self.skip_start,
            edges: self.edges,
            next: self.next, // EdgeIndex is Copy
        }
    }
}
impl<'a, E, Ix> Neighbors<'a, E, Ix>
where
    Ix: IndexType,
{
    /// Return a `walker` object that can be used to step through the neighbours
    /// and edges from the origin node.
    /// Note: The walker does not borrow from the graph, this is to allow mixing
    /// edge walking with mutating the graph's weights.
    pub fn detach(&self) -> WalkNeighbors<Ix> {
        WalkNeighbors {
            skip_start: self.skip_start,
            next: self.next,
        }
    }
}
/// A "walker" object that can be used to step through the edge list of a node.
///
/// Created with [`.detach()`](struct.Neighbors.html#method.detach).
///
/// The walker does not borrow from the graph, so it lets you step through
/// neighbors or incident edges while also mutating graph weights.
pub struct WalkNeighbors<Ix> {
    skip_start: NodeIndex<Ix>,
    next: [EdgeIndex<Ix>; 2],
}
impl<Ix> Clone for WalkNeighbors<Ix>
where
    Ix: IndexType,
{
    fn clone(&self) -> Self {
        WalkNeighbors {
            skip_start: self.skip_start,
            next: self.next,
        }
    }
}
impl<Ix: IndexType> WalkNeighbors<Ix> {
    /// Step to the next edge and its endpoint node in the walk for graph `g`.
    ///
    /// The next node indices are always the others than the starting point where
    /// the `WalkNeighbors` value was created.
    /// For an `Outgoing` walk, the target nodes,
    /// for an `Incoming` walk, the source nodes of the edge.
    pub fn next<N, E, Ty: EdgeType>(
        &mut self,
        g: &Graph<N, E, Ty, Ix>,
    ) -> Option<(EdgeIndex<Ix>, NodeIndex<Ix>)> {
        // First any outgoing edgees
        match g.edges.get(self.next[0].index()) {
            None => {}
            Some(edge) => {
                let ed = self.next[0];
                self.next[0] = edge.next[0];
                return Some((ed, edge.node[1]));
            }
        }
        // Then incoming edges
        // For an "undirected" iterator (traverse both incoming and outgoing
        // edge lists), make sure we don't double count selfloops by skipping
        // them in the incoming list.
        while let Some(edge) = g.edges.get(self.next[1].index()) {
            let ed = self.next[1];
            self.next[1] = edge.next[1];
            if edge.node[0] != self.skip_start {
                return Some((ed, edge.node[0]));
            }
        }
        None
    }

    /// Step to the next node in the walk for graph `g`.
    ///
    /// The next node indices are always the others than the starting point where
    /// the `WalkNeighbors` value was created.
    /// For an `Outgoing` walk, the target nodes,
    /// for an `Incoming` walk, the source nodes of the edge.
    pub fn next_node<N, E, Ty: EdgeType>(
        &mut self,
        g: &Graph<N, E, Ty, Ix>,
    ) -> Option<NodeIndex<Ix>> {
        self.next(g).map(|t| t.1)
    }
    /// Step to the next edge in the walk for graph `g`.
    pub fn next_edge<N, E, Ty: EdgeType>(
        &mut self,
        g: &Graph<N, E, Ty, Ix>,
    ) -> Option<EdgeIndex<Ix>> {
        self.next(g).map(|t| t.0)
    }
}
/// Iterator over all nodes of a graph.
pub struct NodeReferences<'a, N: 'a, Ix: IndexType = DefaultIx> {
    iter: iter::Enumerate<slice::Iter<'a, Node<N, Ix>>>,
}
impl<'a, N, Ix> Iterator for NodeReferences<'a, N, Ix>
where
    Ix: IndexType,
{
    type Item = (NodeIndex<Ix>, &'a N);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(i, node)| (NodeIndex::new(i), &node.data))
    }
}
impl<'a, N, Ix> DoubleEndedIterator for NodeReferences<'a, N, Ix>
where
    Ix: IndexType,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|(i, node)| (NodeIndex::new(i), &node.data))
    }
}
impl<'a, N, Ix> ExactSizeIterator for NodeReferences<'a, N, Ix>
where
    Ix: IndexType,
{
}
/// Iterator over the node indices of a graph.
#[derive(Clone, Debug)]
pub struct NodeIndices<Ix = DefaultIx> {
    r: Range<usize>,
    ty: PhantomData<fn() -> Ix>,
}
impl<Ix: IndexType> Iterator for NodeIndices<Ix> {
    type Item = NodeIndex<Ix>;
    fn next(&mut self) -> Option<Self::Item> {
        self.r.next().map(NodeIndex::new)
    }
}
impl<Ix: IndexType> DoubleEndedIterator for NodeIndices<Ix> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.r.next_back().map(NodeIndex::new)
    }
}
impl<Ix: IndexType> ExactSizeIterator for NodeIndices<Ix> {}

//* EDGES *//
struct EdgesWalkerMut<'a, E: 'a, Ix: IndexType = DefaultIx> {
    edges: &'a mut [Edge<E, Ix>],
    next: EdgeIndex<Ix>,
    dir: Direction,
}
fn edges_walker_mut<E, Ix>(
    edges: &mut [Edge<E, Ix>],
    next: EdgeIndex<Ix>,
    dir: Direction,
) -> EdgesWalkerMut<E, Ix>
where
    Ix: IndexType,
{
    EdgesWalkerMut { edges, next, dir }
}
impl<'a, E, Ix> EdgesWalkerMut<'a, E, Ix>
where
    Ix: IndexType,
{
    fn next_edge(&mut self) -> Option<&mut Edge<E, Ix>> {
        self.next().map(|t| t.1)
    }
    fn next(&mut self) -> Option<(EdgeIndex<Ix>, &mut Edge<E, Ix>)> {
        let this_index = self.next;
        let k = self.dir.index();
        match self.edges.get_mut(self.next.index()) {
            None => None,
            Some(edge) => {
                self.next = edge.next[k];
                Some((this_index, edge))
            }
        }
    }
}
/// Iterator over the edges from or to a node
pub struct Edges<'a, E: 'a, Ty, Ix: 'a = DefaultIx>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    /// starting node to skip over
    skip_start: NodeIndex<Ix>,
    edges: &'a [Edge<E, Ix>],

    /// next node to visit.
    /// If we are only following one direction, we only use next[0] regardless.
    next: [EdgeIndex<Ix>; 2],

    /// Which direction to follow
    /// None: Both,
    /// Some(d): d if Directed, Both if Undirected
    direction: Option<Direction>,
    ty: PhantomData<Ty>,
}
impl<'a, E, Ty, Ix> Iterator for Edges<'a, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    type Item = EdgeReference<'a, E, Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        // First the outgoing or incoming edges (directionally)
        let k = self.direction.unwrap_or(Outgoing).index();
        let i = self.next[0].index();
        match self.edges.get(i) {
            None => {}
            Some(&Edge {
                ref node,
                ref weight,
                ref next,
            }) => {
                self.next[0] = next[k];
                return Some(EdgeReference {
                    index: EdgeIndex::new(i),
                    node: *node,
                    weight,
                });
            }
        }
        // Stop here if we only follow one direction
        if self.direction.is_some() {
            return None;
        }

        // Then incoming edges
        // For an "undirected" iterator (traverse both incoming and
        // outgoing edge lists), make sure we don't double count
        // selfloops by skipping them in the incoming list.

        // We reach here if self.direction was None or Outgoing.
        debug_assert_eq!(k, 0);
        while let Some(edge) = self.edges.get(self.next[1].index()) {
            let i = self.next[1].index();
            self.next[1] = edge.next[1];
            if edge.node[0] != self.skip_start {
                // previously a call to swap_pair()
                let mut n: [_; 2] = edge.node;
                n.swap(0, 1);
                return Some(EdgeReference {
                    index: EdgeIndex::new(i),
                    node: n,
                    weight: &edge.weight,
                });
            }
        }
        None
    }
}
// fn swap_pair<T>(mut x: [T; 2]) -> [T; 2] {
//     x.swap(0, 1);
//     x
// }
impl<'a, E, Ty, Ix> Clone for Edges<'a, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    fn clone(&self) -> Self {
        Edges {
            skip_start: self.skip_start,
            edges: self.edges,
            next: self.next,
            direction: self.direction,
            ty: self.ty,
        }
    }
}

/// Reference to a `Graph` edge.
#[derive(Debug)]
pub struct EdgeReference<'a, E: 'a, Ix = DefaultIx> {
    index: EdgeIndex<Ix>,
    node: [NodeIndex<Ix>; 2],
    weight: &'a E,
}
impl<'a, E, Ix: IndexType> Clone for EdgeReference<'a, E, Ix> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, E, Ix: IndexType> PartialEq for EdgeReference<'a, E, Ix>
where
    E: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.index == rhs.index && self.weight == rhs.weight
    }
}
impl<'a, E, Ix: IndexType> Copy for EdgeReference<'a, E, Ix> {}

/// Iterator over all edges of a graph.
pub struct EdgeReferences<'a, E: 'a, Ix: IndexType = DefaultIx> {
    iter: iter::Enumerate<slice::Iter<'a, Edge<E, Ix>>>,
}
impl<'a, E, Ix: IndexType> Iterator for EdgeReferences<'a, E, Ix> {
    type Item = EdgeReference<'a, E, Ix>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, edge)| EdgeReference {
            index: EdgeIndex::new(i),
            node: edge.node,
            weight: &edge.weight,
        })
    }
}
impl<'a, E, Ix: IndexType> DoubleEndedIterator for EdgeReferences<'a, E, Ix> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(i, edge)| EdgeReference {
            index: EdgeIndex::new(i),
            node: edge.node,
            weight: &edge.weight,
        })
    }
}
impl<'a, E, Ix: IndexType> ExactSizeIterator for EdgeReferences<'a, E, Ix> {}

/// Iterator over the edge indices of a graph.
pub struct EdgeIndices<Ix = DefaultIx> {
    r: Range<usize>,
    ty: PhantomData<fn() -> Ix>,
}
impl<Ix: IndexType> Iterator for EdgeIndices<Ix> {
    type Item = EdgeIndex<Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        self.r.next().map(EdgeIndex::new)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.r.size_hint()
    }
}

/// Iterator yielding mutable access to all edge weights.
pub struct EdgeWeightsMut<'a, E: 'a, Ix: IndexType = DefaultIx> {
    edges: slice::IterMut<'a, Edge<E, Ix>>,
}
impl<'a, E, Ix> Iterator for EdgeWeightsMut<'a, E, Ix>
where
    Ix: IndexType,
{
    type Item = &'a mut E;
    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().map(|edge| &mut edge.weight)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.edges.size_hint()
    }
}
