pub use graph::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct VertexId(usize);

pub struct EdgeList<T> {
    vertices: HashMap<VertexId, Option<T>>,
    edges: HashMap<VertexId, HashMap<VertexId, Weight>>,
    graph_type: GraphType,
    vertice_next_id: usize,
}

impl<T> Graph<T> for EdgeList<T> {
    type Vertex = VertexId;

    fn new(graph_type: GraphType) -> Self {
        EdgeList {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            graph_type,
            vertice_next_id: 0,
        }
    }

    fn graph_type(&self) -> GraphType {
        self.graph_type
    }

    fn vertices(&self) -> Vec<Self::Vertex> {
        use std::collections::hash_map::Keys;
        let keys: Keys<VertexId, _> = self.vertices.keys();
        let collected: Vec<_> = keys.cloned().collect();
        collected
    }

    fn get_weight(&self, from: Self::Vertex, to: Self::Vertex) -> Result<Weight> {
        if !self.vertices.contains_key(&from) || !self.vertices.contains_key(&to) {
            return Err(GraphError::InvalidVertex)
        }
        Ok(self.edges.get(&from).and_then(|neighbours| neighbours.get(&to).map(|w| *w)).unwrap_or_default())
    }
    fn create_vertex(&mut self) -> Self::Vertex {
        let new_id = VertexId(self.vertice_next_id);
        self.vertice_next_id += 1;
        self.vertices.insert(new_id, None);
        new_id
    }

    fn delete_vertex(&mut self, vertex: Self::Vertex) -> Result<()> {
        self.vertices.remove(&vertex).ok_or(GraphError::InvalidVertex).map(|_| ())
    }
    fn _create_edge_directed(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()> {
        let neighbours: &mut HashMap<VertexId, Weight> = self.edges.entry(from).or_default();
        let edge: &mut Weight = neighbours.entry(to).or_default();
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
        self.edges.get_mut(&from).and_then(|neighbours| neighbours.remove(&to));
        if let GraphType::Undirected = self.graph_type() {
            self.edges.get_mut(&to).and_then(|neighbours| neighbours.remove(&from));
        }
        Ok(())
    }
    fn set_data(&mut self, vertex: Self::Vertex, data: T) -> Result<()> {
        *self.vertices.entry(vertex).or_default() = Some(data);
        Ok(())
    }
    fn get_data(&self, vertex: Self::Vertex) -> Result<Option<&T>> {
        self.vertices.get(&vertex).ok_or(GraphError::InvalidVertex).map(|e| e.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creation_and_empty_graph() {
        let g: EdgeList<()> = EdgeList::new(GraphType::Undirected);
        assert_eq!(g.vertices(), Vec::new());
    }
    #[test]
    fn vertices() {
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Undirected);
        let mut verts = Vec::new();
        for _ in 0..5 {
            verts.push(g.create_vertex());
        }
        let mut g_verts = g.vertices();
        g_verts.sort_unstable_by_key(|v: &VertexId| v.0);
        assert_eq!(verts, g_verts);
    }
    #[test]
    fn get_weight_no_edge() {
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
    }
    #[test]
    fn get_weight_directed() {
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        //? Not equal because directed Graph
        assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    }
    #[test]
    fn get_weight_undirected() {
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Undirected);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        //? Equal because undirected Graph
        assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    }
    #[test]
    fn delete_edge_directed() {
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Directed);
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
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Undirected);
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
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Undirected);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        g.delete_vertex(v1).unwrap();
        assert_eq!(g.get_weight(v1, v2), Err(GraphError::InvalidVertex));
    }
    #[test]
    fn out_of_bounds() {
        let mut g: EdgeList<()> = EdgeList::new(GraphType::Undirected);
        let _ = g.create_vertex(); // 0
        let v1 = g.create_vertex(); // 1
        assert_eq!(g.get_weight(v1, VertexId(2)), Err(GraphError::InvalidVertex));
    }
}