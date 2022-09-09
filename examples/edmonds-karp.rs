// # Edmonds–Karp algorithm for maximum flow.
//
// https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
//
// An algorithm for finding a maximum flow in a flow network.

use gdsl::{
	digraph_node as node,
	digraph_connect as connect,
	digraph::*,
};

use std::rc::{Weak, Rc};
use std::cell::Cell;

type N = Node<usize, (), FlowEdge>;
type G = Graph<usize, (), FlowEdge>;

#[derive(Clone, Copy)]
struct Flow(u64, u64);

#[derive(Clone)]
struct FlowEdge(Rc<Cell<Flow>>, Weak<Cell<Flow>>);

impl FlowEdge {

	// Connect two nodes with a flow.
	fn connect(s: &N, t: &N, max: u64) {

		// Create a forward and a reverse flow.
		let mut fflow = FlowEdge(Rc::new(Cell::new(Flow(max, 0))), Weak::new());
		let mut rflow = FlowEdge(Rc::new(Cell::new(Flow(max, max))), Weak::new());

		// Cross-connect the flows.
		fflow.1 = Rc::downgrade(&rflow.0);
		rflow.1 = Rc::downgrade(&fflow.0);

		// Connect the flows to the nodes.
		connect!(s => t, fflow);
		connect!(t => s, rflow);
	}

	// Update flow with the augmenting flow.
	fn update(&self, aug_flow: u64) {

		// Decompose the flow parameters.
		let (fflow, rflow) = (&self.0, &self.1.upgrade().unwrap());
		let Flow (fmax, fcur) = fflow.get();
		let Flow (rmax, rcur)  = rflow.get();

		// Increase the flow in the forward direction and decrease
		// the flow in the reverse direction.
		fflow.set(Flow (fmax, fcur + aug_flow));
		rflow.set(Flow (rmax, rcur - aug_flow));
	}

	// Get the max flow.
	fn max(&self) -> u64 { self.0.get().0 }

	// Get the current flow.
	fn cur(&self) -> u64 { self.0.get().1 }
}

fn max_flow(g: &G) -> u64 {

	// 1. We loop breadth-first until there is no more paths to explore.
	let mut max_flow: u64 = 0;

	while let Some(path) = g[0]
		.dfs()
		.target(&5)
		// 2. We exclude saturated edges from the search.
		.filter(&|Edge(_, _, e)| e.cur() < e.max())
		.search_path()
	{
		let mut aug_flow = std::u64::MAX;

		// 3. We find the minimum augmenting flow along the path.
		for Edge(_, _, flow) in path.iter_edges() {
			if flow.max() - flow.cur() < aug_flow {
				aug_flow = flow.max() - flow.cur();
			}
		}

		// 4. We update the flow along the path.
		for Edge(_, _, flow) in path.iter_edges() {
			flow.update(aug_flow);
		}

		// 5. We update the maximum flow.
		max_flow += aug_flow;
	}
	max_flow
}

fn main() {

	// Generate an example Graph with a max flow of 23 from 0 to 5.
	let mut g = Graph::new();

	g.insert(node!(0));
	g.insert(node!(1));
	g.insert(node!(2));
	g.insert(node!(3));
	g.insert(node!(4));
	g.insert(node!(5));

	FlowEdge::connect(&g[0], &g[1], 16);
	FlowEdge::connect(&g[0], &g[2], 13);
	FlowEdge::connect(&g[1], &g[2], 10);
	FlowEdge::connect(&g[1], &g[3], 12);
	FlowEdge::connect(&g[2], &g[1], 4);
	FlowEdge::connect(&g[2], &g[4], 14);
	FlowEdge::connect(&g[3], &g[2], 9);
	FlowEdge::connect(&g[3], &g[5], 20);
	FlowEdge::connect(&g[4], &g[3], 7);
	FlowEdge::connect(&g[4], &g[5], 4);

	// For this Graph we expect the maximum flow from 0 -> 5 to be 23
	assert!(max_flow(&g) == 23);

	// print_flow_graph(&g);
}

// fn attr(field: &str, value: &str) -> (String, String) {
// 	(field.to_string(), value.to_string())
// }

// pub const THEME: [&str; 5] = [
// 	"#ffffff", // 0. background
// 	"#ffe5a9", // 1. medium
// 	"#423f3b", // 2. dark
// 	"#ff6666", // 3. accent
// 	"#525266", // 4. theme
// ];

// fn print_flow_graph(g: &G) {
// 	let dot_str = g.to_dot_with_attr(
// 		&|| {
// 			Some(vec![
// 				attr("bgcolor", THEME[0]),
// 				attr("fontcolor", THEME[4]),
// 				attr("label", "Flow Graph"),
// 			])
// 		 },
// 		&|node| {
// 			Some(vec![
// 				attr("fillcolor", THEME[1]),
// 				attr("fontcolor", THEME[4]),
// 				attr("label", &format!("{}", node.key())),
// 			])
// 		},
// 		&|_, _, edge| {
// 			let Flow (max, cur) = edge.0.get();
// 			let flow_str = format!("{}/{}", cur, max);
// 			let color = if cur == 0 { THEME[4] } else { THEME[3] };
// 			Some(vec![
// 				attr("fontcolor", THEME[4]),
// 				attr("label", &flow_str),
// 				attr("color", &color),
// 			])
// 		 }
// 	);

// 	println!("{}", dot_str);
// }