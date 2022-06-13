// use fastgraph::core::*;
use fastgraph::collections::*;

fn create_graph() -> Ungraph<usize, usize, usize> {
	let mut g = Ungraph::<usize, usize, usize>::new();
	g.add_node(0, std::usize::MAX);
	g.add_node(1, std::usize::MAX);
	g.add_node(2, std::usize::MAX);
	g.add_node(3, std::usize::MAX);
	g.add_node(4, std::usize::MAX);
	g.add_node(5, std::usize::MAX);
	g.add_node(6, std::usize::MAX);
	g.add_node(7, std::usize::MAX);
	g.add_node(8, std::usize::MAX);
	g.add_edge(0, 1, 4);
	g.add_edge(0, 7, 8);
	g.add_edge(1, 2, 8);
	g.add_edge(1, 7, 11);
	g.add_edge(7, 8, 7);
	g.add_edge(7, 6, 1);
	g.add_edge(8, 6, 6);
	g.add_edge(2, 8, 2);
	g.add_edge(2, 5, 4);
	g.add_edge(2, 3, 7);
	g.add_edge(3, 4, 9);
	g.add_edge(3, 5, 14);
	g.add_edge(5, 4, 10);
	g
}

#[test]
fn djikstra() {
	let g = create_graph();

	// let stree = g.breadth_first(0, |e| {
	// 	Traverse::Include
	// });
	g.print_graph();
}