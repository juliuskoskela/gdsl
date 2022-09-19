#![allow(non_snake_case)]
/// # PageRank algorithm
///
/// The algorithm calculates the page rank for each node and stores it in the
/// node itself.
///
/// Methodology derived from https://github.com/alixaxel/pagerank
use gdsl::digraph::*;
use gdsl::*;
use std::cell::Cell;
use std::rc::Rc;
use gdsl::digraph as graph;
use std::fs::File;
use std::io::Write;

mod barabasi_albert_graph;
use barabasi_albert_graph::barabasi_albert_random_graph;

/// Types definitions for the graph and nodes.
type N = Node<usize, Cell<f64>, Rc<Cell<f64>>>;
type G = Vec<N>;

/// PageRank algorithm takes in a graph `G`, a damping factor `α` and a
/// convergence threshold `ε`. Edges are expected to have non-negative weights.
/// The algorithm calculates the page rank for each node and stores it in the
/// node itself.
///
/// The PageRank theory holds that an imaginary surfer who is randomly clicking
/// on edges will eventually stop clicking. The probability, at any step, that
/// the person will continue is a damping factor `α`. Various studies have tested
/// different damping factors, but it is generally assumed that the damping
/// factor will be set around 0.85.
pub fn page_rank(g: &G, α: f64, ε: f64) {
	let inverse = 1.0 / g.len() as f64;

	// Normalize all edge weights to sum to 1
	for node in g {
		let sum = node.iter_out()
			.map(|e| e.value().get())
			.sum::<f64>();
		for Edge(_, _, e) in node {
			e.set(e.get() / sum);
		}
		node.set(inverse);
	}

	let mut Δ = 1.0;

	// Iterate until convergence
	while Δ > ε {
		let mut nodes = vec![0.0; g.len()];
		let mut leak = 0.0;

		for u in g.iter() {
			nodes[*u.key()] = u.get();
			if u.out_degree() == 0 {
				leak += u.get();
			}
			u.set(0.0);
		}

		leak *= α;

		// Move values from nodes to temporary array
		// and calculate the leak. The leak is the
		// amount of weight that is not distributed
		// to any other node.
		for u in g {
			for Edge(_, v, e) in u {
				v.set(v.get() + α * nodes[*u.key()] * e.get());
			}
			u.set(u.get() + (1.0 - α) * inverse + leak * inverse);

		}

		// Calculate the change `Δ` in node weights.
		Δ = g.iter()
			.map(|u| (u.get() - nodes[*u.key()]).abs())
			.sum();
	}
}

#[test]
fn test() {
    let mut g = graph![
		(usize, Cell<f64>) => [Rc<Cell<f64>>]
		(0, Cell::new(0.0)) => [(1, Rc::new(Cell::new(1.0))), (2, Rc::new(Cell::new(2.0)))]
		(1, Cell::new(0.0)) => [(2, Rc::new(Cell::new(3.0))), (3, Rc::new(Cell::new(4.0)))]
		(2, Cell::new(0.0)) => [(0, Rc::new(Cell::new(5.0)))]
		(3, Cell::new(0.0)) => []
	].to_vec();

	g.sort_by(|l, r| l.key().cmp(r.key()));

	page_rank(&g, 0.85, 0.0001);

	assert!(g[0].get() == 0.3498289767085323);
	assert!(g[1].get() == 0.16887946089722788);
	assert!(g[2].get() == 0.3295188106005262);
	assert!(g[3].get() == 0.1517727517937135);
}

fn attr(field: &str, value: &str) -> (String, String) {
	(field.to_string(), value.to_string())
}

pub const THEME: [&str; 5] = [
	"#ffffff", // 0. background
	"#ffe5a9", // 1. medium
	"#423f3b", // 2. dark
	"#ff6666", // 3. accent
	"#525266", // 4. theme
];

fn main() {
	let g = barabasi_albert_random_graph(50, 5);
	let mut v = g.to_vec();
	page_rank(&v, 0.85, 0.0001);
	v.sort_by(|l, r| l.key().cmp(r.key()));
	for node in &v {
		println!("{}: {}", node.key(), node.get());
	}
	let sum = v.iter().map(|n| n.get()).sum::<f64>();
	println!("Sum: {}", sum);

	// Write a dot representation with page rank * 100 as node size.
	let dot_str = g.to_dot_with_attr(
		&|graph| {
			let graph_size_mb = graph.sizeof() as f64 / 1024.0 / 1024.0;
			Some(vec![
				attr("bgcolor", THEME[0]),
				attr("fontcolor", THEME[4]),
				attr("label", &format!("Barabasi Albert Graph {:.4} Mb", graph_size_mb)),
			])
		},
		&|node| {
			let color = THEME[3];
			let size = node.get() * 10.0;
			Some(vec![
				attr("color", color),
				attr("fontcolor", THEME[4]),
				attr("label", &format!("{}", node.key())),
				attr("width", &format!("{}", size)),
				attr("height", &format!("{}", size)),
			])
		},
		&|_, _, edge| {
			let color = THEME[4];
			Some(vec![
				attr("fontcolor", THEME[4]),
				attr("weight", &edge.get().to_string()),
				attr("color", color),
			])
		},
	);

	// Write the dot file to disk.
	let mut file = File::create("graph.dot").unwrap();
	file.write_all(dot_str.as_bytes()).unwrap();
}
