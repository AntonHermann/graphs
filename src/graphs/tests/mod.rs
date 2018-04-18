#![allow(dead_code)]

use graphs::*;


// macro_rules! make_test {
//     (($($x:ident),*), fn $name:ident($g:ident) $t:tt) => {
//         #[test]
//         fn $name() {
//             {
//                 let mut $g: AdjList<()> = AdjList::new();
//                 $t
//             }
//             {
//                 let mut $g: AdjMatrix<()> = AdjMatrix::new();
//                 $t
//             }
//             {
//                 let mut $g: EdgeList<()> = EdgeList::new();
//                 $t
//             }
//         }
//     };
// }
macro_rules! make_test {
    (($($x:ident),*) fn $name:ident($g:ident) $t:tt) => {
        #[test]
        fn $name() {
            $(
            {
                let mut $g: $x<()> = $x::new();
                $t
            }
            )*
        }
    };
    (fn $name:ident($g:ident) $t:tt) => {
        make_test!((AdjList, AdjMatrix, EdgeList) fn $name($g) $t);
    }
}

// multi_test!(creation_and_empty_graph, creation_and_empty_graph_test);
// pub fn creation_and_empty_graph<G: Graph<()>>(graph: G) {
//     assert_eq!(graph.vertices(), Vec::new());
// }
make_test!(
fn vertices(g) {
    let mut verts = Vec::new();
    for _ in 0..5 {
        verts.push(g.create_vertex());
    }
    let mut g_verts = g.vertices();
    g_verts.sort_unstable_by_key(|v: &VertexId| v.0);
    assert_eq!(verts, g_verts);
}
);
make_test!(
fn get_weight_no_edge(g) {
    let v1 = g.create_vertex();
    let v2 = g.create_vertex();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
}
);
make_test!((AdjList)
fn get_weight_directed(g) {
    let v1 = g.create_vertex();
    let v2 = g.create_vertex();
    g.create_directed_edge(v1, v2, Weight::W(5)).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
    //? Not equal because directed Graph
    assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
}
);
make_test!((AdjList)
fn get_weight_undirected(g) {
    let v1 = g.create_vertex();
    let v2 = g.create_vertex();
    g.create_undirected_edge(v1, v2, Weight::W(5)).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
    //? Equal because undirected Graph
    assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
}
);
make_test!((AdjList)
fn delete_edge_directed(g) {
    let v1 = g.create_vertex();
    let v2 = g.create_vertex();
    g.create_directed_edge(v1, v2, Weight::W(5)).expect("1");
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
    assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    g.delete_directed_edge(v1, v2).expect("2");
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
    assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::Infinity);
}
);
make_test!((AdjList)
fn delete_edge_undirected(g) {
    let v1 = g.create_vertex();
    let v2 = g.create_vertex();
    g.create_undirected_edge(v1, v2, Weight::W(5)).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
    assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
    g.delete_undirected_edge(v1, v2).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
    assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::Infinity);
}
);
make_test!((AdjList)
fn delete_vertex(g) {
    let v1 = g.create_vertex();
    let v2 = g.create_vertex();
    g.create_undirected_edge(v1, v2, Weight::W(5)).unwrap();
    g.delete_vertex(v1).unwrap();
    assert_eq!(g.get_weight(v1, v2), Err(GraphError::InvalidVertex));
}
);
make_test!(
fn out_of_bounds(g) {
    let _ = g.create_vertex(); // 0
    let v1 = g.create_vertex(); // 1
    assert_eq!(g.get_weight(v1, VertexId(2)), Err(GraphError::InvalidVertex));
}
);