use std::result::Result as stdResult;

/// The weight of an edge
/// Infinity if the edge doesn't exist (yet)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weight {
    Infinity,
    W(usize),
}
impl Default for Weight {
    fn default() -> Self { Weight::Infinity }
}

/// The type of a `Graph`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphType {
    Directed,
    Undirected,
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
    fn new(graph_type: GraphType) -> Self;

    /// Returns the type of the `Graph`.
    ///
    /// Can be either GraphType::Directed or GraphType::Undirected
    fn graph_type(&self) -> GraphType;

    /// Returns a Vector containing all vertices
    fn vertices(&self) -> Vec<VertexId>;

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

    /// Creates a new directed edge.
    /// Prefer using `Graph::create_edge()` since it handles the different graph
    /// types correctly. This method is only an internal helper
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectices doesn't exist
    fn _create_edge_directed(&mut self, from: VertexId, to: VertexId, weight: Weight) -> Result<()>;

    /// Creates a new edge.
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectices doesn't exist
    fn create_edge(&mut self, from: VertexId, to: VertexId, weight: Weight) -> Result<()>;

    /// Creates a directed edge.
    /// Prefer using `Graph::delete_edge()` since it handles the different graph
    /// types correctly. This method is only an internal helper
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectices doesn't exist
    fn _delete_edge_directed(&mut self, from: VertexId, to: VertexId) -> Result<()>;

    /// Deletes an edge.
    ///
    /// Returns Err(GraphError::InvalidVertex) if one of the vectors doesn't exist
    fn delete_edge(&mut self, from: VertexId, to: VertexId) -> Result<()>;

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