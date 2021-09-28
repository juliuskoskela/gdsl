use crate::{digraph::*, global::*};
use std::sync::{Arc, Weak};
use crate::global::Traverse::{Traverse, Skip, Finish};

// Flow Graph

// A struct which records the maximum flow and current flow for an edge
// and stores a weak pointer to the reverse edge. We store the reverse
// edge so we don't have to search for it in the outbound edges of the
// edge's target node.
#[derive(Clone, Debug)]
pub struct Flow {
	pub max: usize,
	pub cur: usize,
	pub rev: WeakEdge<usize, Void, Flow>,
}

impl std::fmt::Display for Flow {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "{}/{}", self.cur, self.max)
	}
}

// A type for the flow graph.
pub type FlowGraph = Digraph<usize, Void, Flow>;

// Edge insertion in the flow-graph is a bit more involved. We need to save
// a reverse edge for each forward edge with a maxed out capacity. When we
// add flow to the edge, we decrease it from it's reverse edge.
pub fn connect_flow(g: &mut FlowGraph, u: &usize, v: &usize, flow: usize) {
	g.connect(u, v, Flow { max: flow, cur: 0, rev: Weak::new() });
	g.connect(v, u, Flow { max: flow, cur: flow, rev: Weak::new() });
	let uv = g.edge(u, v).unwrap();
	let vu = g.edge(v, u).unwrap();
	let mut uv_data = uv.load();
	let mut vu_data = vu.load();
	uv_data.rev = Arc::downgrade(&vu);
	vu_data.rev = Arc::downgrade(&uv);
	uv.store(uv_data);
	vu.store(vu_data);
}

// Maximum flow of a directed graph using the Edmond's-Karp algorithm.
pub fn maximum_flow_edmonds_karp(g: &FlowGraph, s: usize, t: usize) -> usize {

	// We loop over the graph doing a breadth first search. Inside the closure
	// we check the flow of each edge. If the edge is not saturated, we traverse
	// it. The Traverse enum will collect the edge in the results. Otherwise we
	// skip the edge.
	let target = g.node(&t).unwrap();
	let mut max_flow: usize = 0;
	while let Some(b) = g.breadth_first(&s,
		|e| {
			let flow = e.load();
			if flow.cur < flow.max {
				if *target == e.target() { Finish }
				else { Traverse }
			}
			else { Skip }
	})
	{
		// We backtrack the results from the breadth first traversal which will
		// produce the shortest path.
		let path = b.backtrack().unwrap();

		// We find the bottleneck value of the path.
		let mut aug_flow = std::usize::MAX;
		for weak in path.iter() {
			let e = weak.upgrade().unwrap();
			let flow = e.load();
			if flow.max - flow.cur < aug_flow {
				aug_flow = flow.max - flow.cur;
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

// Maximum flow of a directed graph using the Ford-Fulkerson method.
pub fn maximum_flow_ford_fulkerson(g: &FlowGraph, s: usize, t: usize) -> usize {

	// We loop over the graph doing a breadth first search. Inside the closure
	// we check the flow of each edge. If the edge is not saturated, we traverse
	// it. The Traverse enum will collect the edge in the results. Otherwise we
	// skip the edge.
	let target = g.node(&t).unwrap();
	let mut max_flow: usize = 0;
	while let Some(b) = g.depth_first(&s,
		|e| {
			let flow = e.load();
			if flow.cur < flow.max {
				if *target == e.target() {
					Finish
				}
				else {
					Traverse
				}
			}
			else {
				Skip
			}
	})
	{
		// We backtrack the results from the breadth first traversal which will
		// produce the shortest path.
		let path = b.backtrack().unwrap();

		// We find the bottleneck value of the path.
		let mut aug_flow = std::usize::MAX;
		for weak in path.iter() {
			let e = weak.upgrade().unwrap();
			let flow = e.load();
			if flow.max - flow.cur < aug_flow {
				aug_flow = flow.max - flow.cur;
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
