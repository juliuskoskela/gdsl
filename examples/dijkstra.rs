use gdsl::*;
use std::cell::Cell;

fn dijkstra_1() {

	let g = graph![
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
	];

	g["A"].set(0);

	g["A"].pfs_min().search_filter_map(Some(&g["E"]), &|u, v, e| {

		let (u_dist, v_dist) = (u.get(), v.get());

		match v_dist > u_dist + e {
			true => {
				v.set(u_dist + e);
				true
			},
			false => false,
		}

	});

	assert!(g["E"].take() == 21);
}

fn dijkstra_2() {
	use gdsl::*;
	use min_max_heap::MinMaxHeap;

	let g = graph![
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
	];

	let mut heap = MinMaxHeap::new();
	let mut visited = std::collections::HashSet::new();

	g["A"].set(0);
	heap.push(g["A"].clone());

	'search: while let Some(u) = heap.pop_min() {
		for (v, delta) in &u {
			let (u_dist, v_dist) = (u.get(), v.get());

			if !visited.contains(v.key()) {
				if v_dist > u_dist + delta {
					visited.insert(v.key().clone());
					v.set(u_dist + delta);
					if v == g["E"] { break 'search }
					heap.push(v.clone());
				}
			}
		}
	}

	assert!(g["E"].take() == 21);
}

fn main() {
	dijkstra_1();
	dijkstra_2();
}