use graph::*;
use std::collections::{HashMap, VecDeque};

pub fn bfs<T, G: DirectedGraph<T>>(graph: G, start: VertexId, target: VertexId) -> Option<Vec<VertexId>> {
    let mut besucht = vec![start];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut predecessors: HashMap<VertexId, VertexId> = HashMap::new();

    loop {
        // let el = match queue.pop_front() {
        //     Some(el) => el,
        //     None => return None,
        // };
        let el = queue.pop_front()?;
        if el == target {
            let mut path = Vec::new();
            let mut curr = el;
            loop {
                path.push(curr);
                curr = match predecessors.get(&curr) {
                    Some(pred) => *pred,
                    None => break,
                };
            }
            path.reverse();
            return Some(path)
        }
        for (neighbour, _weight) in graph.outgoing_edges(el).unwrap() {
            if !besucht.contains(&neighbour) {
                besucht.push(neighbour);
                queue.push_back(neighbour);
                predecessors.insert(neighbour, el);
            }
        }
    }
}