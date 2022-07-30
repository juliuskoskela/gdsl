//! # Generic Graph Interface
//!
//! This is a generic graph interface.
//!
//! # Examples
//!
//! Create a directed graph with nodes and edges
//!
//! ```
//! use ::digraph::*;
//! use gdsl::*;
//!
//! let mut g = DiGraph::<usize, Empty, Empty>::new();
//!
//! g.insert(dinode!(0));
//! g.insert(dinode!(1));
//!
//! connect!(&g[0] => &g[1]);
//! ```
//!
//! ```
//! use ::ungraph::*;
//! use gdsl::*;
//!
//! let mut g = UnGraph::<usize, Empty, Empty>::new();
//!
//! g.insert(unnode!(0));
//! g.insert(unnode!(1));
//!
//! connect!(&g[0] => &g[1]);
//! ```

pub mod graph_macros;
pub mod digraph;
pub mod ungraph;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Empty;

impl std::fmt::Display for Empty {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "_")
    }
}
