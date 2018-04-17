pub use graph::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct VertexId(usize);

type Data<T> = Option<T>;
type AdjacentVertices = Vec<(VertexId, Weight)>;
type Vertex<T> = (AdjacentVertices, Data<T>);

pub struct AdjList<T> {
    vertices: HashMap<VertexId, Vertex<T>>,
    graph_type: GraphType,
    vertice_next_id: usize,
}

impl<T> Graph<T> for AdjList<T> {
    type Vertex = VertexId;

    fn new(graph_type: GraphType) -> Self {
        AdjList {
            vertices: HashMap::new(),
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
        let vertex: &Vertex<T> = unwrap_vertex!(self.vertices.get(&from));
        if !self.vertices.contains_key(&to) { return Err(GraphError::InvalidVertex) }
        let adj_verts: &AdjacentVertices = &vertex.0;
        let (_, weight) = unwrap_vertex!(adj_verts.iter().find(|(v,_)| v == &to), Ok(Weight::Infinity));
        Ok(*weight)
    }
    fn create_vertex(&mut self) -> Self::Vertex {
        let new_id = VertexId(self.vertice_next_id);
        self.vertice_next_id += 1;
        // self.vertices.insert(new_id, (Vec::new(), None));
        self.vertices.insert(new_id, Default::default());
        new_id
    }

    fn delete_vertex(&mut self, vertex: Self::Vertex) -> Result<()> {
        unwrap_vertex!(self.vertices.remove(&vertex)); // removes vector with all outgoing edges
        for (vert, _) in self.vertices.values_mut() {
            vert.retain(|(v, _)| v != &vertex); // keep only edges not going to `vertex`
        }
        Ok(())
    }
    fn _create_edge_directed(&mut self, from: Self::Vertex, to: Self::Vertex, weight: Weight) -> Result<()> {
        let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(&from));
        let adj_verts: &mut AdjacentVertices = &mut vertex.0;
        if let Some((_, ref mut w)) = adj_verts.iter_mut().find(|(v, _)| v == &to) {
            *w = weight;
            return Ok(());
        }
        adj_verts.push((to, weight));
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
    fn _delete_edge_directed(&mut self, from: Self::Vertex, to: Self::Vertex) -> Result<()> {
        let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(&from));
        let adj_verts: &mut AdjacentVertices = &mut vertex.0;
        adj_verts.retain(|(v, _)| v != &to); // keep only edges not going to `to`
        Ok(())
    }
    fn delete_edge(&mut self, from: Self::Vertex, to: Self::Vertex) -> Result<()> {
        let res1 = self._delete_edge_directed(from, to);
        match self.graph_type() {
            GraphType::Directed => res1,
            GraphType::Undirected => {
                res1.and_then(|_| self._delete_edge_directed(to, from))
            }
        }
    }
    fn set_data(&mut self, vertex: Self::Vertex, data: T) -> Result<()> {
        let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(&vertex));
        let d: &mut Data<T> = &mut vertex.1;
        *d = Some(data);
        Ok(())
    }
    fn get_data(&self, vertex: Self::Vertex) -> Result<Option<&T>> {
        let vertex: &Vertex<T> = unwrap_vertex!(self.vertices.get(&vertex));
        let d: &Data<T> = &vertex.1;
        Ok(d.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creation_and_empty_graph() {
        let g: AdjList<()> = AdjList::new(GraphType::Undirected);
        assert_eq!(g.vertices(), Vec::new());
    }
    #[test]
    fn vertices() {
        let mut g: AdjList<()> = AdjList::new(GraphType::Undirected);
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
        let mut g: AdjList<()> = AdjList::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
    }
    #[test]
    fn get_weight_directed() {
        let mut g: AdjList<()> = AdjList::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        //? Not equal because directed Graph
        assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    }
    #[test]
    fn get_weight_undirected() {
        let mut g: AdjList<()> = AdjList::new(GraphType::Undirected);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        //? Equal because undirected Graph
        assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    }
    #[test]
    fn delete_edge_directed() {
        let mut g: AdjList<()> = AdjList::new(GraphType::Directed);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).expect("1");
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
        assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
        g.delete_edge(v1, v2).expect("2");
        assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
        assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::Infinity);
    }
    #[test]
    fn delete_edge_undirected() {
        let mut g: AdjList<()> = AdjList::new(GraphType::Undirected);
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
        let mut g: AdjList<()> = AdjList::new(GraphType::Undirected);
        let v1 = g.create_vertex();
        let v2 = g.create_vertex();
        g.create_edge(v1, v2, Weight::W(5)).unwrap();
        g.delete_vertex(v1).unwrap();
        assert_eq!(g.get_weight(v1, v2), Err(GraphError::InvalidVertex));
    }
    #[test]
    fn out_of_bounds() {
        let mut g: AdjList<()> = AdjList::new(GraphType::Undirected);
        let _ = g.create_vertex(); // 0
        let v1 = g.create_vertex(); // 1
        assert_eq!(g.get_weight(v1, VertexId(2)), Err(GraphError::InvalidVertex));
    }
}