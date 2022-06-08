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
pub mod node;

#[macro_export]
macro_rules! node {
	( $key:expr ) => {
        {
            GraphNode::new($key, None)
        }
    };
    ( $key:expr, $param:expr ) => {
        {
            GraphNode::new($key, Some($param))
        }
    };
}

#[macro_export]
macro_rules! graph {

	// s => [e1, e2]
	( ($K:ty, $N:ty, $E:ty), $($NODE:expr $( => [ $( $EDGE:expr),*] )? )* )
	=> {
        {
			use std::collections::BTreeMap;
			let mut edges = Vec::<($K, $K)>::new();
            let mut map = BTreeMap::<$K, GraphNode<$N, $E, $K>>::new();
            $(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE);
                map.insert(n.id().clone(), n);
            )*
			for (s, t) in edges {
				if map.contains_key(&s) && map.contains_key(&t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t);
				}
			}
            map
        }
    };

	// (s, T) => [e1, e2]
	( ($K:ty, $N:ty, $E:ty), $(($NODE:expr, $NPARAM:expr) => [ $( $EDGE:expr),*] )* )
	=> {
        {
			use std::collections::BTreeMap;
			let mut edges = Vec::<($K, $K)>::new();
            let mut map = BTreeMap::<$K, GraphNode<$N, $E, $K>>::new();
            $(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE, $NPARAM);
                map.insert(n.id().clone(), n);
            )*
			for (s, t) in edges {
				if map.contains_key(&s) && map.contains_key(&t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t);
				}
			}
            map
        }
    };

	// s => [(e1, T), (e2, T)]
	( ($K:ty, $N:ty, $E:ty) $(($NODE:expr) $( =>  $( [$EDGE:expr, $EPARAM:expr] ),* )? )* )
	=> {
		{
			use std::collections::BTreeMap;
			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = BTreeMap::<$K, GraphNode<$N, $E, $K>>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE);
				map.insert(n.id().clone(), n);
			)*
			for (s, t, param) in edges {
				if map.contains_key(&s) && map.contains_key(&t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t, Some(param));
				}
			}
			map
		}
	};

	// (s, T) => [(e1, T), (e2, T)]
	( ($K:ty, $N:ty, $E:ty) $(($NODE:expr, $NPARAM:expr) $( =>  $( [$EDGE:expr, $EPARAM:expr] ),* )? )* )
	=> {
		{
			use std::collections::BTreeMap;
			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = BTreeMap::<$K, GraphNode<$N, $E, $K>>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				map.insert(n.id().clone(), n);
			)*
			for (s, t, param) in edges {
				if map.contains_key(&s) && map.contains_key(&t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t, Some(param));
				}
			}
			map
		}
	};
}

#[macro_export]
macro_rules! graph_with_params {
// (s, T) => [(e1, T), (e2, T)]
( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use std::collections::BTreeMap;
			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = BTreeMap::<$K, GraphNode<$N, $E, $K>>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				map.insert(n.id().clone(), n);
			)*
			for (s, t, param) in edges {
				if map.contains_key(&s) && map.contains_key(&t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t, Some(param));
				}
			}
			map
		}
	};
}

#[macro_export]
macro_rules! connect {
	( $s:expr => $t:expr ) => {
        {
            Node::connect($s, $t, None)
        }
    };
    ( $s:expr => $t:expr, $params:expr ) => {
        {
            Node::connect($s, $t, $params)
        }
    };
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::node::*;

	#[test]
	fn it_works() {
		let g1 = graph![
			(usize, &str, usize),
			0 => [1, 2]
			1 => [0, 2]
			2 => [0, 4]
			3 => [0, 1, 3]
			4 => [1, 2, 4, 5]
			5
		];

		#[derive(Debug, PartialEq, Eq, Clone)]
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

		use Space::*;

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

			("Horse Nebula", Nebula) =>
				[
					("Proxima Centauri", 46137.124),
					("Milky Way", 146.312),
					("Andromeda", 6143.12)
				]

			("TON-512", BlackHole) =>
				[
					("Milky Way", 134.342),
					("Andromeda", 24834.1432),
					("Sagittarius A*", 74313.731)
				]

			("Sagittarius A*", BlackHole) => []

		];

		println!("");
		println!("Orginal graph:");
		println!("");

		space.iter().for_each(|(_,node)| {
			println!("{}", node);
		});

		let result = space[&"Proxima Centauri"].breadth_first_search(&space[&"Sagittarius A*"]);

		println!("");
		println!("BFS results:");
		println!("");

		for edge in result.unwrap() {
			println!("{} -> {}", edge.source().id(), edge.target().id());
		}
	}
}