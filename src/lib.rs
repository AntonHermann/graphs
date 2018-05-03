//! **graphs** is a graph data sructure library.
//!
//! I wanted to implement some of the concepts from my computer science
//! studies, but it turned out that the project structure wasn't really
//! thought out well and my practical knowledge wasn't sufficient, so
//! I copied the code from [**petgraph**](https://crates.io/crates/petgraph)
//!
//! I plan to rebuild the code while understanding and learning how
//! the author did things, and will then implement some graph
//! algorithms myself. Until then you almost certainly want to look
//! at **petgraph** insted.
#![deny(missing_docs)]

mod graph;

// #[cfg(test)]
mod tests;

pub use self::graph::*;
