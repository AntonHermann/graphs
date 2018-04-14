use graph::*;

pub struct AdjMatrix<T> {
    // Option to allow deletion of vertices
    vertices: Vec<Option<Vec<Weight>>>,
    data: Vec<Option<T>>,
}

impl<T> Graph<T> for AdjMatrix<T> {
    type Vertex = usize;
    fn vertices(&self) -> Vec<Self::Vertex> {
        use std::slice::Iter;
        let iter: Iter<_> = self.vertices.iter();
        let enumerated = iter.enumerate();
        let filtered = enumerated.filter(|(_, val)| val.is_some());
        let mapped = filtered.map(|(i, _)| i);
        let collected: Vec<usize> = mapped.collect();
        collected
    }
    fn get_weight(&self, from: Self::Vertex, to: Self::Vertex) -> Result<Weight> {
        match self.vertices[from] {
            Some(ref weights) => {
                Ok(weights[to])
            },
            None => Err(GraphError::VertexDeleted)
        }
    }
    fn create_vertex(&mut self) -> Self::Vertex {
        use std::iter;
        let old_vertice_count = self.vertices.len();
        let mut new_edges: Vec<Weight> = iter::repeat(Weight::Infinity).take(old_vertice_count + 1).collect();
        new_edges[old_vertice_count] = Weight::W(0);
        self.vertices.push(Some(new_edges));
        old_vertice_count
    }
    fn delete_vertex(&mut self, vertex: Self::Vertex) -> Result<()> {
        if self.vertices[vertex].is_some() {
            self.vertices[vertex] = None;
            Ok(())
        } else {
            Err(GraphError::VertexDeleted)
        }
    }
    fn create_edge(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()> {
        // may fail if `from` is out of bounds
        let maybe_neighbours_or_deleted: &mut Option<Vec<Weight>> =
            self.vertices.get_mut(from).ok_or(GraphError::VertexDeleted)?;
        // may fail if this vertice has been deleted
        let neighbours_or_deleted: &mut Vec<Weight> = maybe_neighbours_or_deleted.as_mut().ok_or(GraphError::VertexDeleted)?;
        // may fail if `to` is out of bounds
        let maybe_edge: Option<&mut Weight> = neighbours_or_deleted.get_mut(to);
        // a mutable reference to the respective edge
        let edge: &mut Weight = maybe_edge.ok_or(GraphError::VertexDeleted)?;
        *edge = weight;
        Ok(())
    }
    fn delete_edge(&mut self, from: Self::Vertex, to: Self::Vertex) -> Result<()> {
        self.create_edge(from, to, Weight::Infinity)
    }
    fn set_data(&mut self, vertex: Self::Vertex, data: T) -> Result<()> {
        let data_if_existent: Option<&mut T> = self.data.get_mut(vertex)
            .ok_or(GraphError::IndexOutOfBound)?.as_mut();
        data_if_existent.map(|dt: &mut T| *dt = data);
        Ok(())
    }
    fn get_data(&self, vertex: Self::Vertex) -> Result<Option<&T>> {
        let data_if_existent: Option<&T> = self.data.get(vertex)
            .ok_or(GraphError::IndexOutOfBound)?.as_ref();
        Ok(data_if_existent)
    }
}