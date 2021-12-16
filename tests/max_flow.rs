use fastgraph::core::*;
use fastgraph::collections::*;
use fastgraph::core::Empty;
use std::sync::{Arc, Weak};

// Flow Graph

// A struct which records the maximum flow and current flow for an edge
// and stores a weak pointer to the reverse edge. We store the reverse
// edge so we don't have to search for it in the outbound edges of the
// edge's target node.
#[derive(Clone, Debug)]
pub struct Flow {
	pub max: usize,
	pub cur: usize,
	pub rev: Weak<Edge<usize, Empty, Flow>>,
}

impl std::fmt::Display for Flow {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "{}/{}", self.cur, self.max)
	}
}

// A type for the flow graph.
pub type FlowGraph = Digraph<usize, Empty, Flow>;

// Edge insertion in the flow-graph is a bit more involved. We need to save
// a reverse edge for each forward edge with a maxed out capacity. When we
// add flow to the edge, we decrease it from it's reverse edge.
pub fn add_edge_flow(g: &mut FlowGraph, u: usize, v: usize, flow: usize) {
	g.add_edge(u, v, Flow { max: flow, cur: 0, rev: Weak::new() });
	g.add_edge(v, u, Flow { max: flow, cur: flow, rev: Weak::new() });
	let uv = g.get_edge(u, v).unwrap();
	let vu = g.get_edge(v, u).unwrap();
	let mut uv_data = uv.load();
	let mut vu_data = vu.load();
	uv_data.rev = Arc::downgrade(&vu);
	vu_data.rev = Arc::downgrade(&uv);
	uv.store(uv_data);
	vu.store(vu_data);
}

// Maximum flow of a directed graph using the Edmond's-Karp algorithm.
pub fn parallel_maximum_flow_edmonds_karp(g: &FlowGraph, s: usize, t: usize) -> usize {

	// We loop over the graph doing a breadth first search. Inside the closure
	// we check the flow of each edge. If the edge is not saturated, we traverse
	// it. The Traverse enum will collect the edge in the results. Otherwise we
	// skip the edge.
	let target = g.get_node(t).unwrap();
	let explorer = |e: &Arc<Edge<usize, Empty, Flow>> | {
		let flow = e.load();
		if flow.cur < flow.max {
			if target == e.target() {
				Traverse::Finish
			}
			else {
				Traverse::Include
			}
		}
		else {
			Traverse::Skip
		}
	};
	let mut max_flow: usize = 0;
	while let Some(b) = g.par_breadth_first(s, explorer)
	{
		// We backtrack the results from the breadth first traversal which will
		// produce the shortest path.
		let path = backtrack_edges(&b);

		// We find the bottleneck value of the path.
		let mut aug_flow = std::usize::MAX;
		for weak in path.iter() {
			let e = weak.upgrade();
			match e {
				Some(edge) => {
					let flow = edge.load();
						if flow.max - flow.cur < aug_flow {
							aug_flow = flow.max - flow.cur;
					}
				}
				None => { panic!("Weak pointer invalid!") }
			}
		}

		// We update the flow along the path.
		for weak in path.iter() {

			// Increase flow in the forward edge.
			let e = weak.upgrade().unwrap();
			let mut flow = e.load();
			flow.cur += aug_flow;

			// Decrease flow in the reverse edge.
			let r = flow.rev.upgrade().unwrap();
			let mut rev_flow = r.load();
			rev_flow.cur -= aug_flow;

			// Store results.
			e.store(flow);
			r.store(rev_flow);
		}

		// Max flow is the sum of all augmenting flows.
		max_flow += aug_flow;
	}
	max_flow
}

pub fn maximum_flow_edmonds_karp(g: &FlowGraph, s: usize, t: usize) -> usize {
	let target = g.get_node(t).unwrap();
	let explorer = |e: &Arc<Edge<usize, Empty, Flow>>| {
		let flow = e.load();
		if flow.cur < flow.max {
			if target == e.target() {
				Traverse::Finish
			}
			else {
				Traverse::Include
			}
		}
		else {
			Traverse::Skip
		}
	};
	let mut max_flow: usize = 0;
	while let Some(b) = g.breadth_first(s, explorer)
	{
		let path = backtrack_edges(&b);
		let mut aug_flow = std::usize::MAX;
		for weak in path.iter() {
			let e = weak.upgrade();
			match e {
				Some(edge) => {
					let flow = edge.load();
						if flow.max - flow.cur < aug_flow {
							aug_flow = flow.max - flow.cur;
					}
				}
				None => { panic!("Weak pointer invalid!") }
			}
		}
		for weak in path.iter() {
			let e = weak.upgrade().unwrap();
			let mut flow = e.load();
			flow.cur += aug_flow;
			let r = flow.rev.upgrade().unwrap();
			let mut rev_flow = r.load();
			rev_flow.cur -= aug_flow;
			e.store(flow);
			r.store(rev_flow);
		}
		max_flow += aug_flow;
	}
	max_flow
}


fn flow_graph_example_1to6_23() -> FlowGraph {
	let mut g = FlowGraph::new();
	g.add_node(1, Empty);
	g.add_node(2, Empty);
	g.add_node(3, Empty);
	g.add_node(4, Empty);
	g.add_node(5, Empty);
	g.add_node(6, Empty);
	add_edge_flow(&mut g, 1, 2, 16);
	add_edge_flow(&mut g, 1, 3, 13);
	add_edge_flow(&mut g, 2, 3, 10);
	add_edge_flow(&mut g, 2, 4, 12);
	add_edge_flow(&mut g, 3, 2, 4);
	add_edge_flow(&mut g, 3, 5, 14);
	add_edge_flow(&mut g, 4, 3, 9);
	add_edge_flow(&mut g, 4, 6, 20);
	add_edge_flow(&mut g, 5, 4, 7);
	add_edge_flow(&mut g, 5, 6, 4);
	g
}

#[test]
fn digraph_test_maximum_flow_edmonds_karp() {
	let g = flow_graph_example_1to6_23();
	let max_flow = parallel_maximum_flow_edmonds_karp(&g, 1, 6);
	assert!(max_flow == 23);
}
