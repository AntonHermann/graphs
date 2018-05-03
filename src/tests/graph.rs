#![allow(unused_imports)]

use super::Direction::{Incoming, Outgoing};
use super::*;

#[cfg(test)]
macro_rules! empty_graph {
    ($n:ty, $e:ty) => {{
        let dg: DiGraph<$n, $e> = Graph::new();
        let ug: UnGraph<$n, $e> = Graph::new_undirected();
        (dg, ug)
    }};
    () => {
        empty_graph!(&str, usize)
    };
}

#[cfg(test)]
macro_rules! test_func {
    ($gs:ident => $(.$f:ident($($param:expr ),*))* == $expected:expr) => {
        assert_eq!(
            ($gs.0)$(.$f($($param ),*))*,
            $expected,
            "dir {} != {}", stringify!($(.$f($($param ),*))*), stringify!($expected)
        );
        assert_eq!(
            ($gs.1)$(.$f($($param ),*))*,
            $expected,
            "und {} != {}", stringify!($(.$f($($param ),*))*), stringify!($expected)
        );
    };
    (;new => $(.$f:ident($($param:expr ),*))* == $expected:expr) => {
        let gs = empty_graph!();
        test_func!(gs => $(.$f($($param ),*))* == $expected);
    };
    (;new_mut => $(.$f:ident($($param:expr ),*))* == $expected:expr) => {
        let mut gs = empty_graph!();
        test_func!(gs => $(.$f($($param ),*))* == $expected);
    };
}

#[cfg(test)]
macro_rules! apply_both {
    ($gs:ident => $(.$f:ident($($param:expr ),*))*) => {{
        let dr = ($gs.0)$(.$f($($param ),*))*;
        let ur = ($gs.1)$(.$f($($param ),*))*;
        (dr, ur)
    }};
}

#[test]
fn direction() {
    assert_eq!(Direction::opposite(&Incoming), Outgoing);
    assert_eq!(Direction::opposite(&Outgoing), Incoming);
    assert_eq!(Direction::index(&Incoming), 1);
    assert_eq!(Direction::index(&Outgoing), 0);
}

#[test]
fn edge_type() {
    assert_eq!(Directed::is_directed(), true);
    assert_eq!(Undirected::is_directed(), false);
}

#[test]
fn into_weighted_edge() {
    assert_eq!((0, 0).into_weighted_edge(), (0, 0, usize::default()));
    assert_eq!((0, 0, 5).into_weighted_edge(), (0, 0, 5));
    assert_eq!(&(0, 0).into_weighted_edge(), &(0, 0, usize::default()));
    assert_eq!(&(0, 0, 5).into_weighted_edge(), &(0, 0, 5));
}

#[test]
fn index_type() {
    assert_eq!(<usize as IndexType>::new(3), 3usize);
    assert_eq!(<usize as IndexType>::index(&3), 3usize);
    assert_eq!(<usize as IndexType>::max(), ::std::usize::MAX);
    assert_eq!(<u32 as IndexType>::new(3), 3u32);
    assert_eq!(<u32 as IndexType>::index(&3), 3usize);
    assert_eq!(<u32 as IndexType>::max(), ::std::u32::MAX);
    assert_eq!(<u16 as IndexType>::new(3), 3u16);
    assert_eq!(<u16 as IndexType>::index(&3), 3usize);
    assert_eq!(<u16 as IndexType>::max(), ::std::u16::MAX);
    assert_eq!(<u8 as IndexType>::new(3), 3u8);
    assert_eq!(<u8 as IndexType>::index(&3), 3usize);
    assert_eq!(<u8 as IndexType>::max(), ::std::u8::MAX);
}

#[test]
fn node_and_edge_index() {
    assert_eq!(NodeIndex::<usize>::new(5).index(), 5);
    assert_eq!(NodeIndex::<usize>::end().index(), ::std::usize::MAX);
    assert_eq!(EdgeIndex::<usize>::new(5).index(), 5);
    assert_eq!(EdgeIndex::<usize>::end().index(), ::std::usize::MAX);
}

#[test]
fn empty_graph() {
    let gs = {
        let dg: DiGraph<&str, usize> = Graph::new();
        let ug: UnGraph<&str, usize> = Graph::new_undirected();
        (dg, ug)
    };
    test_func!(gs => .node_count() == 0);
    test_func!(gs => .edge_count() == 0);
    assert_eq!(gs.0.is_directed(), true);
    assert_eq!(gs.1.is_directed(), false);
    test_func!(gs => .node_data(NodeIndex::new(0)) == None);
    test_func!(gs => .edge_weight(EdgeIndex::new(0)) == None);
    test_func!(gs => .edge_endpoints(EdgeIndex::new(0)) == None);
    test_func!(gs => .neighbors(NodeIndex::new(0)).count() == 0);
    test_func!(gs => .neighbors_directed(NodeIndex::new(0), Incoming).count() == 0);
    test_func!(gs => .neighbors_undirected(NodeIndex::new(0)).count() == 0);
    test_func!(gs => .edges(NodeIndex::new(0)).count() == 0);
    test_func!(gs => .edges_directed(NodeIndex::new(0), Incoming).count() == 0);
    test_func!(gs => .edges_undirected(NodeIndex::new(0)).count() == 0);
    test_func!(gs => .contains_edge(NodeIndex::new(0), NodeIndex::new(3)) == false);
    test_func!(gs => .find_edge(NodeIndex::new(0), NodeIndex::new(3)) == None);
    test_func!(gs => .find_edge_undirected(NodeIndex::new(0), NodeIndex::new(3)) == None);
    test_func!(gs => .externals(Incoming).count() == 0);
    test_func!(gs => .externals(Outgoing).count() == 0);
    test_func!(gs => .raw_nodes().is_empty() == true);
    test_func!(gs => .raw_edges().is_empty() == true);
    test_func!(gs => .first_edge(NodeIndex::new(0), Outgoing) == None);
    test_func!(gs => .first_edge(NodeIndex::new(0), Incoming) == None);
    test_func!(gs => .next_edge(NodeIndex::new(0), Outgoing) == None);
    test_func!(gs => .next_edge(NodeIndex::new(0), Incoming) == None);
}

#[test]
fn nodes() {
    let mut gs = empty_graph!();
    test_func!(gs => .node_count() == 0);
    let na = NodeIndex::new(0);
    test_func!(gs => .add_node("a") == na);
    test_func!(gs => .node_count() == 1);
    test_func!(gs => .node_data(na) == Some(&"a"));
    gs.0.node_data_mut(na).map(|d| *d = "a*");
    gs.1.node_data_mut(na).map(|d| *d = "a*");
    test_func!(gs => .node_data(na) == Some(&"a*"));
    let nb = NodeIndex::new(1);
    test_func!(gs => .add_node("b") == nb);
    test_func!(gs => .node_count() == 2);
    test_func!(gs => .node_data(nb) == Some(&"b"));
    test_func!(gs => .remove_node(nb) == Some("b"));
    test_func!(gs => .node_count() == 1);
    test_func!(gs => .externals(Outgoing).next() == Some(na));
}

#[test]
fn edges() {
    let mut gs = empty_graph!();
    let (na, _) = apply_both!(gs => .add_node("a"));
    let (nb, _) = apply_both!(gs => .add_node("b"));
    let (nc, _) = apply_both!(gs => .add_node("c"));
    test_func!(gs => .edge_count() == 0);
    let e1 = EdgeIndex::new(0);
    test_func!(gs => .add_edge(na, nb, 5) == e1);
    // a -> b
    test_func!(gs => .edge_count() == 1);
    test_func!(gs => .edge_weight(e1) == Some(&5));
    test_func!(gs => .update_edge(na, nb, 7) == e1);
    test_func!(gs => .edge_count() == 1);
    test_func!(gs => .edge_weight(e1) == Some(&7));
    let e2 = EdgeIndex::new(1);
    test_func!(gs => .update_edge(nb, nc, 9) == e2);
    // b -> c
    test_func!(gs => .edge_count() == 2);
    test_func!(gs => .edge_weight(e2) == Some(&9));
    apply_both!(gs => .edge_weight_mut(e2).map(|w| *w = 10));
    test_func!(gs => .edge_weight(e2) == Some(&10));
    test_func!(gs => .edge_endpoints(e2) == Some((nb, nc)));
    test_func!(gs => .contains_edge(nb, nc) == true);
    assert_eq!(gs.0.contains_edge(nc, nb), false);
    assert_eq!(gs.1.contains_edge(nc, nb), true );
    test_func!(gs => .find_edge(nb, nc) == Some(e2));
    test_func!(gs => .find_edge_undirected(nc, nb) == Some((e2, Incoming)));
    assert_eq!(gs.0.source_nodes().collect::<Vec<_>>(), vec![na]);
    assert_eq!(gs.0.sink_nodes().collect::<Vec<_>>(), vec![nc]);
    assert_eq!(gs.1.externals(Incoming).count(), 0);
    assert_eq!(gs.1.externals(Outgoing).count(), 0);
}