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

pub type Result<T> = stdResult<T, GraphError>;
pub trait Graph<T> {
    type Vertex;

    fn new(graph_type: GraphType) -> Self;
    fn graph_type(&self) -> GraphType;
    fn vertices(&self) -> Vec<Self::Vertex>;
    fn get_weight(&self, from: Self::Vertex, to: Self::Vertex) -> Result<Weight>;
    fn create_vertex(&mut self) -> Self::Vertex;
    fn delete_vertex(&mut self, vertex: Self::Vertex) -> Result<()>;
    fn _create_edge_directed(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()>;
    fn create_edge(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()>;
    fn delete_edge(&mut self, from: Self::Vertex, to: Self::Vertex) -> Result<()>;
    fn set_data(&mut self, vertex: Self::Vertex, data: T) -> Result<()>;
    fn get_data(&self, vertex: Self::Vertex) -> Result<Option<&T>>;
}