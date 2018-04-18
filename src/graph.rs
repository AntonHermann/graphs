use std::result::Result as stdResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weight {
    Infinity,
    W(usize),
}
impl Default for Weight {
    fn default() -> Self { Weight::Infinity }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphType {
    Directed,
    Undirected,
}

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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct VertexId(pub usize);

pub type Result<T> = stdResult<T, GraphError>;

pub trait Graph<T> {
    fn new(graph_type: GraphType) -> Self;
    fn graph_type(&self) -> GraphType;
    fn vertices(&self) -> Vec<VertexId>;
    fn get_weight(&self, from: VertexId, to: VertexId) -> Result<Weight>;
    fn create_vertex(&mut self) -> VertexId;
    fn delete_vertex(&mut self, vertex: VertexId) -> Result<()>;
    fn _create_edge_directed(&mut self, from: VertexId, to: VertexId, weight: Weight) -> Result<()>;
    fn create_edge(&mut self, from: VertexId, to: VertexId, weight: Weight) -> Result<()>;
    fn _delete_edge_directed(&mut self, from: VertexId, to: VertexId) -> Result<()>;
    fn delete_edge(&mut self, from: VertexId, to: VertexId) -> Result<()>;
    fn set_data(&mut self, vertex: VertexId, data: T) -> Result<()>;
    fn get_data(&self, vertex: VertexId) -> Result<Option<&T>>;
}