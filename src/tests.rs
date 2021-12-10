use crate::core::*;
use crate::digraph::*;
use crate::examples::*;

type SimpleGraph = Digraph<usize, Empty, Empty>;

#[test]
fn digraph_test_breadth_traversal() {
	let g = test_digraph_1();

	let res = g.breadth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();

	let path = backtrack_edges(&res);
	println!("Breadth First Search\n");
	for edge in path.iter() {
		println!("{}", edge.upgrade().unwrap());
	}
}

#[test]
fn digraph_test_par_breadth_traversal() {
	let g = test_digraph_1();

	let res = g.par_breadth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();

	let path = backtrack_edges(&res);
	println!("Parallel Breadth First Search\n");
	for edge in path.iter() {
		println!("{}", edge.upgrade().unwrap());
	}
	g.print_nodes();
}

#[test]
fn digraph_test_depth_traversal() {
	let g = test_digraph_1();

	let res = g.depth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();

	let path = backtrack_edges(&res);
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
	let max_flow = parallel_maximum_flow_edmonds_karp(&g, 1, 6);
	g.print_nodes();
	g.print_edges();
	println!("MAX FLOW = {}", max_flow);
	// assert!(max_flow == 23);
}

// Test graph constructors

fn test_digraph_1() -> SimpleGraph {
	let mut g = SimpleGraph::new();

	g.insert(1, Empty);
	g.insert(2, Empty);
	g.insert(3, Empty);
	g.insert(4, Empty);
	g.insert(5, Empty);
	g.insert(6, Empty);

	g.connect(&1, &2, Empty);
	g.connect(&1, &3, Empty);
	g.connect(&2, &1, Empty);
	g.connect(&2, &3, Empty);
	g.connect(&3, &1, Empty);
	g.connect(&3, &5, Empty);
	g.connect(&5, &2, Empty);
	g.connect(&5, &4, Empty);
	g.connect(&5, &1, Empty);
	g.connect(&4, &5, Empty);
	g.connect(&4, &3, Empty);
	g.connect(&4, &2, Empty);
	g.connect(&4, &6, Empty);
	g
}

fn flow_graph_example_1to6_23() -> FlowGraph {
	let mut g = FlowGraph::new();
	g.insert(1, Empty);
	g.insert(2, Empty);
	g.insert(3, Empty);
	g.insert(4, Empty);
	g.insert(5, Empty);
	g.insert(6, Empty);
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