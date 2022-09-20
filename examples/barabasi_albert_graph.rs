/// # Barabasi Albert graph
///
/// Create a random graph with the Barabasi-Albert preferential
/// attachment model.
///

use gdsl::digraph::*;
use gdsl::{digraph_node as node, digraph_connect as connect};
use std::cell::Cell;
use std::rc::Rc;
use std::fs::File;
use std::io::Write;

/// Types definitions for the graph and nodes.
type N = Node<usize, Cell<f64>, Rc<Cell<f64>>>;
type G = Graph<usize, Cell<f64>, Rc<Cell<f64>>>;

// Select n random nodes from the vector v
fn get_random_subset(v: &Vec<N>, n: usize) -> Vec<N> {
	let mut subset = vec![];
	for _ in 0..n {
		let node = &v[rand::random::<usize>() % v.len()];
		subset.push(node.clone());
	}
	subset
}

pub fn barabasi_albert_random_graph(size: usize, degree: usize) -> G {
	let mut g = Graph::new();

	// Initialize the graph with `degree` empty nodes
	for i in 0..degree {
		g.insert(node!(i, Cell::new(0.0)));
	}

	// Target nodes for new edges
	let mut targets = g.to_vec();

	// List of existing nodes where each node is repeated
	// by the number of its incoming edges
	let mut repeated_nodes = Vec::new();

	// Add `size - degree` nodes to the graph
	for i in degree..size {
		let new_node = node!(i, Cell::new(0.0));
		for node in targets {
			connect!(&new_node => &node, Rc::new(Cell::new(1.0)));
			repeated_nodes.push(node);
			repeated_nodes.push(new_node.clone());
		}
		g.insert(new_node);
		targets = get_random_subset(&repeated_nodes, degree);
	}
	g
}

fn main() {
	let g = barabasi_albert_random_graph(50, 5);
	let dot = g.to_dot();

	// write the dot file to disk
	let mut file = File::create("barabasi-albert-graph.dot").unwrap();
	file.write_all(dot.as_bytes()).unwrap();
}
