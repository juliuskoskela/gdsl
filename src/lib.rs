pub mod digraph;
pub mod node;
pub mod edge;
pub mod adjacent;
pub mod global;
pub mod path;
pub mod examples;

#[cfg(test)]
mod tests {
	use crate::global::*;
	use crate::global::Traverse::*;
	use crate::digraph::*;
	use crate::examples::*;

	type SimpleGraph = Digraph<usize, Null, Null>;

	#[test]
	fn digraph_test_breadth_traversal() {
		let g = test_digraph_1();

		let path = g.breadth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Finish
			} else {
				Include
			}
		}).unwrap().backtrack().unwrap();

		println!("Breadth First Search\n");
		for edge in path.iter() {
			println!("{}", edge.upgrade().unwrap());
		}
	}

	#[test]
	fn digraph_test_par_breadth_traversal() {
		let g = test_digraph_1();

		let path = g.par_breadth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Finish
			} else {
				Include
			}
		}).unwrap().backtrack().unwrap();

		println!("Breadth First Search\n");
		for edge in path.iter() {
			println!("{}", edge.upgrade().unwrap());
		}
		g.print_nodes();
	}

	#[test]
	fn digraph_test_depth_traversal() {
		let g = test_digraph_1();

		let path = g.depth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Finish
			} else {
				Include
			}
		}).unwrap().backtrack().unwrap();

		println!("\nDepth First Search\n");
		for edge in path.iter() {
			println!("{}", edge.upgrade().unwrap());
		}
		for (_, n) in g.nodes.iter() {
			println!("{}", n);
		}
	}

	#[test]
	fn digraph_test_maximum_flow_edmonds_karp() {
		let g = flow_graph_example_1to6_23();
		let max_flow = maximum_flow_edmonds_karp(&g, 1, 6);
		g.print_nodes();
		g.print_edges();
		assert!(max_flow == 23);
	}

	#[test]
	fn digraph_test_maximum_flow_ford_fulkerson() {
		let g = flow_graph_example_1to6_23();
		let max_flow = maximum_flow_ford_fulkerson(&g, 1, 6);
		println!("max flow = {}", max_flow);
	}

	#[test]
	fn digraph_colouring() {
		type ChromaticGraph = Digraph<usize, usize>;

		let mut g = ChromaticGraph::new();

		g.insert(1, 0);
		g.insert(2, 0);
		g.insert(3, 0);
		g.insert(4, 0);
		g.connect(&1, &2,  Null);
		g.connect(&1, &3,  Null);
		// g.connect(&1, &4,  Null);

		g.connect(&2, &1,  Null);
		g.connect(&2, &3,  Null);
		// g.connect(&2, &4,  Null);

		g.connect(&3, &1,  Null);
		g.connect(&3, &2,  Null);
		// g.connect(&3, &4,  Null);

		g.connect(&4, &1,  Null);
		g.connect(&4, &2,  Null);
		// g.connect(&4, &3,  Null);

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

	// Test graph constructors

	fn test_digraph_1() -> SimpleGraph {
		let mut g = SimpleGraph::new();

		g.insert(1, Null);
		g.insert(2, Null);
		g.insert(3, Null);
		g.insert(4, Null);
		g.insert(5, Null);
		g.insert(6, Null);

		g.connect(&1, &2, Null);
		g.connect(&1, &3, Null);
		g.connect(&2, &1, Null);
		g.connect(&2, &3, Null);
		g.connect(&3, &1, Null);
		g.connect(&3, &5, Null);
		g.connect(&5, &2, Null);
		g.connect(&5, &4, Null);
		g.connect(&5, &1, Null);
		g.connect(&4, &5, Null);
		g.connect(&4, &3, Null);
		g.connect(&4, &2, Null);
		g.connect(&4, &6, Null);
		g
	}

	fn flow_graph_example_1to6_23() -> FlowGraph {
		let mut g = FlowGraph::new();
		g.insert(1, Null);
		g.insert(2, Null);
		g.insert(3, Null);
		g.insert(4, Null);
		g.insert(5, Null);
		g.insert(6, Null);
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
}
