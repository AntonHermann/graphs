#![allow(dead_code)]

use std::cmp::Ordering;
use std::result::Result as stdResult;

/// The weight of an edge
/// Infinity if the edge doesn't exist (yet)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    Infinity,
    W(usize),
}
impl Default for Weight {
    fn default() -> Self {
        Weight::Infinity
    }
}
impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Weight {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Weight::Infinity, &Weight::Infinity) => Ordering::Equal,
            (_, &Weight::Infinity) => Ordering::Less,
            (&Weight::Infinity, _) => Ordering::Greater,
            (&Weight::W(w_self), &Weight::W(w_other)) => w_self.cmp(&w_other),
        }
    }
}
impl From<usize> for Weight {
    fn from(w: usize) -> Weight {
        Weight::W(w)
    }
}

/// The error type used in `Graph`
/// May get expanded later to cover other error cases
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphError {
    InvalidVertex,
}

#[macro_export]
macro_rules! unwrap_vertex {
    ($e:expr) => {
        $e.ok_or(GraphError::InvalidVertex)?
    };
    ($e:expr, $ret:expr) => {
        match $e {
            Some(v) => v,
            None => return $ret,
        }
    };
}

/// A handle representing a vertex in a `Graph`
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct VertexId(pub usize);

pub type Result<T> = stdResult<T, GraphError>;

/// Abstract data type Graph (collection of Vertices and Edges)
/// T: Type of data objects stored in the vertices
pub trait Graph<T> {
    /// Creates an empty `Graph`.
    ///
    /// The Graph is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    fn new() -> Self;

    /// Returns a Vector containing all vertices
    fn vertices(&self) -> Vec<VertexId>;

    /// Returns a Vector containing all edges as (from, to, weight) tuple
    fn edges(&self) -> Vec<(VertexId, VertexId, Weight)>;

    /// Returns the `Weight` of a specific edge.
    ///
    /// Returns Weight::Infinity if the edge doesn't exist
    fn get_weight(&self, from: VertexId, to: VertexId) -> Result<Weight>;

    /// Creates a new vertex and returns a handle to it.
    fn create_vertex(&mut self) -> VertexId;

    /// Delete a vertex.
    ///
    /// Returns Err(GraphError::InvalidVertex) if this vector didn't exist
    fn delete_vertex(&mut self, vertex: VertexId) -> Result<()>;

    /// Sets the data associated with the given vertex.
    ///
    /// Overrides the previous data
    ///
    /// Returns Err(GraphError::InvalidVertex) if the vector doesn't exist
    fn set_data(&mut self, vertex: VertexId, data: T) -> Result<()>;

    /// Returns the data associated with the given vertex (if existent)
    ///
    /// Returns Err(GraphError::InvalidVertex) if the vector doesn't exist
    /// Returns Ok(None) if no data was associated with this vector
    fn get_data(&self, vertex: VertexId) -> Result<Option<&T>>;
}

pub trait DirectedGraph<T>: Graph<T> {
    fn outgoing_edges(&self, vertex: VertexId) -> Result<Vec<(VertexId, Weight)>>;
    fn incoming_edges(&self, vertex: VertexId) -> Result<Vec<(VertexId, Weight)>>;

    /// Creates a new edge.
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectices doesn't exist
    fn create_directed_edge(&mut self, from: VertexId, to: VertexId, weight: Weight) -> Result<()>;

    /// Deletes an edge.
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectors doesn't exist
    fn delete_directed_edge(&mut self, from: VertexId, to: VertexId) -> Result<()>;
}
pub trait UndirectionedGraph<T>: Graph<T> {
    /// Creates a new edge.
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectices doesn't exist
    fn create_undirected_edge(
        &mut self,
        from: VertexId,
        to: VertexId,
        weight: Weight,
    ) -> Result<()>;

    /// Deletes an edge.
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectors doesn't exist
    fn delete_undirected_edge(&mut self, from: VertexId, to: VertexId) -> Result<()>;
}
