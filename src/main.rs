extern crate graphs;

use graphs::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use graphs::Weight::W;
use algorithms::bfs::*;

fn main() {
    let (g, d) = dummy();
    let start = d["Frankfurt"];
    let target = d["München"];
    let res = bfs(g, start, target);
    if let Some(path) = res {
        let d2 = create_reverse_lookup(&d);
        for vert in path {
            print!("{} -> ", d2[&vert]);
        }
    } else {
        println!("NOT FOUND");
    }
}

fn create_reverse_lookup<K, V>(dict: &HashMap<K, V>) -> HashMap<&V, &K>
    where   K: std::hash::Hash + Eq,
            V: std::hash::Hash + Eq
{
    let mut new_dict = HashMap::new();
    for (k, v) in dict.iter() {
        new_dict.insert(v, k);
    }
    new_dict
}

//? Frankfurt Mannheim Würzburg Stuttgart Karlsruhe Erfurt Nürnberg Augsburg München
fn dummy() -> (AdjList<String>, HashMap<&'static str, VertexId>) {
    let mut g: AdjList<String> = AdjList::new();
    let cities = vec![
        "Frankfurt",
        "Mannheim",
        "Würzburg",
        "Stuttgart",
        "Karlsruhe",
        "Erfurt",
        "Nürnberg",
        "Augsburg",
        "München",
        "Kassel",
    ];
    let d: HashMap<&str, VertexId> = HashMap::from_iter(
        cities.iter().cloned().zip(
            g.create_vertices(
                cities
                    .iter().cloned()
                    .map(String::from)
                    .map(Option::from)
                    .collect(),
            ),
        ),
    );
    (|| -> Result<()> {
        g.create_undirected_edge(d["Frankfurt"], d["Mannheim"], W(85))?;
        g.create_undirected_edge(d["Frankfurt"], d["Würzburg"], W(217))?;
        g.create_undirected_edge(d["Frankfurt"], d["Kassel"], W(173))?;
        g.create_undirected_edge(d["Mannheim"], d["Karlsruhe"], W(80))?;
        g.create_undirected_edge(d["Karlsruhe"], d["Augsburg"], W(250))?;
        g.create_undirected_edge(d["Augsburg"], d["München"], W(84))?;
        g.create_undirected_edge(d["Würzburg"], d["Erfurt"], W(186))?;
        g.create_undirected_edge(d["Würzburg"], d["Nürnberg"], W(103))?;
        g.create_undirected_edge(d["Stuttgart"], d["Nürnberg"], W(183))?;
        g.create_undirected_edge(d["Nürnberg"], d["München"], W(167))?;
        g.create_undirected_edge(d["Kassel"], d["München"], W(502))?;
        Ok(())
    })().unwrap();
    (g, d)
}