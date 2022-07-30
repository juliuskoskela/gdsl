// # Edmonds–Karp algorithm for maximum flow.
//
// https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm

use gdsl::digraph::DiGraph as Graph;
use gdsl::digraph::DiNode as Node;
use gdsl::*;
use std::rc::{Weak, Rc};
use std::cell::Cell;

#[derive(Clone, Copy)]
struct Flow(u64, u64);

#[derive(Clone)]
struct FlowEdge(Rc<Cell<Flow>>, Weak<Cell<Flow>>);

impl FlowEdge {

	// Connect two nodes with a flow.
	fn connect(s: &Node<usize, Empty, FlowEdge>, t: &Node<usize, Empty, FlowEdge>, max: u64) {

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

	// Update the flow with the agmenting flow.
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

fn max_flow(g: Graph<usize, Empty, FlowEdge>) -> u64 {

	// 1. We loop breadth-first until there is no more paths to explore.
	let mut max_flow: u64 = 0;
	while let Some(path) = g[0].bfs()
		.target(&5)
		// 2. We exclude saturated edges from the search.
		.filter(&|_, _, edge| edge.cur() < edge.max())
		.path_edges()
	{
		let mut aug_flow = std::u64::MAX;

		// 3. We find the minimum augmenting flow along the path.
		for (_, _, e) in &path {
			if e.max() - e.cur() < aug_flow {
				aug_flow = e.max() - e.cur();
			}
		}

		// 4. We update the flow along the path.
		for (_, _, e) in &path {
			e.update(aug_flow);
		}

		// 5. We update the maximum flow.
		max_flow += aug_flow;
	}
	max_flow
}

fn main() {
	// Generate an example Graph with a max flow of 23 from 0 to 5.
	let mut g = Graph::new();

	g.insert(dinode!(0));
	g.insert(dinode!(1));
	g.insert(dinode!(2));
	g.insert(dinode!(3));
	g.insert(dinode!(4));
	g.insert(dinode!(5));

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
	assert!(max_flow(g) == 23);
}
