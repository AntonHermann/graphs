use graphs::graph::*;

#[derive(Clone)]
struct Vertex<T> {
    data: Option<T>,
    neighbours: Vec<Weight>,
}

pub struct AdjMatrix<T> {
    // Option to allow deletion of vertices
    vertices: Vec<Option<Vertex<T>>>,
}

impl<T> AdjMatrix<T> {
    pub fn new() -> Self {
        AdjMatrix {
            vertices: Vec::with_capacity(10),
        }
    }
}

impl<T> Graph<T> for AdjMatrix<T> {
    fn vertices(&self) -> Vec<VertexId> {
        use std::slice::Iter;
        let iter: Iter<_> = self.vertices.iter();
        let enumerated = iter.enumerate();
        let filtered = enumerated.filter(|(_, val)| val.is_some());
        let mapped = filtered.map(|(i, _)| VertexId(i));
        let collected: Vec<VertexId> = mapped.collect();
        collected
    }

    fn edges(&self) -> Vec<(VertexId, VertexId, Weight)> {
        self.vertices.iter().enumerate().filter_map(|(from, maybe_vertex)|{
            maybe_vertex.as_ref().map(|vertex| (from, vertex))
        }).flat_map(|(from, vertex): (usize, &Vertex<T>)| {
            let neighbours: &Vec<Weight> = &vertex.neighbours;
            neighbours.iter().enumerate().map(move |(to, weight)| {
                (VertexId(from), VertexId(to), *weight)
            })
        }).collect()
    }

    fn get_weight(&self, from: VertexId, to: VertexId) -> Result<Weight> {
        let maybe_vertex: Option<&Vertex<T>> = self.vertices
            .get(from.0)
            .ok_or(GraphError::InvalidVertex)?
            .as_ref();
        let vertex: &Vertex<T> = maybe_vertex.ok_or(GraphError::InvalidVertex)?;
        let weight: &Weight = vertex
            .neighbours
            .get(to.0)
            .ok_or(GraphError::InvalidVertex)?;
        Ok(*weight)
    }
    fn create_vertex(&mut self) -> VertexId {
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
        VertexId(new_vertex_id)
    }
    fn delete_vertex(&mut self, vertex: VertexId) -> Result<()> {
        let maybe_vertex: &mut Option<Vertex<T>> = self.vertices
            .get_mut(vertex.0)
            .ok_or(GraphError::InvalidVertex)?;
        *maybe_vertex = None;
        Ok(())
    }
    fn set_data(&mut self, vertex: VertexId, data: T) -> Result<()> {
        let vertex_if_existent: Option<&mut Vertex<T>> = self.vertices
            .get_mut(vertex.0)
            .ok_or(GraphError::InvalidVertex)?
            .as_mut();
        let vertex: &mut Vertex<T> = vertex_if_existent.ok_or(GraphError::InvalidVertex)?;
        let data_if_existent: Option<&mut T> = vertex.data.as_mut();
        data_if_existent.map(|dt: &mut T| *dt = data);
        Ok(())
    }
    fn get_data(&self, vertex: VertexId) -> Result<Option<&T>> {
        let vertex_if_existent: Option<&Vertex<T>> = self.vertices
            .get(vertex.0)
            .ok_or(GraphError::InvalidVertex)?
            .as_ref();
        let vertex: &Vertex<T> = vertex_if_existent.ok_or(GraphError::InvalidVertex)?;
        Ok(vertex.data.as_ref())
    }
}
// fn _create_edge_directed<W: Into<Weight> + Copy>(&mut self, from: VertexId, to: VertexId, weight: W) -> Result<()> {
//     // may fail if `from` is out of bounds
//     let maybe_vertex: Option<&mut Vertex<T>> =
//         self.vertices.get_mut(from.0).ok_or(GraphError::InvalidVertex)?.as_mut();
//     // may fail if vertex has been deleted
//     let vertex: &mut Vertex<T> = maybe_vertex.ok_or(GraphError::InvalidVertex)?;
//     let neighbours: &mut Vec<Weight> = &mut vertex.neighbours;

//     // may fail if `to` is out of bounds
//     let edge: &mut Weight = neighbours.get_mut(to.0).ok_or(GraphError::InvalidVertex)?;
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
//     self._create_edge_directed(from, to, Weight::Infinity)
// }
// fn delete_edge(&mut self, from: VertexId, to: VertexId) -> Result<()> {
//     self.create_edge(from, to, Weight::Infinity)
// }
