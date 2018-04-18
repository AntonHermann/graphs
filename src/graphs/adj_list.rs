use graphs::graph::*;
use std::collections::HashMap;

type Data<T> = Option<T>;
type AdjacentVertices = Vec<(VertexId, Weight)>;
type Vertex<T> = (AdjacentVertices, Data<T>);

pub struct AdjList<T> {
    vertices: HashMap<VertexId, Vertex<T>>,
    vertice_next_id: usize,
}

impl<T> Graph<T> for AdjList<T> {
    fn new() -> Self {
        AdjList {
            vertices: HashMap::new(),
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
        let vertex: &Vertex<T> = unwrap_vertex!(self.vertices.get(&from));
        if !self.vertices.contains_key(&to) { return Err(GraphError::InvalidVertex) }
        let adj_verts: &AdjacentVertices = &vertex.0;
        let (_, weight) = unwrap_vertex!(adj_verts.iter().find(|(v,_)| v == &to), Ok(Weight::Infinity));
        Ok(*weight)
    }
    fn create_vertex(&mut self) -> VertexId {
        let new_id = VertexId(self.vertice_next_id);
        self.vertice_next_id += 1;
        // self.vertices.insert(new_id, (Vec::new(), None));
        self.vertices.insert(new_id, Default::default());
        new_id
    }

    fn delete_vertex(&mut self, vertex: VertexId) -> Result<()> {
        unwrap_vertex!(self.vertices.remove(&vertex)); // removes vector with all outgoing edges
        for (vert, _) in self.vertices.values_mut() {
            vert.retain(|(v, _)| v != &vertex); // keep only edges not going to `vertex`
        }
        Ok(())
    }
    fn set_data(&mut self, vertex: VertexId, data: T) -> Result<()> {
        let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(&vertex));
        let d: &mut Data<T> = &mut vertex.1;
        *d = Some(data);
        Ok(())
    }
    fn get_data(&self, vertex: VertexId) -> Result<Option<&T>> {
        let vertex: &Vertex<T> = unwrap_vertex!(self.vertices.get(&vertex));
        let d: &Data<T> = &vertex.1;
        Ok(d.as_ref())
    }
}
impl<T> DirectedGraph<T> for AdjList<T> {
    fn outgoing_edges(&self, vertex: VertexId) -> Result<Vec<(VertexId, Weight)>> {
        let vertex: &Vertex<T> = unwrap_vertex!(self.vertices.get(&vertex));
        let adj: &AdjacentVertices = &vertex.0;
        Ok(adj.clone())
    }
    fn incoming_edges(&self, vertex: VertexId) -> Result<Vec<(VertexId, Weight)>> {
        let is_incoming = |(from, v): (&VertexId, &Vertex<T>)| -> Option<(VertexId, Weight)> {
            let adj: &AdjacentVertices = &v.0;
            // lookup `vertex` in `from`s adjacency list
            let maybe_weight: Option<&Weight> = adj.iter()
                .find(|(to, _)| to == &vertex)
                // if found, map it to its weight
                .map(|(_to, w)| w);
            maybe_weight.map(|weight| (*from, *weight))
        };
        Ok(self.vertices.iter().filter_map(is_incoming).collect())
    }
    fn edges(&self) -> Vec<(VertexId, VertexId, Weight)> {
        self.vertices.iter().flat_map(|(from, v): (&VertexId, &Vertex<T>)| {
            let adj_vertices: &AdjacentVertices = &v.0;
            adj_vertices.iter().map(move |(to, weight): &(VertexId, Weight)| (*from, *to, *weight))
        }).collect()
    }
    fn create_directed_edge(&mut self, from: VertexId, to: VertexId, weight: Weight) -> Result<()> {
        let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(&from));
        let adj_verts: &mut AdjacentVertices = &mut vertex.0;
        if let Some((_, ref mut w)) = adj_verts.iter_mut().find(|(v, _)| v == &to) {
            *w = weight.into();
            return Ok(());
        }
        adj_verts.push((to, weight.into()));
        Ok(())
    }
    fn delete_directed_edge(&mut self, from: VertexId, to: VertexId) -> Result<()> {
        let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(&from));
        let adj_verts: &mut AdjacentVertices = &mut vertex.0;
        adj_verts.retain(|(v, _)| v != &to); // keep only edges not going to `to`
        Ok(())
    }
}
impl<T> UndirectionedGraph<T> for AdjList<T> {
    fn create_undirected_edge(&mut self, v1: VertexId, v2: VertexId, weight: Weight) -> Result<()> {
        let mut ce = move |from: &VertexId, to: &VertexId| -> Result<()> {
            let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(from));
            // let vertex: &mut Vertex<T> = self.vertices.get_mut(from).unwrap();
            let mut adj_verts: &mut AdjacentVertices = &mut vertex.0;
            // update or insert edge
            vector_update(&mut adj_verts, |(v, _)| v == to, (*to, weight));
            Ok(())
        };
        ce(&v1, &v2).and(ce(&v1, &v1))
    }
    fn delete_undirected_edge(&mut self, v1: VertexId, v2: VertexId) -> Result<()> {
        let mut de = move |from: &VertexId, to: &VertexId| -> Result<()> {
            let vertex: &mut Vertex<T> = unwrap_vertex!(self.vertices.get_mut(from));
            let adj_verts: &mut AdjacentVertices = &mut vertex.0;
            adj_verts.retain(|(v, _)| v != to); // keep only edges not going to `to`
            Ok(())
        };
        de(&v1, &v2).and(de(&v2, &v1))
    }
}

fn vector_update<A, P>(vector: &mut Vec<A>, predicate: P, el: A)
    where P: Fn(&A) -> bool,
{
    for v in vector.iter_mut() {
        if predicate(v) {
            *v = el;
            return;
        }
    }
    // not foud: insert
    vector.push(el);
}