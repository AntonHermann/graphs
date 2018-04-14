use std::result::Result as stdResult;

#[derive(Debug, Clone, Copy)]
pub enum Weight {
    Infinity,
    W(usize),
}

pub enum GraphError {
    VertexDeleted,
    IndexOutOfBound,
}
pub type Result<T> = stdResult<T, GraphError>;

pub trait Graph<T> {
    type Vertex;
    type Err = GraphError;
    fn vertices(&self) -> Vec<Self::Vertex>;
    fn get_weight(&self, from: Self::Vertex, to: Self::Vertex) -> Result<Weight>;
    fn create_vertex(&mut self) -> Self::Vertex;
    fn delete_vertex(&mut self, vertex: Self::Vertex) -> Result<()>;
    fn create_edge(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()>;
    fn delete_edge(&mut self, from: Self::Vertex, to: Self::Vertex) -> Result<()>;
    fn set_data(&mut self, vertex: Self::Vertex, data: T) -> Result<()>;
    fn get_data(&self, vertex: Self::Vertex) -> Result<Option<&T>>;
}