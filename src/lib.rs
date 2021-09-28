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
	use crate::global::Traverse::*;
	use crate::digraph::*;
	use crate::examples::*;

	type SimpleGraph = Digraph<usize, Void, Void>;

	#[test]
	fn digraph_test_breadth_traversal() {
		let mut g = SimpleGraph::new();

		g.insert(1, Void);
		g.insert(2, Void);
		g.insert(3, Void);
		g.insert(4, Void);
		g.insert(5, Void);
		g.insert(6, Void);

		g.connect(&1, &2, Void);
		g.connect(&1, &3, Void);
		g.connect(&2, &1, Void);
		g.connect(&2, &3, Void);
		g.connect(&3, &1, Void);
		g.connect(&3, &5, Void);
		g.connect(&5, &2, Void);
		g.connect(&5, &4, Void);
		g.connect(&5, &1, Void);
		g.connect(&4, &5, Void);
		g.connect(&4, &3, Void);
		g.connect(&4, &2, Void);
		g.connect(&4, &6, Void);

		let path = g.breadth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Finish
			} else {
				Traverse
			}
		}).unwrap().backtrack().unwrap();

		for edge in path.iter() {
			println!("{}", edge.upgrade().unwrap());
		}
	}

	fn flow_graph_example_1to6_23() -> FlowGraph {
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
		let g = flow_graph_example_1to6_23();
		let max_flow = maximum_flow_edmonds_karp(&g, 1, 6);
		assert!(max_flow == 23);
	}

	#[test]
	fn digraph_test_maximum_flow_ford_fulkerson() {
		let g = flow_graph_example_1to6_23();
		let max_flow = maximum_flow_ford_fulkerson(&g, 1, 6);
		println!("max flow = {}", max_flow);
		// assert!(max_flow == 23);
	}
}
