//! A graph API offering a powerful graph trait that allows for the construction of
//! many types of graphs for different use-cases. Implements powerful parallel traversal
//! algorithms offering speedups when traversing the graph.
//!
//! # Example Usage
//!
//! ```
//! use fastgraph::core::{Empty, Traverse};
//! use fastgraph::collections::*;
//!
//! fn main() {
//! 	let mut g = Digraph::<usize, Empty, Empty>::new();
//!
//! 	g.add_node(1, Empty);
//! 	g.add_node(2, Empty);
//! 	g.add_node(3, Empty);
//! 	g.add_node(4, Empty);
//! 	g.add_node(5, Empty);
//! 	g.add_node(6, Empty);
//!
//! 	g.add_edge(1, 2, Empty);
//! 	g.add_edge(1, 3, Empty);
//! 	g.add_edge(2, 1, Empty);
//! 	g.add_edge(2, 3, Empty);
//! 	g.add_edge(3, 1, Empty);
//! 	g.add_edge(3, 5, Empty);
//! 	g.add_edge(5, 2, Empty);
//! 	g.add_edge(5, 4, Empty);
//! 	g.add_edge(5, 1, Empty);
//! 	g.add_edge(4, 5, Empty);
//! 	g.add_edge(4, 3, Empty);
//! 	g.add_edge(4, 2, Empty);
//! 	g.add_edge(4, 6, Empty);
//!
//! 	let sink = g.get_node(6).unwrap();
//! 	let shortest_tree = g.par_breadth_first(1,
//! 		|edge|{
//! 			if edge.target() == sink {
//! 				Traverse::Finish
//! 			} else {
//! 				Traverse::Include
//! 			}
//! 		}).unwrap();
//!
//! 	let shortest_path = fastgraph::core::backtrack_edges(&shortest_tree);
//!
//! 	for edge in shortest_path {
//! 		println!("{}", edge.upgrade().unwrap())
//! 	}
//! }
//!
//! ```


pub mod core;
pub mod collections;
pub mod graph;
pub mod dot;
pub mod macros;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::graph::*;
	use Space::*;

	#[derive(Debug, PartialEq, Eq, Clone, std::hash::Hash)]
	enum Space { Galaxy, Star, Planet, BlackHole, Nebula}

	impl std::fmt::Display for Space {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			match self {
				Space::Galaxy => write!(f, "Galaxy"),
				Space::Star => write!(f, "Star"),
				Space::Planet => write!(f, "Planet"),
				Space::BlackHole => write!(f, "Black Hole"),
				Space::Nebula => write!(f, "Nebula"),
			}
		}
	}

	#[test]
	fn basic1() {

		let space = graph_with_params![

			(String, Space) => [f32]

			(format!("Proxima Centauri"), Star) =>
				[
					(format!("Milky Way"), 2873.124),
					(format!("Andromeda"), 3425.24)
				]

			(format!("Milky Way"), Galaxy) =>
				[
					(format!("Proxima Centauri"), 547.135),
					(format!("Andromeda"), 78873.145)
				]

			(format!("Andromeda"), Galaxy) =>
				[
					(format!("Proxima Centauri"), 23.442),
					(format!("TON-512"), 25663.156)
				]

			(format!("Horse Nebula"), Nebula) =>
				[
					(format!("Proxima Centauri"), 46137.124),
					(format!("Milky Way"), 146.312),
					(format!("Andromeda"), 6143.12)
				]

			(format!("TON-512"), BlackHole) =>
				[
					(format!("Milky Way"), 134.342),
					(format!("Andromeda"), 24834.1432),
					(format!("Sagittarius A*"), 74313.731)
				]

			(format!("Sagittarius A*"), BlackHole) => []

		];

		println!("\nOriginal graph:\n");
		for node in &space { println!("{}", node.1); }

		let source = space.get(&format!("Proxima Centauri")).unwrap();
		let target = space.get(&format!("Sagittarius A*")).unwrap();

		let result = source.breadth_traversal(|_, t, _| {
			match t == target {
				false => Traverse::Include,
				true => Traverse::Terminate,
			}
		});

		// let mut res = Vec::new();
		// if result.len() == 0 && target != result.last().unwrap().target() {
		// 	println!("Target not found!");
		// }
		// else {
		// 	let w = result.get(result.len() - 1).unwrap();
		// 	res.push(w.clone());
		// 	let mut i = 0;
		// 	for edge in result.iter().rev() {
		// 		let source = res[i].source();
		// 		if edge.target() == source {
		// 			res.push(edge.clone());
		// 			i += 1;
		// 		}
		// 	}
		// 	res.reverse();

		// 	println!("\nBFS results:\n");

		// 	for edge in res {
		// 		println!("{} -> {}", edge.source().id(), edge.target().id());
		// 	}
		// }
	}

	fn basic2() {
		let g1 = graph![
			(usize, &str, usize),
			0 => [1, 2]
			1 => [0, 2]
			2 => [0, 4]
			3 => [0, 1, 3]
			4 => [1, 2, 4, 5]
			5
		];
	}

	#[test]
	fn dot_tests() {

		let space = graph_with_params![

			(&str, Space) => [f32]

			("Proxima Centauri", Star) =>
				[
					("Milky Way", 2873.124),
					("Andromeda", 3425.24)
				]

			("Milky Way", Galaxy) =>
				[
					("Proxima Centauri", 547.135),
					("Andromeda", 78873.145)
				]

			("Andromeda", Galaxy) =>
				[
					("Proxima Centauri", 23.442),
					("TON-512", 25663.156)
				]

		];

		dot::to_dot_directed(space);
	}

	#[test]
	fn lol ()
	{
		use rayon::prelude::*;
		use std::sync::Arc;
		use rayon::iter::IntoParallelRefIterator;
		struct Foo<T: Send + Sync + Sized> {
			data: T,
			func: Arc<dyn Fn (T) -> T>,
		}

		unsafe impl<T: Send + Sync + Sized> Sync for Foo<T> {}

		let foos: Vec<Arc<Foo<i32>>> = Vec::new();

		for _ in 0..10 {
			foos.push(Arc::new(Foo {
				data: 1,
				func: Arc::new(|x| x + 1),
			}));
		}

		let res = foos.into_par_iter().map(|foo| {
			true
		}).collect::<Vec<_>>();
	}
}