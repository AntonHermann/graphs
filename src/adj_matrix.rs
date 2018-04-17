pub use graph::*;

struct Vertex<T> {
    data: Option<T>,
    neighbours: Vec<Weight>,
}

pub struct AdjMatrix<T> {
    // Option to allow deletion of vertices
    vertices: Vec<Option<Vertex<T>>>,
    graph_type: GraphType,
}

impl<T> Graph<T> for AdjMatrix<T> {
    type Vertex = usize;

    fn new(graph_type: GraphType) -> Self {
        AdjMatrix {
            vertices: Vec::with_capacity(10),
            graph_type,
        }
    }

    fn graph_type(&self) -> GraphType {
        self.graph_type
    }

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
        let maybe_vertex: Option<&Vertex<T>> = self.vertices.get(from).ok_or(GraphError::InvalidVertex)?.as_ref();
        let vertex: &Vertex<T> = maybe_vertex.ok_or(GraphError::InvalidVertex)?;
        let weight: &Weight = vertex.neighbours.get(to).ok_or(GraphError::InvalidVertex)?;
        Ok(*weight)
    }
    fn create_vertex(&mut self) -> Self::Vertex {
        let new_vertex_id = self.vertices.len();
        // update existing vertices:
        let unreachable_weight = Weight::Infinity;
        for vert in self.vertices.iter_mut() {
            if let Some(v) = vert {
                // vertex not deleted: push new weight
                v.neighbours.push(unreachable_weight);
            }
        }


        // add new vertex:
        let new_vertex = {
            let mut new_edges: Vec<Weight> = vec![Weight::Infinity; new_vertex_id + 1];
            // set last element (new vertex) weight to 0
            new_edges[new_vertex_id] = Weight::W(0);
            Vertex {
                data: None,
                neighbours: new_edges,
            }
        };
        self.vertices.push(Some(new_vertex));
        new_vertex_id
    }
    // TODO: vertices[index] => vertices.get
    fn delete_vertex(&mut self, vertex: Self::Vertex) -> Result<()> {
        let maybe_vertex: &mut Option<Vertex<T>> =
            self.vertices.get_mut(vertex).ok_or(GraphError::InvalidVertex)?;
        *maybe_vertex = None;
        Ok(())
    }
    fn _create_edge_directed(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()> {
        // may fail if `from` is out of bounds
        let maybe_vertex: Option<&mut Vertex<T>> =
            self.vertices.get_mut(from).ok_or(GraphError::InvalidVertex)?.as_mut();
        // may fail if vertex has been deleted
        let vertex: &mut Vertex<T> = maybe_vertex.ok_or(GraphError::InvalidVertex)?;
        let neighbours: &mut Vec<Weight> = &mut vertex.neighbours;

        // may fail if `to` is out of bounds
        let edge: &mut Weight = neighbours.get_mut(to).ok_or(GraphError::InvalidVertex)?;
        *edge = weight;
        Ok(())
    }
    fn create_edge(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()> {
        let res1 = self._create_edge_directed(from, to, weight);
        match self.graph_type() {
            GraphType::Directed => res1,
            GraphType::Undirected => {
                res1.and_then(|_| self._create_edge_directed(to, from, weight))
            }
        }
    }
    fn delete_edge(&mut self, from: Self::Vertex, to: Self::Vertex) -> Result<()> {
        self.create_edge(from, to, Weight::Infinity)
    }
    fn set_data(&mut self, vertex: Self::Vertex, data: T) -> Result<()> {
        let vertex_if_existent: Option<&mut Vertex<T>> = self.vertices.get_mut(vertex)
            .ok_or(GraphError::InvalidVertex)?.as_mut();
        let vertex: &mut Vertex<T> = vertex_if_existent.ok_or(GraphError::InvalidVertex)?;
        let data_if_existent: Option<&mut T> = vertex.data.as_mut();
        data_if_existent.map(|dt: &mut T| *dt = data);
        Ok(())
    }
    fn get_data(&self, vertex: Self::Vertex) -> Result<Option<&T>> {
        let vertex_if_existent: Option<&Vertex<T>> = self.vertices.get(vertex)
            .ok_or(GraphError::InvalidVertex)?.as_ref();
        let vertex: &Vertex<T> = vertex_if_existent.ok_or(GraphError::InvalidVertex)?;
        Ok(vertex.data.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creation_and_empty_graph() {
        let g: AdjMatrix<()> = AdjMatrix::new(GraphType::Undirected);
        assert_eq!(g.vertices(), Vec::new());
    }
    #[test]
    fn vertices() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Undirected);
        let mut verts = Vec::new();
        for _ in 0..5 {
            verts.push(g.create_vertex());
        }
        assert_eq!(verts, g.vertices());
    }
    #[test]
    fn get_weight_no_edge() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
    }
    #[test]
    fn get_weight_directed() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        //? Not equal because directed Graph
        assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    }
    #[test]
    fn get_weight_undirected() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Undirected);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        //? Equal because undirected Graph
        assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    }
    #[test]
    fn delete_edge_directed() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
        g.delete_edge(v1, v2).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
        assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::Infinity);
    }
    #[test]
    fn delete_edge_undirected() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Undirected);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
        g.delete_edge(v1, v2).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
        assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::Infinity);
    }
    #[test]
    fn delete_vertex() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Undirected);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        g.delete_vertex(v1).unwrap();
        assert_eq!(g.get_weight(v1, v2), Err(GraphError::InvalidVertex));
    }
    #[test]
    fn out_of_bounds() {
        let mut g: AdjMatrix<()> = AdjMatrix::new(GraphType::Undirected);
        let _ = g.create_vertex(); // 0
        let v1 = g.create_vertex(); // 1
        assert_eq!(g.get_weight(v1, 2), Err(GraphError::InvalidVertex));
    }
}