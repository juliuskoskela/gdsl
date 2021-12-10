use crate::{digraph::*, core::*};
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
	pub rev: WeakEdge<usize, Empty, Flow>,
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
pub fn parallel_maximum_flow_edmonds_karp(g: &FlowGraph, s: usize, t: usize) -> usize {

	// We loop over the graph doing a breadth first search. Inside the closure
	// we check the flow of each edge. If the edge is not saturated, we traverse
	// it. The Traverse enum will collect the edge in the results. Otherwise we
	// skip the edge.
	let target = g.node(&t).unwrap();
	let explorer = |e: &ArcEdge<usize, Empty, Flow>| {
		let flow = e.load();
		if flow.cur < flow.max {
			if *target == e.target() {
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
	while let Some(b) = g.par_breadth_first(&s, explorer)
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
	let target = g.node(&t).unwrap();
	let explorer = |e: &ArcEdge<usize, Empty, Flow>| {
		let flow = e.load();
		if flow.cur < flow.max {
			if *target == e.target() {
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
	while let Some(b) = g.breadth_first(&s, explorer)
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

pub fn digraph_colouring() {
	type ChromaticGraph = Digraph<usize, usize>;

	let mut g = ChromaticGraph::new();

	g.insert(1, 0);
	g.insert(2, 0);
	g.insert(3, 0);
	g.insert(4, 0);
	g.connect(&1, &2,  Empty);
	g.connect(&1, &3,  Empty);
	// g.connect(&1, &4,  Empty);

	g.connect(&2, &1,  Empty);
	g.connect(&2, &3,  Empty);
	// g.connect(&2, &4,  Empty);

	g.connect(&3, &1,  Empty);
	g.connect(&3, &2,  Empty);
	// g.connect(&3, &4,  Empty);

	g.connect(&4, &1,  Empty);
	g.connect(&4, &2,  Empty);
	// g.connect(&4, &3,  Empty);

	for (_, node) in g.nodes.iter() {
		let mut unavailable = vec![];
		for edge in node.outbound().iter() {
			let colour = edge.target().load();
			unavailable.push(colour);
		}
		let mut node_colour: usize = 0;
		loop {
			let check = unavailable.iter().find(|predicate| { node_colour == **predicate });
			match check {
				Some(_) => { },
				None => {
					node.store(node_colour);
					break ;
				}
			}
			node_colour += 1;
		}
	}
	g.print_nodes();
}
