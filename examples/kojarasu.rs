// # Kosaraju's algorithm
//
// Kosaraju's algorithm is a linear time algorithm for finding the strongly
// connected components of a graph.
//
// https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
#![allow(unused)]
use gdsl::digraph::{graph, DiNode, DiGraph, Empty};
use std::collections::HashSet;

type Node = DiNode<usize, Empty, Empty>;
type Graph = DiGraph<usize, Empty, Empty>;

fn ordering(graph: &Graph) -> Vec<Node> {
	let mut visited = HashSet::new();
	let mut ordering = Vec::new();
	for (_, root) in graph.iter() {
		if !visited.contains(root.key()) {
			let partition = root
				.order()
				.post()
				.filter(&|_, v, _| !visited.contains(v.key()))
				.collect_nodes();
			for node in &partition {
				visited.insert(node.key().clone());
				ordering.push(node.clone());
			}
		}
	}
	ordering
}

fn kojarasu(graph: &Graph) -> Vec<Vec<Node>> {
	let mut invariant = HashSet::new();
	let mut components = Vec::new();
	let mut ordering = ordering(graph);

	while let Some(node) = ordering.pop() {
		if !invariant.contains(node.key()) {
			let cycle = node
				.dfs()
				.transpose()
				.filter(&|_, v, _| !invariant.contains(v.key()))
				.cycle();
			match cycle {
				Some(cycle) => {
					for node in &cycle {
						invariant.insert(node.key().clone());
					}
					components.push(cycle);
				},
				None => {
					invariant.insert(node.key().clone());
					components.push(vec![node.clone()]);
				},
			}
		}
	}
	components
}

// ## SCC Example 1
//
// https://www.programiz.com/dsa/strongly-connected-components
//
// Exapected SCC's:
//
// 0: [ 0 1 2 3 ]
// 1: [ 4 5 6 ]
// 2: [ 7 ]
fn ex1() {
	let g = graph![
		(usize) =>
		(0) => [1]
		(1) => [2]
		(2) => [3, 4]
		(3) => [0]
		(4) => [5]
		(5) => [6]
		(6) => [4, 7]
		(7) => []
	];

	let expect = vec![
		vec![0, 1, 2, 3],
		vec![4, 5, 6],
		vec![7],
	];

	let mut components = kojarasu(&g);

	for (i, component) in components.iter_mut().enumerate() {
		component.sort_by(|a, b| a.key().cmp(&b.key()));
		let keys = component.iter().map(|node| node.key().clone()).collect::<Vec<_>>();
		assert_eq!(keys, expect[i]);
	}
}

// ## SCC Example 2
//
// https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
//
// Exapected SCC's:
//
// 0: [ 8 ]
// 1: [ 4 5 ]
// 2: [ 6 7 ]
// 3: [ 1 2 3 ]
fn ex2() {
	let g = graph![
		(usize) =>
		(1) => [2]
		(2) => [3]
		(3) => [1]
		(4) => [2, 3, 5]
		(5) => [4, 6]
		(6) => [7]
		(7) => [6]
		(8) => [5, 7, 8]
	];

	let expect = vec![
		vec![8],
		vec![4, 5],
		vec![6, 7],
		vec![1, 2, 3],
	];

	let mut components = kojarasu(&g);

	for (i, component) in components.iter_mut().enumerate() {
		component.sort_by(|a, b| a.key().cmp(&b.key()));
		let keys = component.iter().map(|node| node.key().clone()).collect::<Vec<_>>();
		assert_eq!(keys, expect[i]);
	}
}

// ## SCC Example 3
//
// https://iq.opengenus.org/tarjans-algorithm/
//
// Exapected SCC's:
//
// 0: [ 1 2 3 ]
// 1: [ 4 ]
// 2: [ 5 6 7 8 ]
fn ex3() {
	let g = graph![
		(usize) =>
		(1) => [3]
		(2) => [1]
		(3) => [2, 4]
		(4) => [5]
		(5) => [6]
		(6) => [7]
		(7) => [8]
		(8) => [5]
	];

	let expect = vec![
		vec![1, 2, 3],
		vec![4],
		vec![5, 6, 7, 8],
	];

	let mut components = kojarasu(&g);

	for (i, component) in components.iter_mut().enumerate() {
		component.sort_by(|a, b| a.key().cmp(&b.key()));
		let keys = component.iter().map(|node| node.key().clone()).collect::<Vec<_>>();
		assert_eq!(keys, expect[i]);
	}
}

// ## SCC Example 4
//
// https://www.youtube.com/watch?v=TyWtx7q2D7Y
//
// Exapected SCC's:
//
// 0: [ 3 7 ]
// 1: [ 4 5 6 ]
// 2: [ 0 1 2 ]
fn ex4() {
	let g = graph![
		(usize) =>
		(0) => [1]
		(1) => [2]
		(2) => [0]
		(3) => [4, 7]
		(4) => [5]
		(5) => [6, 0]
		(6) => [0, 2, 4]
		(7) => [3, 5]
	];

	let expect = vec![
		vec![3, 7],
		vec![4, 5, 6],
		vec![0, 1, 2],
	];

	let mut components = kojarasu(&g);

	for (i, component) in components.iter_mut().enumerate() {
		component.sort_by(|a, b| a.key().cmp(&b.key()));
		let keys = component.iter().map(|node| node.key().clone()).collect::<Vec<_>>();
		assert_eq!(keys, expect[i]);
	}
}

fn main() {
	ex1();
	ex2();
	ex3();
	ex4();
}
