use crate::core::*;
use crate::graph::*;
use crate::examples::*;

type SimpleDigraph = Digraph<usize, Empty, Empty>;
type SimpleUngraph = Ungraph<usize, Empty, Empty>;

#[test]
fn ungraph_test_breadth_traversal() {
	let g = test_ungraph_1();

	let res1 = g.breadth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();
	let res2 = g.breadth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		}).unwrap();
	let path1 = backtrack_edges(&res1);
	assert!(path1.len() == 4);
	assert!(path1[0].upgrade().unwrap().target().key() == &3);
	assert!(path1[1].upgrade().unwrap().target().key() == &5);
	assert!(path1[2].upgrade().unwrap().target().key() == &4);
	assert!(path1[3].upgrade().unwrap().target().key() == &6);
	let path2 = backtrack_edges(&res2);
	assert!(path2.len() == 4);
	assert!(path2[0].upgrade().unwrap().target().key() == &3);
	assert!(path2[1].upgrade().unwrap().target().key() == &5);
	assert!(path2[2].upgrade().unwrap().target().key() == &4);
	assert!(path2[3].upgrade().unwrap().target().key() == &6);
}

#[test]
fn ungraph_test_par_breadth_traversal() {
	let g = test_ungraph_1();

	let res1 = g.par_breadth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();
	let res2 = g.par_breadth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		}).unwrap();
	let path1 = backtrack_edges(&res1);
	assert!(path1.len() == 4);
	assert!(path1[0].upgrade().unwrap().target().key() == &3);
	assert!(path1[1].upgrade().unwrap().target().key() == &5);
	assert!(path1[2].upgrade().unwrap().target().key() == &4);
	assert!(path1[3].upgrade().unwrap().target().key() == &6);
	let path2 = backtrack_edges(&res2);
	assert!(path2.len() == 4);
	assert!(path2[0].upgrade().unwrap().target().key() == &3);
	assert!(path2[1].upgrade().unwrap().target().key() == &5);
	assert!(path2[2].upgrade().unwrap().target().key() == &4);
	assert!(path2[3].upgrade().unwrap().target().key() == &6);
}

#[test]
fn digraph_test_breadth_traversal() {
	let g = test_digraph_1();

	let res1 = g.breadth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();
	let res2 = g.breadth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		}).unwrap();
	let path1 = backtrack_edges(&res1);
	assert!(path1.len() == 4);
	assert!(path1[0].upgrade().unwrap().target().key() == &3);
	assert!(path1[1].upgrade().unwrap().target().key() == &5);
	assert!(path1[2].upgrade().unwrap().target().key() == &4);
	assert!(path1[3].upgrade().unwrap().target().key() == &6);
	let path2 = backtrack_edges(&res2);
	assert!(path2.len() == 4);
	assert!(path2[0].upgrade().unwrap().target().key() == &3);
	assert!(path2[1].upgrade().unwrap().target().key() == &5);
	assert!(path2[2].upgrade().unwrap().target().key() == &4);
	assert!(path2[3].upgrade().unwrap().target().key() == &6);
}

#[test]
fn digraph_test_par_breadth_traversal() {
	let g = test_digraph_1();

	let res1 = g.par_breadth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();
	let res2 = g.par_breadth_first(&1,
		|edge|{
			if edge.target().key() == &6 {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		}).unwrap();
	let path1 = backtrack_edges(&res1);
	assert!(path1.len() == 4);
	assert!(path1[0].upgrade().unwrap().target().key() == &3);
	assert!(path1[1].upgrade().unwrap().target().key() == &5);
	assert!(path1[2].upgrade().unwrap().target().key() == &4);
	assert!(path1[3].upgrade().unwrap().target().key() == &6);
	let path2 = backtrack_edges(&res2);
	assert!(path2.len() == 4);
	assert!(path2[0].upgrade().unwrap().target().key() == &3);
	assert!(path2[1].upgrade().unwrap().target().key() == &5);
	assert!(path2[2].upgrade().unwrap().target().key() == &4);
	assert!(path2[3].upgrade().unwrap().target().key() == &6);
}

#[test]
fn digraph_test_maximum_flow_edmonds_karp() {
	let g = flow_graph_example_1to6_23();
	let max_flow = parallel_maximum_flow_edmonds_karp(&g, 1, 6);
	assert!(max_flow == 23);
}

// Test graph constructors

fn test_ungraph_1() -> SimpleDigraph {
	let mut g = SimpleDigraph::new();

	g.add_node(1, Empty);
	g.add_node(2, Empty);
	g.add_node(3, Empty);
	g.add_node(4, Empty);
	g.add_node(5, Empty);
	g.add_node(6, Empty);

	g.add_edge(&1, &2, Empty);
	g.add_edge(&1, &3, Empty);
	g.add_edge(&2, &1, Empty);
	g.add_edge(&2, &3, Empty);
	g.add_edge(&3, &1, Empty);
	g.add_edge(&3, &5, Empty);
	g.add_edge(&5, &2, Empty);
	g.add_edge(&5, &4, Empty);
	g.add_edge(&5, &1, Empty);
	g.add_edge(&4, &5, Empty);
	g.add_edge(&4, &3, Empty);
	g.add_edge(&4, &2, Empty);
	g.add_edge(&4, &6, Empty);
	g
}

fn test_digraph_1() -> SimpleUngraph {
	let mut g = SimpleUngraph::new();

	g.add_node(1, Empty);
	g.add_node(2, Empty);
	g.add_node(3, Empty);
	g.add_node(4, Empty);
	g.add_node(5, Empty);
	g.add_node(6, Empty);

	g.add_edge(&1, &2, Empty);
	g.add_edge(&1, &3, Empty);
	g.add_edge(&2, &1, Empty);
	g.add_edge(&2, &3, Empty);
	g.add_edge(&3, &1, Empty);
	g.add_edge(&3, &5, Empty);
	g.add_edge(&5, &2, Empty);
	g.add_edge(&5, &4, Empty);
	g.add_edge(&5, &1, Empty);
	g.add_edge(&4, &5, Empty);
	g.add_edge(&4, &3, Empty);
	g.add_edge(&4, &2, Empty);
	g.add_edge(&4, &6, Empty);
	g
}

fn flow_graph_example_1to6_23() -> FlowGraph {
	let mut g = FlowGraph::new();
	g.add_node(1, Empty);
	g.add_node(2, Empty);
	g.add_node(3, Empty);
	g.add_node(4, Empty);
	g.add_node(5, Empty);
	g.add_node(6, Empty);
	add_edge_flow(&mut g, &1, &2, 16);
	add_edge_flow(&mut g, &1, &3, 13);
	add_edge_flow(&mut g, &2, &3, 10);
	add_edge_flow(&mut g, &2, &4, 12);
	add_edge_flow(&mut g, &3, &2, 4);
	add_edge_flow(&mut g, &3, &5, 14);
	add_edge_flow(&mut g, &4, &3, 9);
	add_edge_flow(&mut g, &4, &6, 20);
	add_edge_flow(&mut g, &5, &4, 7);
	add_edge_flow(&mut g, &5, &6, 4);
	g
}