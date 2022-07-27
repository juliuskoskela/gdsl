
#[test]
fn test_graph_macro() {
	use gdsl::*;

	let g1 = graph![
		(&str) =>
		("A") => ["B", "C"]
		("B") => ["C"]
		("C") => ["D"]
		("D") => []
	];

	let g2 = graph![
		(&str, i32) =>
		("A", 42) => ["B", "C"]
		("B", 42) => ["C"]
		("C", 42) => ["D"]
		("D", 42) => []
	];

	let g3 = graph![
		(&str) => [i32]
		("A") => [("B", 42), ("C", 42)]
		("B") => [("C", 42)]
		("C") => [("D", 42)]
		("D") => []
	];

	let g4 = graph![
		(&str, i32) => [f64]
		("A", 42) => [("B", 3.14), ("C", 3.14), ("D", 3.14)]
		("B", 42) => [("C", 3.14), ("D", 3.14)]
		("C", 42) => [("D", 3.14)]
		("D", 42) => []
	];

	assert_eq!(g1.len(), 4);
	assert_eq!(g2.len(), 4);
	assert_eq!(g3.len(), 4);
	assert_eq!(g4.len(), 4);
}

#[test]
fn test_digraph_ungraph() {
	use gdsl::*;

	let digraph = graph![
		(char) =>
		('A') => ['C']
		('B') => ['E', 'A']
		('C') => ['D', 'B']
		('D') => ['E']
		('E') => []
	];

	let ungraph = graph![
		(char) :
		('A') : ['C']
		('B') : ['E', 'A']
		('C') : ['D', 'B']
		('D') : ['E']
		('E') : []
	];

	if let Some(bfs) = digraph['A'].bfs_path().search(Some(&digraph['E'])) {
		let path = bfs.node_path();
		assert!(path[0] == digraph['A']);
		assert!(path[1] == digraph['C']);
		assert!(path[2] == digraph['D']);
		assert!(path[3] == digraph['E']);
	}

	print!("\n");

	if let Some(bfs) = ungraph['A'].bfs_path().search(Some(&ungraph['E'])) {
		let path = bfs.node_path();
		assert!(path[0] == ungraph['A']);
		assert!(path[1] == ungraph['B']);
		assert!(path[2] == ungraph['E']);
	}
}

#[test]
fn test_digraph_bfs() {
	use gdsl::*;

	let g = graph![
		(usize) =>
		(0) => [1, 2, 3]
		(1) => [3]
		(2) => [4]
		(3) => [2]
		(4) => []
	];

	if let Some(bfs) = g[0].bfs_path().search(Some(&g[4])) {
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
	use gdsl::*;

	let g = graph![
		(usize) =>
		(0) => [1, 2, 3]
		(1) => [3]
		(2) => [4]
		(3) => [2]
		(4) => []
	];

	if let Some(dfs) = g[0].dfs_path().search(Some(&g[4])) {
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
