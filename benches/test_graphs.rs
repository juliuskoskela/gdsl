use gdsl::digraph::*;
use std::cell::Cell;
use std::cmp::{max, min};

use gdsl::{digraph_connect as connect, digraph_node as node};

pub fn create_graph_vec_distance_1(size: usize) -> Vec<Node<usize, Cell<usize>, usize>> {
	let mut g = Vec::new();

	for i in 0..size {
		g.push(node!(i, Cell::new(usize::MAX)));
	}

	for (i, node) in g.iter().enumerate() {
		let neighbour_count = i % 8 + 3;
		let j_from = max(0, i - neighbour_count / 2);
		let j_to = min(size, j_from + neighbour_count);
		for j in j_from..j_to {
			connect!(&node => &g[j], (i + 3) % 10);
		}
	}
	g
}

pub fn create_graph_simple_1(size: usize, avg_dgr: usize) -> Graph<usize, (), ()> {
	let mut g = Graph::new();

	for i in 0..size {
		g.insert(node!(i, ()));
	}

	for (_, node) in g.iter() {
		for _ in 0..avg_dgr {
			connect!(&node => &g[rand::random::<usize>() % size], ());
		}
	}
	g
}
