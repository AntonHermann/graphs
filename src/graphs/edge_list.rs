use graphs::graph::*;
use std::collections::HashMap;

pub struct EdgeList<T> {
    vertices: HashMap<VertexId, Option<T>>,
    edges: HashMap<VertexId, HashMap<VertexId, Weight>>,
    vertice_next_id: usize,
}

impl<T> Graph<T> for EdgeList<T> {
    fn new() -> Self {
        EdgeList {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            vertice_next_id: 0,
        }
    }

    fn vertices(&self) -> Vec<VertexId> {
        use std::collections::hash_map::Keys;
        let keys: Keys<VertexId, _> = self.vertices.keys();
        let collected: Vec<_> = keys.cloned().collect();
        collected
    }

    fn get_weight(&self, from: VertexId, to: VertexId) -> Result<Weight> {
        if !self.vertices.contains_key(&from) || !self.vertices.contains_key(&to) {
            return Err(GraphError::InvalidVertex)
        }
        Ok(self.edges.get(&from).and_then(|neighbours| neighbours.get(&to).map(|w| *w)).unwrap_or_default())
    }
    fn create_vertex(&mut self) -> VertexId {
        let new_id = VertexId(self.vertice_next_id);
        self.vertice_next_id += 1;
        self.vertices.insert(new_id, None);
        new_id
    }

    fn delete_vertex(&mut self, vertex: VertexId) -> Result<()> {
        self.vertices.remove(&vertex).ok_or(GraphError::InvalidVertex).map(|_| ())
    }
    fn set_data(&mut self, vertex: VertexId, data: T) -> Result<()> {
        *self.vertices.entry(vertex).or_insert_with(Default::default) = Some(data);
        Ok(())
    }
    fn get_data(&self, vertex: VertexId) -> Result<Option<&T>> {
        self.vertices.get(&vertex).ok_or(GraphError::InvalidVertex).map(|e| e.as_ref())
    }
}
    // fn _create_edge_directed<W: Into<Weight> + Copy>(&mut self, from: VertexId, to: VertexId, weight: W) -> Result<()> {
    //     let neighbours: &mut HashMap<VertexId, Weight> = self.edges.entry(from).or_insert_with(Default::default);
    //     let edge: &mut Weight = neighbours.entry(to).or_insert_with(Default::default);
    //     *edge = weight.into();
    //     Ok(())
    // }
    // fn create_edge<W: Into<Weight> + Copy>(&mut self, from: VertexId, to: VertexId, weight: W) -> Result<()> {
    //     let res1 = self._create_edge_directed(from, to, weight);
    //     match self.graph_type() {
    //         GraphType::Directed => res1,
    //         GraphType::Undirected => {
    //             res1.and_then(|_| self._create_edge_directed(to, from, weight))
    //         }
    //     }
    // }
    // fn _delete_edge_directed(&mut self, from: VertexId, to: VertexId) -> Result<()> {
    //     self.edges.get_mut(&from).and_then(|neighbours| neighbours.remove(&to));
    //     Ok(())
    // }
    // fn delete_edge(&mut self, from: VertexId, to: VertexId) -> Result<()> {
    //     if let GraphType::Directed = self.graph_type() {
    //         self._delete_edge_directed(from, to)
    //     } else {
    //         self._delete_edge_directed(from, to)?;
    //         self._delete_edge_directed(to, from)
    //     }
    // }