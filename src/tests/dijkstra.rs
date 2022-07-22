use crate::*;
use crate::graph::*;
use std::cell::RefCell;

pub fn dijkstra_graph<'a>() -> Graph<&'a str, RefCell<u64>, u64> {

	graph![
		(&str, RefCell<u64>) => [u64]
		("A", RefCell::new(u64::MAX)) => [ ("B", 4), ("H", 8) ]
		("B", RefCell::new(u64::MAX)) => [ ("A", 4), ("H", 11), ("C", 8) ]
		("C", RefCell::new(u64::MAX)) => [ ("B", 8), ("C", 2), ("F", 4), ("D", 7) ]
		("D", RefCell::new(u64::MAX)) => [ ("C", 7), ("F", 14), ("E", 9) ]
		("E", RefCell::new(u64::MAX)) => [ ("D", 9), ("F", 10) ]
		("F", RefCell::new(u64::MAX)) => [ ("G", 2), ("C", 4), ("D", 14), ("E", 10) ]
		("G", RefCell::new(u64::MAX)) => [ ("H", 1), ("I", 6), ("F", 2) ]
		("H", RefCell::new(u64::MAX)) => [ ("A", 8), ("B", 11), ("I", 7), ("G", 1) ]
		("I", RefCell::new(u64::MAX)) => [ ("H", 7), ("C", 2), ("G", 6) ]
	]
}

#[test]
fn dijkstra_pfs_min() {
	let g = dijkstra_graph();

	g["A"].replace(0);

	g["A"].search().pfs_min_map(&g["E"], &|edge| {
		let s_dist = *edge.source().borrow();
		let t_dist = *edge.target().borrow();
		let delta = *edge;

		if t_dist > s_dist + delta {
			edge.target().replace(s_dist + delta);
			true
		} else {
			false
		}
	});

	assert!(g["E"].take() == 21);
}

#[test]
fn dijkstra_pfs_max() {
	let g = dijkstra_graph();

	g["A"].replace(0);

	g["A"].search().pfs_max_map(&g["E"], &|edge| {
		let s_dist = *edge.source().borrow();
		let t_dist = *edge.target().borrow();
		let delta = *edge;

		if t_dist > s_dist + delta {
			edge.target().replace(s_dist + delta);
			true
		} else {
			false
		}
	});

	assert!(g["E"].take() == 33);
}

#[test]
fn dijkstra() {
	use min_max_heap::MinMaxHeap;

	let g = dijkstra_graph();
	let mut heap = MinMaxHeap::new();
	let mut visited = Graph::new();

	g["A"].replace(0);
	heap.push(g["A"].clone());

	'search: while let Some(s) = heap.pop_min() {
		for edge in &s {
			let (_, t, delta) = edge.decomp();
			let (s_dist, t_dist) = (*s.borrow(), *t.borrow());

			if visited.insert(t.clone()) {
				if t_dist > s_dist + delta {
					t.replace(s_dist + delta);
					if s == g["E"] { break 'search }
					heap.push(t.clone());
				}
			}
		}
	}

	assert!(g["E"].take() == 21);
}
