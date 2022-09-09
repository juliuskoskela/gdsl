// Prim's algorithm
//
// Prim's algorithm (also known as Jarník's algorithm) is a greedy algorithm
// that finds a minimum spanning tree for a weighted undirected graph. This
// means it finds a subset of the edges that forms a tree that includes every
// vertex, where the total weight of all the edges in the tree is minimized.
// The algorithm operates by building this tree one vertex at a time,
// from an arbitrary starting vertex, at each step adding the cheapest possible
// connection from the tree to another vertex.
// 
// https://en.wikipedia.org/wiki/Prim%27s_algorithm
//

use gdsl::ungraph::*;
use gdsl::*;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
use std::cell::RefCell;

type N = Node<usize, (), u64>;
type E = Edge<usize, (), u64>;
type Heap = BinaryHeap<Reverse<E>>;

fn prim_minimum_spanning_tree(s: &N) -> Vec<E> {
    let mut forest: Vec<E> = vec![];
    let mut added_nodes: HashSet<usize> = HashSet::new();
	let heap = RefCell::new(Heap::new());

	added_nodes.insert(*s.key());

	// Add all edges reachable from s to a Min Heap
	s.bfs().map(&|edge| {
		heap.borrow_mut().push(Reverse(edge.clone()));
	}).search();

	// While the heap is not empty, search for the next edge
	// that connects a node in the tree to a node not in the tree.
	//
	let mut tmp: Vec<E> = vec![];
	loop {
		if let Some(edge) = heap.borrow_mut().pop() {
			let Reverse(Edge(u, v, e)) = edge;
			let u_in_tree = added_nodes.contains(u.key());
			let v_in_tree = added_nodes.contains(v.key());
			if u_in_tree && !v_in_tree {
				added_nodes.insert(*v.key());
				forest.push(Edge(u, v, e));
			} else if !u_in_tree && !v_in_tree {
				tmp.push(Edge(u, v, e));
				continue;
			} else {
				continue;
			}
		} else {
			break;
		}
		for tmp_edge in &tmp {
			heap.borrow_mut().push(Reverse(tmp_edge.clone()));
		}
	}
	forest
}

fn main() {
	// Example g1 from Wikipedia
	let g1 = ungraph![
		(usize) => [u64]
		(0) => [ (1, 1), (3, 4), (4, 3)]
		(1) => [ (3, 4), (4, 2)]
		(2) => [ (4, 4), (5, 5)]
		(3) => [ (4, 4)]
		(4) => [ (5, 7)]
		(5) => []
	];
	let forest = prim_minimum_spanning_tree(&g1[0]);
	let sum = forest.iter().fold(0, |acc, e| acc + e.2);
	assert!(sum == 16);
	
	// Example g2 from Figure 7.1 in https://jeffe.cs.illinois.edu/teaching/algorithms/book/07-mst.pdf
	let g2 = ungraph![
		(usize) => [u64]
		(0) => [ (1, 8), (2, 5)]
		(1) => [ (2, 10), (3, 2), (4, 18)]
		(2) => [ (3, 3), (5, 16)]
		(3) => [ (4, 12), (5, 30)]
		(4) => [ (6, 4)]
		(5) => [ (6, 26)]
		(6) => []
	];
	let forest = prim_minimum_spanning_tree(&g2[0]);
	let sum = forest.iter().fold(0, |acc, e| acc + e.2);
	assert!(sum == 42);
}
