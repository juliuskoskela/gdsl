use crate::*;
use crate::graph::*;
use std::cell::Cell;

pub fn dijkstra_graph<'a>() -> Graph<&'a str, Cell<u64>, u64> {

	graph![
		(&str, Cell<u64>) => [u64]
		("A", Cell::new(u64::MAX)) => [ ("B", 4), ("H", 8) ]
		("B", Cell::new(u64::MAX)) => [ ("A", 4), ("H", 11), ("C", 8) ]
		("C", Cell::new(u64::MAX)) => [ ("B", 8), ("C", 2), ("F", 4), ("D", 7) ]
		("D", Cell::new(u64::MAX)) => [ ("C", 7), ("F", 14), ("E", 9) ]
		("E", Cell::new(u64::MAX)) => [ ("D", 9), ("F", 10) ]
		("F", Cell::new(u64::MAX)) => [ ("G", 2), ("C", 4), ("D", 14), ("E", 10) ]
		("G", Cell::new(u64::MAX)) => [ ("H", 1), ("I", 6), ("F", 2) ]
		("H", Cell::new(u64::MAX)) => [ ("A", 8), ("B", 11), ("I", 7), ("G", 1) ]
		("I", Cell::new(u64::MAX)) => [ ("H", 7), ("C", 2), ("G", 6) ]
	]
}

#[test]
fn dijkstra_pfs_min() {
	let g = dijkstra_graph();

	g["A"].set(0);

	g["A"].search().pfs_min_map(&g["E"], &|s, t, delta| {
		let (s_dist, t_dist) = (s.get(), t.get());

		if t_dist > s_dist + delta {
			t.set(s_dist + delta);
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

	g["A"].set(0);

	g["A"].search().pfs_max_map(&g["E"], &|s, t, delta| {
		let (s_dist, t_dist) = (s.get(), t.get());

		if t_dist > s_dist + delta {
			t.set(s_dist + delta);
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

	g["A"].set(0);
	heap.push(g["A"].clone());

	'search: while let Some(s) = heap.pop_min() {
		for edge in &s {
			let (_, t, delta) = edge.decomp();
			let (s_dist, t_dist) = (s.get(), t.get());

			if visited.insert(t.clone()) {
				if t_dist > s_dist + delta {
					t.set(s_dist + delta);
					if s == g["E"] { break 'search }
					heap.push(t.clone());
				}
			}
		}
	}

	assert!(g["E"].take() == 21);
}
