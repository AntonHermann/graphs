#![allow(dead_code)]

use adj_list::AdjList;
use graph::*;

macro_rules! make_test {
    (($($x:ident),*) fn $name:ident($g:ident<$type:ty>) $t:tt) => {
        #[test]
        fn $name() {
            $(
            {
                #[allow(unused_mut)]
                let mut $g: $x<$type> = $x::new();
                $t
            }
            )*
        }
    };
    (fn $name:ident($g:ident<$type:ty>) $t:tt) => {
        // make_test!((AdjList, AdjMatrix, EdgeList) fn $name($g<$type>) $t);
        make_test!((AdjList) fn $name($g<$type>) $t);
    };
    (($($x:ident),*) fn $name:ident($g:ident) $t:tt) => {
        make_test!(($($x),*) fn $name($g<()>) $t);
    };
    (fn $name:ident($g:ident) $t:tt) => {
        make_test!(fn $name($g<()>) $t);
    };
}

make_test!(
fn empty(g) {
    assert_eq!(g.vertices(), Vec::new(), "vertices");
    assert_eq!(g.edges(), Vec::new(), "edges");
}
);
make_test!(
fn vertices(g) {
    let mut verts = Vec::new();
    for _ in 0..5 {
        verts.push(g.create_vertex(None));
    }
    let mut g_verts = g.vertices();
    g_verts.sort_unstable_by_key(|v: &VertexId| v.0);
    assert_eq!(verts, g_verts);
}
);
make_test!(
fn vertices_2(g) {
    let verts = g.create_vertices(vec![None; 5]);
    let mut g_verts = g.vertices();
    g_verts.sort_unstable_by_key(|v: &VertexId| v.0);
    assert_eq!(verts, g_verts);
}
);
make_test!(
fn get_weight_no_edge(g) {
    let v1 = g.create_vertex(None);
    let v2 = g.create_vertex(None);
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity);
}
);
make_test!((AdjList)
fn get_weight_directed(g) {
    let v1 = g.create_vertex(None);
    let v2 = g.create_vertex(None);
    g.create_directed_edge(v1, v2, Weight::W(5)).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
    //? Not equal because directed Graph
    assert_ne!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
}
);
make_test!((AdjList)
fn get_weight_undirected(g) {
    let v1 = g.create_vertex(None);
    let v2 = g.create_vertex(None);
    g.create_undirected_edge(v1, v2, Weight::W(5)).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5));
    //? Equal because undirected Graph
    assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5));
}
);
make_test!((AdjList)
fn delete_edge_directed(g) {
    let from = g.create_vertex(None);
    let to   = g.create_vertex(None);
    g.create_directed_edge(from, to, Weight::W(5)).unwrap();
    // the edge we just created
    assert_eq!(g.get_weight(from, to).unwrap(), Weight::W(5), "edge creation failed");
    // shouldn't exist, since it's a directed edge
    assert_ne!(g.get_weight(to, from).unwrap(), Weight::W(5), "edge should be directed, but isn't");
    g.delete_directed_edge(from, to).unwrap();
    // edge should be removed again, no edge <=> Infinity
    assert_eq!(g.get_weight(from, to).unwrap(), Weight::Infinity, "edge removal failed");
    assert_eq!(g.get_weight(to, from).unwrap(), Weight::Infinity);
}
);
make_test!((AdjList)
fn delete_edge_undirected(g) {
    let v1 = g.create_vertex(None);
    let v2 = g.create_vertex(None);
    g.create_undirected_edge(v1, v2, Weight::W(5)).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::W(5), "edge creation failed");
    assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::W(5), "inverse edge wasn't created");
    g.delete_undirected_edge(v1, v2).unwrap();
    assert_eq!(g.get_weight(v1, v2).unwrap(), Weight::Infinity, "edge removal failed");
    assert_eq!(g.get_weight(v2, v1).unwrap(), Weight::Infinity, "inverse edge wasn't removed");
}
);
make_test!((AdjList)
fn delete_vertex(g) {
    let v1 = g.create_vertex(None);
    let v2 = g.create_vertex(None);
    g.create_undirected_edge(v1, v2, Weight::W(5)).unwrap();
    g.delete_vertex(v1).unwrap();
    assert_eq!(g.get_weight(v1, v2), Err(GraphError::InvalidVertex));
}
);
make_test!(
fn out_of_bounds(g) {
    let _ = g.create_vertex(None); // 0
    let v1 = g.create_vertex(None); // 1
    assert_eq!(g.get_weight(v1, VertexId(2)), Err(GraphError::InvalidVertex));
}
);
