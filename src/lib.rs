pub mod digraph;
pub mod node;
pub mod edge;
pub mod adjacent;
pub mod global;
pub mod edge_list;
pub mod examples;

#[cfg(test)]
mod tests {
	use crate::global::*;
	use crate::examples::*;
	// Digraph Test: Maximum Flow Edmond's Karp

	// A struct which records the maximum flow and current flow for an edge
	// and stores a weak pointer to the reverse edge. We store the reverse
	// edge so we don't have to search for it in the outbound edges of the
	// edge's target node.

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

	#[test]
	fn digraph_test_maximum_flow_edmonds_karp() {
		let g = flow_graph_example();
		let max_flow = maximum_flow_edmonds_karp(&g);
		assert!(max_flow == 23);
	}
}
