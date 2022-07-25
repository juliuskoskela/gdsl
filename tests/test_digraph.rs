#[test]
fn test_node_new() {
	use dug::digraph::*;

	let node = DiNode::<&str, &str, usize>::new("key", "val");

	assert!(*node.key() == "key");
	assert!(*node == "val");
}

#[test]
fn test_graph_macro() {
	use dug::*;

	let digraph = graph![
		(&str)
		("A") => ["B", "C"]
		("B") => ["C"]
		("C") => ["D"]
		("D") => []
	];

	let digraph_n = graph![
		(&str, i32)
		("A", 42) => ["B", "C"]
		("B", 42) => ["C"]
		("C", 42) => ["D"]
		("D", 42) => []
	];

	let digraph_e = graph![
		(&str) => [i32]
		("A") => [("B", 42), ("C", 42)]
		("B") => [("C", 42)]
		("C") => [("D", 42)]
		("D") => []
	];

	let digraph_ne = graph![
		(&str, i32) => [f64]
		("A", 42) => [("B", 3.14), ("C", 3.14), ("D", 3.14)]
		("B", 42) => [("C", 3.14), ("D", 3.14)]
		("C", 42) => [("D", 3.14)]
		("D", 42) => []
	];

	assert_eq!(digraph.len(), 4);
	assert_eq!(digraph_n.len(), 4);
	assert_eq!(digraph_e.len(), 4);
	assert_eq!(digraph_ne.len(), 4);
}

#[test]
fn test_digraph_bfs() {
	use dug::*;

	let g = graph![(usize)
		(0) => [1, 2, 3]
		(1) => [3]
		(2) => [4]
		(3) => [2]
		(4) => []
	];

	if let Some(bfs) = g[0].search().bfs(&g[4]) {
		let path = bfs.node_path();
		assert!(path[0] == g[0]);
		assert!(path[1] == g[2]);
		assert!(path[2] == g[4]);
	} else {
		panic!();
	}
}

#[test]
fn test_digraph_dfs() {
	use dug::*;

	let g = graph![(usize)
		(0) => [1, 2, 3]
		(1) => [3]
		(2) => [4]
		(3) => [2]
		(4) => []
	];

	if let Some(dfs) = g[0].search().dfs_map(&g[4], &|_, _, _| {true}) {
		let path = dfs.node_path();
		assert!(path[0] == g[0]);
		assert!(path[1] == g[1]);
		assert!(path[2] == g[3]);
		assert!(path[3] == g[2]);
		assert!(path[4] == g[4]);
	} else {
		panic!();
	}
}

#[test]
fn digraph_search_pfs() {
	use dug::*;

	let g = graph![(&str, u64)
		("A", 0) => ["B", "C", "D"]
		("B", 1) => ["E"]
		("C", 2) => ["G"]
		("D", 3) => ["F"]
		("E", 0) => ["G"]
		("F", 0) => ["B", "G"]
		("G", 0) => []
	];

	println!("\nPFS_MIN:\n");

	if let Some(pfs) = g["A"].search().pfs_min(&g["G"])  {
		let path = pfs.node_path();
		for node in path {
			println!("{}", node.key());
		}
	} else {
		panic!();
	}

	println!("\nPFS_MAX:\n");

	if let Some(pfs) = g["A"].search().pfs_max(&g["G"])  {
		let path = pfs.node_path();
		for node in path {
			println!("{}", node.key());
		}
	} else {
		panic!();
	}
}
