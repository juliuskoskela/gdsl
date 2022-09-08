use gdsl::ungraph::*;
use gdsl::*;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
use std::cell::RefCell;

type N = Node<usize, (), u64>;
type E = Edge<usize, (), u64>;
type Heap = BinaryHeap<Reverse<ForestEdge>>;
type Forest = Vec<E>;

#[derive(Clone)]
struct ForestEdge(E);

impl PartialEq for ForestEdge {
    fn eq(&self, other: &ForestEdge) -> bool {
        self.0.2 == other.0.2
    }
}

impl Eq for ForestEdge {}

impl PartialOrd for ForestEdge {
    fn partial_cmp(&self, other: &ForestEdge) -> Option<std::cmp::Ordering> {
        Some(self.0.2.cmp(&other.0.2))
    }
}

impl Ord for ForestEdge {
    fn cmp(&self, other: &ForestEdge) -> std::cmp::Ordering {
        self.0.2.cmp(&other.0.2)
    }
}

fn prim_minimum_spanning_tree(s: &N) -> Forest {
    let mut forest: Forest = vec![];
    let mut added_nodes: HashSet<usize> = HashSet::new();
	let heap = RefCell::new(Heap::new());

	added_nodes.insert(*s.key());

	s.bfs().map(&|u, v, e| {
		heap.borrow_mut().push(Reverse(ForestEdge((u.clone(), v.clone(), *e))));
	}).search();

	let mut tmp: Vec<ForestEdge> = vec![];
	loop {
		if let Some(edge) = heap.borrow_mut().pop() {
			let Reverse(ForestEdge((u, v, e))) = edge;
			if added_nodes.contains(u.key()) && !added_nodes.contains(v.key()) {
				forest.push((u.clone(), v.clone(), e));
				added_nodes.insert(*v.key());
			} else {
				tmp.push(ForestEdge((u.clone(), v.clone(), e)));
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
