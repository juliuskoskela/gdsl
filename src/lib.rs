pub mod digraph;
pub mod node;
pub mod edge;
pub mod adjacent;
pub mod global;
pub mod edge_list;

#[cfg(test)]
mod tests {
	use crate::{digraph::*, global::*};
	use std::sync::{Arc, Weak};
	use crate::global::Traverse::{Traverse, Skip};
	// Digraph Test: Maximum Flow Edmond's Karp

	// A struct which records the maximum flow and current flow for an edge
	// and stores a weak pointer to the reverse edge. We store the reverse
	// edge so we don't have to search for it in the outbound edges of the
	// edge's target node.
	#[derive(Clone, Debug)]
	struct Flow {
		pub max: i64,
		pub cur: i64,
		pub rev: EdgeWeak<usize, Void, Flow>,
	}

	impl std::fmt::Display for Flow {
		fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(fmt, "{}/{}", self.cur, self.max)
		}
	}

	// A type for the flow graph.
	type FlowGraph = Digraph<usize, Void, Flow>;

	// Edge insertion in the flow-graph is a bit more involved. We need to save
	// a reverse edge for each forward edge with a maxed out capacity. When we
	// add flow to the edge, we decrease it from it's reverse edge.
	fn connect_flow(g: &mut FlowGraph, u: &usize, v: &usize, flow: i64) {
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

	// Example graph for the test with a max flow of 23.
	fn flow_graph_example() -> FlowGraph {
		let mut g = FlowGraph::new();
		g.insert(1, Void);
		g.insert(2, Void);
		g.insert(3, Void);
		g.insert(4, Void);
		g.insert(5, Void);
		g.insert(6, Void);
		connect_flow(&mut g, &1, &2, 16);
		connect_flow(&mut g, &1, &3, 13);
		connect_flow(&mut g, &2, &3, 10);
		connect_flow(&mut g, &2, &4, 12);
		connect_flow(&mut g, &3, &2, 4);
		connect_flow(&mut g, &3, &5, 14);
		connect_flow(&mut g, &4, &3, 9);
		connect_flow(&mut g, &4, &6, 20);
		connect_flow(&mut g, &5, &4, 7);
		connect_flow(&mut g, &5, &6, 4);
		g
	}

	fn maximum_flow_edmonds_karp(g: &FlowGraph) -> i64 {

		// We loop over the graph doing a breadth first search. Inside the closure
		// we check the flow of each edge. If the edge is not saturated, we traverse
		// it. The Traverse enum will collect the edge in the results. Otherwise we
		// skip the edge.
		let mut max_flow: i64 = 0;
		while let Some(b) = g.depth_first(&1, &6,
			|e| {
				let flow = e.load();
				if flow.cur < flow.max { Traverse } else { Skip }
		})
		{
			// We backtrack the results from the breadth first traversal which will
			// produce the shortest path.
			let path = b.backtrack().unwrap();

			// We find the bottleneck value of the path.
			let mut aug_flow = std::i64::MAX;
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

	#[test]
	fn digraph_test_maximum_flow_edmonds_karp() {
		let g = flow_graph_example();
		let max_flow = maximum_flow_edmonds_karp(&g);
		println!("{}", max_flow);
		// assert!(max_flow == 23);
	}
}
