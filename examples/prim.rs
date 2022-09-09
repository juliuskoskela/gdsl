use gdsl::ungraph::*;
use gdsl::*;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
use std::cell::RefCell;

type N = Node<usize, (), u64>;
type E = Edge<usize, (), u64>;
type Heap = BinaryHeap<Reverse<E>>;
type Forest = Vec<E>;

fn prim_minimum_spanning_tree(s: &N) -> Forest {
    let mut forest: Forest = vec![];
    let mut added_nodes: HashSet<usize> = HashSet::new();
	let heap = RefCell::new(Heap::new());

	added_nodes.insert(*s.key());

	s.bfs().map(&|edge| {
		heap.borrow_mut().push(Reverse(edge.clone()));
	}).search();

	let mut tmp: Vec<E> = vec![];
	loop {
		if let Some(edge) = heap.borrow_mut().pop() {
			let Reverse(edge) = edge;
			if added_nodes.contains(edge.source().key())
			&& !added_nodes.contains(edge.target().key()) {
				forest.push(edge.clone());
				added_nodes.insert(*edge.target().key());
			} else {
				tmp.push(edge);
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
