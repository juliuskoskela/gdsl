//! A graph API offering a powerful graph trait that allows for the construction of
//! many types of graphs for different use-cases. Implements powerful parallel traversal
//! algorithms offering speedups when traversing the graph.
//!
//! # Example Usage
//!
//! ```
//! fn main() {
//!		use fastgraph::*;
//!		use fastgraph::enums::{Coll, Sig};
//!		use fastgraph::node_trait::*;
//!
//!		let g = graph![
//!			(usize)
//!			(0) => [1, 2]
//!			(1) => [0, 2]
//!			(2) => [0, 4]
//!			(3) => [0, 1, 3]
//!			(4) => [1, 2, 4, 5]
//!			(5) => []
//!		];
//!
//!		let shortest_path = g[&0].bfs(|_, v, _| {
//!			if *v == g[&5] {
//! 			(Coll::Include, Sig::Terminate)
//! 		} else {
//!				(Coll::Include, Sig::Continue)
//! 		}
//!		}).unwrap();
//!
//!		assert!(shortest_path.node_count() == 4);
//!	}
//! ```

pub mod node_trait;
pub mod edge_trait;
pub mod path;
pub mod enums;
pub mod dinode;
pub mod async_ptr;
pub mod templates;
pub mod macros;

#[cfg(test)]
mod tests {
	use crate::*;
	// use crate::edge_trait::GraphEdge;
	// use crate::path::Path;
	use crate::enums::{Coll, Sig};
	#[test]
	fn shortest_path() {
		use crate::*;
		use crate::enums::{Coll, Sig};
		// use crate::enums::*;
		use crate::node_trait::*;
		// use crate::dinode::Node;

		let g = graph![
			(usize)
			(0) => [1, 2]
			(1) => [0, 2]
			(2) => [0, 4]
			(3) => [0, 1, 3]
			(4) => [1, 2, 4, 5]
			(5) => []
		];

		let shortest_path = g[&0].bfs(|_, cur, _| {
			if *cur == g[&5] { (Coll::Include, Sig::Terminate) }
			else { (Coll::Include, Sig::Continue) }
		}).unwrap();

		assert!(shortest_path.node_count() == 4);

		// let source = &g[&0];
		// let target = &g[&5];

		// let mut edge_tree = Vec::new();
		// let first_edge = source.outbound().read();
		// let first_edge = first_edge.first().unwrap();
		// edge_tree.push(first_edge.clone());
		// let mut bounds: (usize, usize) = (0, 0);
		// loop {
		// 	let cur = edge_tree[bounds.1].target();
		// 	bounds.1 = edge_tree.len();
		// 	if bounds.0 == bounds.1 {
		// 		break;
		// 	}
		// 	let adjacent = source.outbound().read();
		// 	for edge in adjacent.iter() {
		// 		match edge.target().try_close() {
		// 			Ok(_) => {
		// 				edge_tree.push(edge.clone());
		// 				if edge.target() == target {
		// 					break;
		// 				}
		// 			}
		// 			Err(_) => {
		// 				continue;
		// 			}
		// 		}
		// 	}
		// 	bounds.0 = bounds.1;
		// 	let next = edge_tree[bounds.0].target();
		// }

		// let shortest_path: Path<Node<usize, Empty, Empty>> = Path::from_edge_tree(edge_tree);

		// assert!(shortest_path.node_count() == 4);
	}

	#[test]
	fn any_path() {
		use crate::node_trait::*;
		let g = graph![
			(usize)
			(0) => [1, 2]
			(1) => [0, 2]
			(2) => [0, 4]
			(3) => [0, 1, 3]
			(4) => [1, 2, 4, 5]
			(5) => []
		];

		let source = &g[&0];
		let target = &g[&5];

		let any_path = source.dfs(|_, t, _| {
			if t == target { (Coll::Include, Sig::Terminate) }
			else { (Coll::Include, Sig::Continue) }
		}).unwrap();

		assert!(any_path.node_count() == 5);
	}

	#[test]
	fn djikstra() {
		use crate::node_trait::*;
		use crate::dinode::Node;
		use std::cmp::Reverse;
		use std::collections::BinaryHeap;

		// CREATE GRAPH:

		// In order to run Dijkstra's algorithm, we create a graph where edges
		// represent the cost of moving from one node to another and the nodes
		// save the shortest distance to the source node.
		let g = graph![

			// The graph's type signature is (NodeKey, NodeParam) => [EdgeParam]
			// in the macro invocation.
			(&str, u64) => [u64]

			// We can add nodes and edges simultaneously. Trying to add an edge
			// between nodes that don't exist will result in a panic.
			("A", u64::MAX)	=> [ ("B", 4), ("H", 8) ]
			("B", u64::MAX) => [ ("A", 4), ("H", 11), ("C", 8) ]
			("C", u64::MAX) => [ ("B", 8), ("C", 2), ("F", 4), ("D", 7) ]
			("D", u64::MAX) => [ ("C", 7), ("F", 14), ("E", 9) ]
			("E", u64::MAX) => [ ("D", 9), ("F", 10) ]
			("F", u64::MAX) => [ ("G", 2), ("C", 4), ("D", 14), ("E", 10) ]
			("G", u64::MAX) => [ ("H", 1), ("I", 6), ("F", 2) ]
			("H", u64::MAX) => [ ("A", 8), ("B", 11), ("I", 7), ("G", 1) ]
			("I", u64::MAX) => [ ("H", 7), ("C", 2), ("G", 6) ]

		];

		// Since the standard library's BinaryHeap is a max-heap, we need to
		// reverse the order of the nodes to make it a min-heap so we wrap
		// our node (which implements Ord) into the std::cmp::Reverse enum.
		let mut min_heap = BinaryHeap::<Reverse<Node<&str, u64, u64>>>::new();

		// DIJKSTRA'S ALGORITHM:

		// START:
		// We start by adding the source node to the min-heap with a distance
		// of 0.
		g["A"].store(0);
		min_heap.push(Reverse(g["A"].clone()));

		// OUTER LOOP:
		// We then from the the min-heap until it is empty or the target is
		// found.
		while let Some(u) = min_heap.pop() {

			// Because `u` is wrapped in the Reverse enum so we access it from
			// u.0`. We load the parameters (distance in this case) from the
			// node using the `load` method.
			let (u_dist, u) = (u.0.load(), u.0);

			// TERMINATE:
			// Our loop over nodes in the heap terminates if the target node
			// is reached.
			if u == g["E"] {

				// We expect the minimum distance of A -> E to be 21.
				assert!(u_dist == 21);
				break;
			}

			// INNER LOOP:
			// We then iterate over the edges `e` of the current node `u`.
			// If the distance in `u` plus the cost of `e` is less than
			// the distance already in `v`, we update the distance in `v`
			// and add it to the min-heap.
			while let Some((e, v)) = u.next() {
				let (v_dist, e_len) = (v.load(), e);
				if v_dist > u_dist + e_len {
					v.store(u_dist + e_len);
					min_heap.push(Reverse(v.clone()));
				}
			}
		}
	}

	#[test]
	fn basic1() {
		use crate::node_trait::*;

		#[derive(Debug, PartialEq, Eq, Clone, std::hash::Hash)]
		enum Space { Galaxy, Star, BlackHole, Nebula}

		impl std::fmt::Display for Space {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				match self {
					Space::Galaxy => write!(f, "Galaxy"),
					Space::Star => write!(f, "Star"),
					Space::BlackHole => write!(f, "Black Hole"),
					Space::Nebula => write!(f, "Nebula"),
				}
			}
		}

		let space = graph![

			(&str, Space) => [f32]

			("Proxima Centauri", Space::Star) =>
				[
					("Milky Way", 2873.124),
					("Andromeda", 3425.24)
				]

			("Milky Way", Space::Galaxy) =>
				[
					("Proxima Centauri", 547.135),
					("Andromeda", 78873.145)
				]

			("Andromeda", Space::Galaxy) =>
				[
					("Proxima Centauri", 23.442),
					("TON-512", 25663.156)
				]

			("Horse Nebula", Space::Nebula) =>
				[
					("Proxima Centauri", 46137.124),
					("Milky Way", 146.312),
					("Andromeda", 6143.12)
				]

			("TON-512", Space::BlackHole) =>
				[
					("Milky Way", 134.342),
					("Andromeda", 24834.1432),
					("Sagittarius A*", 74313.731)
				]

			("Sagittarius A*", Space::BlackHole) => []

		];

		let source = &space["Proxima Centauri"];
		let target = &space["Sagittarius A*"];

		let result = source.bfs(|_, t, _| {
			match target == t {
				false => (Coll::Include, Sig::Continue),
				true => (Coll::Include, Sig::Terminate),
			}
		}).unwrap();

		assert!(result.node_count() == 4);
	}

	#[test]
	fn flow_graph() {
		use crate::enums::{Coll::*, Sig::*};
		use crate::templates::*;
		use crate::node_trait::*;

		// Prepare flow graph
		let graph: Vec<FlowNode> = vec![
			node![0], node![1], node![2],
			node![3], node![4], node![5],
		];

		Flow::connect(&graph[0], &graph[1], 16);
		Flow::connect(&graph[0], &graph[2], 13);
		Flow::connect(&graph[1], &graph[2], 10);
		Flow::connect(&graph[1], &graph[3], 12);
		Flow::connect(&graph[2], &graph[1], 4);
		Flow::connect(&graph[2], &graph[4], 14);
		Flow::connect(&graph[3], &graph[2], 9);
		Flow::connect(&graph[3], &graph[5], 20);
		Flow::connect(&graph[4], &graph[3], 7);
		Flow::connect(&graph[4], &graph[5], 4);

		// Maximum flow algorithm
		let mut max_flow: u64 = 0;

		// 1. We loop breadth-first until there is no more paths to explore
		while let Some(path) = graph[0].bfs(|_, t, flow| {
			let flow = flow.read();

			// 2. We exclude saturated edges from the search and terminate
			// if we reach the target
			match flow.cur < flow.max {
				true => {
					match t == &graph[5] {
						true => (Include, Terminate),
						false => (Include, Continue)
					}
				}
				false => (Exclude, Continue)
			}
		}){
			let mut augmenting_flow = std::u64::MAX;

			// 3. We find the minimum augmenting flow along the path
			path.walk(|_, _, edge_param| {
				let flow = edge_param.read();
				if flow.max - flow.cur < augmenting_flow {
					augmenting_flow = flow.max - flow.cur;
				}
			});

			// 4. We augment the flow along the path
			path.walk( |_, _, edge_param| {
				edge_param.update(|flow| flow.cur += augmenting_flow);
				edge_param.read().rev.update(|flow| flow.cur -= augmenting_flow);
			});

			// 5. We update the maximum flow
			max_flow += augmenting_flow;
		}

		// For this graph we expect the maximum flow from 0 -> 5 to be 23
		assert!(max_flow == 23);
	}
}

