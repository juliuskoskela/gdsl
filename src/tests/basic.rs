#[test]
fn graph_search() {
	use crate::*;

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

	if let Some(bfs) = g[0].search().bfs_map(&g[5], &|_| {true}) {
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

	if let Some(dfs) = g[0].search().dfs_map(&g[5], &|_| {true})  {
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
fn graph_search_pfs() {
	use crate::*;

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
