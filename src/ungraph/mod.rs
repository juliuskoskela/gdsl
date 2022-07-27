//! # Generic Graph Interface
//!
//! author: Julius Koskela
//!
//! license: MIT
//!
//! This is a generic graph interface.
//!
//! # Examples
//!
//! Create a directed graph with nodes and edges
//!
//! ```
//! use ::digraph::*;
//! use gdsl::ungraph::*;
//!
//! let mut g = Graph::<usize, Empty, Empty>::new();
//!
//! g.insert(node!(0));
//! g.insert(node!(1));
//!
//! connect!(&g[0] => &g[1]);
//! ```
//!
//! Breadth-first Search
//!
//! Djikstra's Algorithm
//!
//! ```
//! use gdsl::ungraph::*;
//! use std::cell::Cell;
//!
//! // Create with the `graph!` macro. Since we want to mutate the distance
//! // of the nodes, we use a `Cell` to wrap it. Edges contains it's length.
//! let g = graph![
//! 	(&str, Cell<u64>) => [u64]
//! 	("A", Cell::new(u64::MAX)) => [ ("B", 4), ("H", 8) ]
//! 	("B", Cell::new(u64::MAX)) => [ ("A", 4), ("H", 11), ("C", 8) ]
//! 	("C", Cell::new(u64::MAX)) => [ ("B", 8), ("C", 2), ("F", 4), ("D", 7) ]
//! 	("D", Cell::new(u64::MAX)) => [ ("C", 7), ("F", 14), ("E", 9) ]
//! 	("E", Cell::new(u64::MAX)) => [ ("D", 9), ("F", 10) ]
//! 	("F", Cell::new(u64::MAX)) => [ ("G", 2), ("C", 4), ("D", 14), ("E", 10) ]
//! 	("G", Cell::new(u64::MAX)) => [ ("H", 1), ("I", 6), ("F", 2) ]
//! 	("H", Cell::new(u64::MAX)) => [ ("A", 8), ("B", 11), ("I", 7), ("G", 1) ]
//! 	("I", Cell::new(u64::MAX)) => [ ("H", 7), ("C", 2), ("G", 6) ]
//! ];
//!
//! // We set the distance of the start node to 0.
//! g["A"].set(0);
//!
//! // We run the algorithm using `search` "iterator" and `pfs_min_map` in order to
//! // traverse nodes in minimum priority order.
//! g["A"].search().pfs_min_map(&g["E"], &|u, v, edge_len| {
//!
//! 	// The distances are stored in the nodes inside a `Cell` type so we use `get()`
//! 	// to get the inner value.
//! 	let (dist_u, dist_v) = (u.get(), v.get());
//!
//! 	// We if the distance stored in v is greater than the distance stored
//! 	// in u + edge_len we update the distance stored in v.
//! 	match dist_v > dist_u + edge_len {
//! 		true => {v.set(dist_u + edge_len); true},
//! 		false => false,
//! 	}
//! });
//!
//! // We expect the minimum distance of A -> E to be 21.
//! assert!(g["E"].take() == 21);
//! ```

pub mod graph;
pub mod graph_search;
pub mod node;