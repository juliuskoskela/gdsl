#[test]
fn test_digraph_new() {
	use ggi::graph::digraph::{DiGraph, DiNode};
	use ggi::Empty;

	let mut g = DiGraph::<usize, Empty, Empty>::new();

	g.insert(DiNode::new(0, Empty));
	g.insert(DiNode::new(1, Empty));

	g[0].connect(&g[1], Empty)
}

#[test]
fn test_graph_macro() {
	use ggi::*;

	// BiGraph<K, _, _>
	let bigraph = graph![
		(&str)
		("A") : ["B", "C"]
		("B") : ["C"]
		("C") : ["D"]
		("D") : []
	];

	// BiGraph<K, N, _>
	let bigraph_n = graph![
		(&str, i32)
		("A", 42) : ["B", "C"]
		("B", 42) : ["C"]
		("C", 42) : ["D"]
		("D", 42) : []
	];

	// BiGraph<K, _, E>
	let bigraph_e = graph![
		(&str) : [i32]
		("A") : [("B", 42), ("C", 42)]
		("B") : [("C", 42)]
		("C") : [("D", 42)]
		("D") : []
	];

	// BiGraph<K, N, E>
	let bigraph_ne = graph![
		(&str, i32) : [f64]
		("A", 42) : [("B", 3.14), ("C", 3.14), ("D", 3.14)]
		("B", 42) : [("C", 3.14), ("D", 3.14)]
		("C", 42) : [("D", 3.14)]
		("D", 42) : []
	];

	// DiGraph<K, _, _>
	let digraph = graph![
		(&str)
		("A") => ["B", "C"]
		("B") => ["C"]
		("C") => ["D"]
		("D") => []
	];

	// DiGraph<K, N, _>
	let digraph_n = graph![
		(&str, i32)
		("A", 42) => ["B", "C"]
		("B", 42) => ["C"]
		("C", 42) => ["D"]
		("D", 42) => []
	];

	// DiGraph<K, _, E>
	let digraph_e = graph![
		(&str) => [i32]
		("A") => [("B", 42), ("C", 42)]
		("B") => [("C", 42)]
		("C") => [("D", 42)]
		("D") => []
	];

	// DiGraph<K, N, E>
	let digraph_ne = graph![
		(&str, i32) => [f64]
		("A", 42) => [("B", 3.14), ("C", 3.14), ("D", 3.14)]
		("B", 42) => [("C", 3.14), ("D", 3.14)]
		("C", 42) => [("D", 3.14)]
		("D", 42) => []
	];

	assert_eq!(bigraph.len(), 4);
	assert_eq!(bigraph_n.len(), 4);
	assert_eq!(bigraph_e.len(), 4);
	assert_eq!(bigraph_ne.len(), 4);
	assert_eq!(digraph.len(), 4);
	assert_eq!(digraph_n.len(), 4);
	assert_eq!(digraph_e.len(), 4);
	assert_eq!(digraph_ne.len(), 4);
}

#[test]
fn test_digraph_bfs() {
	use ggi::*;

	let g = graph![(usize)
		(0) => [1, 2, 3]
		(1) => [3]
		(2) => [4]
		(3) => [2]
		(4) => []
	];

	if let Some(bfs) = g[0].search().bfs(&g[4]) {
		let path = bfs.node_path();
		for node in &path {
			println!("{}", node.key());
		}
	}
}

#[test]
fn test_digraph_dfs() {
	use ggi::*;

	let g = graph![(usize)
		(0) => [1, 2, 3]
		(1) => [3]
		(2) => [4]
		(3) => [2]
		(4) => []
	];

	if let Some(dfs) = g[0].search().dfs_map(&g[4], &|_, _, _| {true}) {
		let path = dfs.node_path();
		for node in &path {
			println!("{}", node.key());
		}
	}
}

#[test]
fn digraph_search() {
	use ggi::*;

	let g = graph![(usize)
		(0) => [1, 2, 3]
		(1) => [5]
		(2) => [1, 4, 5]
		(3) => [4, 6]
		(4) => [5]
		(5) => []
		(6) => []
	];

	assert!(g[0].search().bfs(&g[5]).is_some());
	assert!(g[0].search().dfs(&g[5]).is_some());

	if let Some(bfs) = g[0].search().bfs(&g[5]) {
		let path = bfs.node_path();
		assert!(path[0] == g[0]);
		assert!(path[1] == g[1]);
		assert!(path[2] == g[5]);
	} else {
		panic!();
	}

	if let Some(bfs) = g[0].search().bfs_map(&g[5], &|_, _, _| {true}) {
		let path = bfs.node_path();
		assert!(path[0] == g[0]);
		assert!(path[1] == g[1]);
		assert!(path[2] == g[5]);
	} else {
		panic!();
	}

	if let Some(dfs) = g[0].search().dfs(&g[5])  {
		let path = dfs.node_path();
		assert!(path[0] == g[0]);
		assert!(path[1] == g[3]);
		assert!(path[2] == g[4]);
		assert!(path[3] == g[5]);
	} else {
		panic!();
	}

	if let Some(dfs) = g[0].search().dfs_map(&g[5], &|_, _, _| {true})  {
		let path = dfs.node_path();
		assert!(path[0] == g[0]);
		assert!(path[1] == g[3]);
		assert!(path[2] == g[4]);
		assert!(path[3] == g[5]);
	} else {
		panic!();
	}
}

#[test]
fn digraph_search_pfs() {
	use ggi::*;

	let g = graph![(&str, u64)
		("A", 0) => ["B", "C", "D"]
		("B", 1) => ["E"]
		("C", 2) => ["G"]
		("D", 3) => ["F"]
		("E", 0) => ["G"]
		("F", 0) => ["B", "G"]
		("G", 0) => []
	];

	println!("\nDFS:\n");

	if let Some(dfs) = g["A"].search().dfs(&g["G"]) {
		let path = dfs.node_path();
		for node in &path {
			println!("{}", node.key());
		}
	} else {
		panic!();
	}

	println!("\nBFS:\n");

	if let Some(bfs) = g["A"].search().bfs(&g["G"]) {
		let path = bfs.node_path();
		for node in &path {
			println!("{}", node.key());
		}
	} else {
		panic!();
	}

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
