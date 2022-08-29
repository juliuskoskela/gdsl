// From paper https://www.researchgate.net/publication/222719985_Delta-stepping_a_parallelizable_shortest_path_algorithm
use gdsl::digraph::*;
use gdsl::{
	digraph_node as node,
	digraph_connect as connect,
	digraph
};
use rayon::prelude::IntoParallelRefIterator;
use std::cell::Cell;
use std::io::Write;
use std::fs::File;

type N = Node<usize, Cell<u64>, u64>;
type E = Edge<usize, Cell<u64>, u64>;
type G = Graph<usize, Cell<u64>, u64>;

fn attr(field: &str, value: &str) -> (String, String) {
	(field.to_string(), value.to_string())
}

const DELTA: usize = 100;

enum EdgeKind {
	Light,
	Heavy,
}

struct Buckets {
	buckets: Vec<Vec<Option<N>>>,
	delta: usize,
}

impl Buckets {
	pub fn new(delta: usize) -> Self {
		Self {
			buckets: vec![vec![]],
			delta,
		}
	}

	pub fn add(&mut self, n: &N, idx: usize) {
		if idx >= self.buckets.len() {
			self.buckets.resize(idx + 1, vec![]);
		}
		let bucket = self.buckets.get_mut(idx).unwrap();
		bucket.push(Some(n.clone()));
	}

	pub fn get(&self, idx: usize) -> Option<&Vec<Option<N>>> {
		self.buckets.get(idx)
	}

	pub fn get_mut(&mut self, idx: usize) -> Option<&mut Vec<Option<N>>> {
		self.buckets.get_mut(idx)
	}

	pub fn remove(&mut self, n: &N, idx: usize) {
		match self.buckets.get_mut(idx) {
			Some(bucket) => {
				let cur = bucket.iter().position(|x| x.as_ref().map(|x| x) == Some(n));
				match cur {
					Some(i) => {
						bucket[i] = None;
					}
					None => {
						return;
					}
				}
			},
			None => {
				return;
			}
		}
	}

	pub fn relax(&mut self, v: N, new_dist: u64) {
		let cur_dist = v.get();
		if new_dist < cur_dist {
			let old_bucket = cur_dist as usize / self.delta;
			let new_bucket = new_dist as usize / self.delta;
			if cur_dist < u64::MAX {
				self.remove(&v, old_bucket);
			}
			self.add(&v, new_bucket);
			v.set(new_dist);
		}
	}

	pub fn relax_light_edges(&mut self, edges: &Vec<E>) {
		for (u, v, e) in edges {
			let new_dist = u.get() + e;
			let cur_dist = v.get();
			if new_dist < cur_dist {
				self.add(&v, new_dist as usize / self.delta);
				v.set(new_dist);
			}
		}
	}

	pub fn relax_heavy_edges(&mut self, edges: &Vec<E>) {
		for (u, v, e) in edges {
			let new_dist = u.get() + e;
			let cur_dist = v.get();
			if new_dist < cur_dist {
				let old_bucket = cur_dist as usize / self.delta;
				let new_bucket = new_dist as usize / self.delta;
				if cur_dist < u64::MAX {
					self.remove(&v, old_bucket);
				}
				self.add(&v, new_bucket);
				v.set(new_dist);
			}
		}
	}

	pub fn find_edges(&self, bucket_idx: usize) -> (Vec<E>, Vec<E>) {
		let bucket = self.buckets.get(bucket_idx).unwrap();
		let mut light_edges = Vec::new();
		let mut heavy_edges = Vec::new();
		for node in bucket.iter() {
			let node = node.as_ref();
			if node.is_none() {
				continue;
			}
			let node = node.unwrap();
			let node_dist = node.get();
			let cur_delta = node_dist % self.delta as u64;
			for (u, v, e) in node {
				match e <= cur_delta as u64 {
					true => {
						light_edges.push((u, v, e));
					}
					false => {
						heavy_edges.push((u, v, e));
					}
				}
			}
		}
		(light_edges, heavy_edges)
	}

	pub fn pop(&self) -> Option<usize> {
		for (i, bucket) in self.buckets.iter().enumerate() {
			if !bucket.is_empty() {
				return Some(i);
			}
		}
		None
	}
}

pub fn seq_dstep_sd(s: &N, delta: usize) {
	let mut buckets = Buckets::new(delta);
	buckets.relax(s.clone(), 0);
	while let Some(idx) = buckets.pop() {
		let mut all_heavy_edges = Vec::new();
		loop {
			let (light_edges, mut heavy_edges) = buckets.find_edges(idx);
			all_heavy_edges.append(&mut heavy_edges);
			let bucket = buckets.get_mut(idx).unwrap();
			bucket.clear();
			buckets.relax_light_edges(&light_edges);
			let bucket = buckets.get(idx).unwrap();
			if bucket.is_empty() {
				break;
			}
		}
		buckets.relax_heavy_edges(&all_heavy_edges);
	}
}

fn create_graph(size: usize, avg_dgr: usize) -> G {
	let mut g = G::new();

    for i in 0..size {
        g.insert(node!(i, Cell::new(u64::MAX)));
    }

	for (_, node) in g.iter() {
		for _ in 0..avg_dgr {
			let target = &g[rand::random::<usize>() % size];
			if node == target {
				continue;
			}
			match node.try_connect(target, rand::random::<u64>() % 3 + 1) {
				Ok(_) => {
					continue;
				}
				Err(_) => {
					continue;
				}
			}
		}
	}
	g
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::HashSet;

fn relax_node(
	node: &N,
	heap: &mut BinaryHeap<Reverse<N>>,
	visited: &mut HashSet<usize>
) {
	for (u, v, e) in node {
		let new_dist = u.get() + e;
		let cur_dist = v.get();
		if new_dist < cur_dist {
			v.set(new_dist);
			if visited.insert(*v.key()) {
				heap.push(Reverse(v.clone()));
			}
		}
	}
}

fn dijkstra(s: &N) {
	let mut heap = BinaryHeap::new();
	let mut visited = HashSet::new();

	s.set(0);
	heap.push(Reverse(s.clone()));
	relax_node(s, &mut heap, &mut visited);

	while let Some(u) = heap.pop() {
		let u = u.0;
		relax_node(&u, &mut heap, &mut visited);
	}
}

#[test]
fn make_graph_with_error() {

	for i in 0..1000 {
		let g = create_graph(1000, 3);
		let a = g.to_vec();
		let s = g[0].clone();

		dijkstra(&s);

		let mut a_dists = a.iter().map(|n| (*n.key(), n.get())).collect::<Vec<_>>();
		a_dists.sort_by(|(a, _), (b, _)| a.cmp(b));

		for node in &a {
			node.set(u64::MAX);
		}

		seq_dstep_sd(&s, DELTA);

		let mut b_dists = a.iter().map(|n| (*n.key(), n.get())).collect::<Vec<_>>();
		b_dists.sort_by(|(a, _), (b, _)| a.cmp(b));
		let mut error = false;
		for ((_, a), (_, b)) in a_dists.iter().zip(b_dists.iter()) {
			if *a != *b {
				error = true;
			}
		}
		if error == false {
			continue;
		}
		println!("Found error in {}", i);
		for ((akey, a), (bkey, b)) in a_dists.iter().zip(b_dists.iter()) {
			let astr = if *a == u64::MAX { "INF".to_string() } else { a.to_string() };
			let bstr = if *b == u64::MAX { "INF".to_string() } else { b.to_string() };
			if a == b {
				println!("{:6}: {:6} | {:6}: {:6}", akey, astr, bkey, bstr);
			} else {
				println!("{:6}: {:6} | {:6}: {:6} (ERROR)", akey, astr, bkey, bstr);
			}
		}
		let dot = g.to_dot_with_attr(
			&|_|{None},
			&|node|{
				Some(vec![
					attr("label", &format!("{}: {}", node.key(), node.get())),
					attr("weight", &format!("{}", node.get())),
				])
			},
			&|_, _, e|{
				Some(vec![
					attr("label", &format!("{}", e)),
					attr("weight", &format!("{}", e)),
				])
			}
		);
		let mut new_file = File::create("dijkstra.dot").unwrap();
		new_file.write_all(dot.as_bytes()).unwrap();
		break;
	}
}

// 0 -> 2 : 1
// 0 -> 2 : 3
// 0 -> 3 : 3
// 1 -> 0 : 1
// 1 -> 3 : 2
// 1 -> 4 : 3
// 2 -> 1 : 1
// 2 -> 2 : 1
// 2 -> 3 : 1
// 3 -> 1 : 2
// 3 -> 2 : 3
// 3 -> 4 : 2
// 4 -> 1 : 1
// 4 -> 4 : 2
// 4 -> 4 : 1

fn create_graph_dijkstra_error() -> Graph<usize, Cell<u64>, u64> {
	digraph![
		(usize, Cell<u64>) => [u64]
		(0, Cell::new(u64::MAX)) => [ (2, 1), (2, 3), (3, 3) ]
		(1, Cell::new(u64::MAX)) => [ (0, 1), (3, 2), (4, 3) ]
		(2, Cell::new(u64::MAX)) => [ (1, 1), (2, 1), (3, 1) ]
		(3, Cell::new(u64::MAX)) => [ (1, 2), (2, 3), (4, 2) ]
		(4, Cell::new(u64::MAX)) => [ (1, 1), (4, 2), (4, 1) ]
	]
}

#[test]
fn compare_dijkstra() {
	let g = create_graph_dijkstra_error();
	// if g.is_none() {
	// 	println!("No graph with error found");
	// 	return ;
	// }
	// let g = g.unwrap();
	let a = g.to_vec();
	let s = g[0].clone();

	s.set(0);
	s.pfs().map(&|u, v, e| {
		let (u_dist, v_dist) = (u.get(), v.get());
		if v_dist > u_dist + e { v.set(u_dist + e); }
	}).search();

	let mut a_dists = a.iter().map(|n| (*n.key(), n.get())).collect::<Vec<_>>();
	a_dists.sort_by(|(a, _), (b, _)| a.cmp(b));

	for node in &a {
		node.set(u64::MAX);
	}

	seq_dstep_sd(&s, DELTA);

	let mut b_dists = a.iter().map(|n| (*n.key(), n.get())).collect::<Vec<_>>();
	b_dists.sort_by(|(a, _), (b, _)| a.cmp(b));

	println!("a	|	b");
	println!("--------------------");
	for ((akey, a), (bkey, b)) in a_dists.iter().zip(b_dists.iter()) {
		let astr = if *a == u64::MAX { "INF".to_string() } else { a.to_string() };
		let bstr = if *b == u64::MAX { "INF".to_string() } else { b.to_string() };
		if a == b {
			println!("{}: {}	|	{}: {}", akey, astr, bkey, bstr);
		} else {
			println!("{}: {}	|	{}: {}	(ERROR)", akey, astr, bkey, bstr);
		}
	}

	let dot = g.to_dot_with_attr(
		&|_|{None},
		&|node|{
			Some(vec![
				attr("label", &format!("{}: {}", node.key(), node.get())),
				attr("weight", &format!("{}", node.get())),
			])
		},
		&|_, _, e|{
			Some(vec![
				attr("label", &format!("{}", e)),
				attr("weight", &format!("{}", e)),
			])
		}
	);

	let mut new_file = File::create("dijkstra.dot").unwrap();
	new_file.write_all(dot.as_bytes()).unwrap();
}

// fn create_graph_vec_distance_2(size: usize, avg_dgr: usize) -> Vec<Node<usize, Cell<u64>, u64>> {
// 	let mut g = Vec::new();

//     for i in 0..size {
//         g.push(node!(i, Cell::new(u64::MAX)));
//     }

// 	for node in g.iter() {
// 		for _ in 0..avg_dgr {
// 			connect!(&node => &g[rand::random::<usize>() % size], rand::random::<u64>() % 100);
// 		}
// 	}
// 	g
// }

// fn create_graph_dijkstra() -> Graph<usize, Cell<u64>, u64> {
// 	digraph![
// 		(usize, Cell<u64>) => [u64]
// 		(0, Cell::new(u64::MAX)) => [ (1, 4), (7, 8) ]
// 		(1, Cell::new(u64::MAX)) => [ (0, 4), (7, 11), (2, 8) ]
// 		(2, Cell::new(u64::MAX)) => [ (1, 8), (2, 2), (5, 4), (3, 7) ]
// 		(3, Cell::new(u64::MAX)) => [ (2, 7), (5, 14), (4, 9) ]
// 		(4, Cell::new(u64::MAX)) => [ (3, 9), (5, 10) ]
// 		(5, Cell::new(u64::MAX)) => [ (6, 2), (2, 4), (3, 14), (4, 10) ]
// 		(6, Cell::new(u64::MAX)) => [ (7, 1), (8, 6), (5, 2) ]
// 		(7, Cell::new(u64::MAX)) => [ (0, 8), (1, 11), (8, 7), (6, 1) ]
// 		(8, Cell::new(u64::MAX)) => [ (7, 7), (2, 2), (6, 6) ]
// 	]
// }

// fn make_graph_with_errot_try_100() -> Option<G> {

// 	for _ in 0..1000_000 {
// 		let g = create_graph(4, 3);
// 		let a = g.to_vec();
// 		let s = g[0].clone();

// 		s.set(0);
// 		s.pfs().map(&|u, v, e| {
// 			let (u_dist, v_dist) = (u.get(), v.get());
// 			if v_dist > u_dist + e { v.set(u_dist + e); }
// 		}).search();

// 		let mut a_dists = a.iter().map(|n| (*n.key(), n.get())).collect::<Vec<_>>();
// 		a_dists.sort_by(|(a, _), (b, _)| a.cmp(b));

// 		for node in &a {
// 			node.set(u64::MAX);
// 		}

// 		// seq_delta_stepping(&s);

// 		let mut b_dists = a.iter().map(|n| (*n.key(), n.get())).collect::<Vec<_>>();
// 		b_dists.sort_by(|(a, _), (b, _)| a.cmp(b));

// 		for ((_, a), (_, b)) in a_dists.iter().zip(b_dists.iter()) {
// 			if *a != *b {
// 				return Some(g);
// 			}
// 		}
// 	}
// 	None
// }

// fn relax_requests(requests: Vec<E>, buckets: &mut Vec<HashMap<usize, N>>) {
// 	for (u, v, e) in requests {
// 		relax(v, u.get() + e, buckets);
// 	}
// }

// fn find_requests(bucket: &HashMap<usize, N>, kind: EdgeKind) -> Vec<E> {
// 	let mut requests = Vec::new();

// 	for (_, node) in bucket.iter() {
// 		for (u, v, e) in node {
// 			match kind {
// 				EdgeKind::Light => {
// 					if e <= DELTA as u64 {
// 						requests.push((u, v, e));
// 					}
// 				}
// 				EdgeKind::Heavy => {
// 					if e > DELTA as u64 {
// 						requests.push((u, v, e));
// 					}
// 				}
// 			}
// 		}
// 	}
// 	requests
// }

// fn relax(n: N, new_dist: u64, buckets: &mut Vec<HashMap<usize, N>>) {
//     let cur_dist = n.get();
//     if new_dist < cur_dist {
//         let old_bucket = cur_dist / DELTA as u64;
//         let new_bucket = new_dist / DELTA as u64;
//         if old_bucket != new_bucket {
// 			if cur_dist < u64::MAX {
//             	buckets[old_bucket as usize].remove(n.key());
// 			}
//             buckets[new_bucket as usize].insert(*n.key(), n.clone());
//         }
//         n.set(new_dist);
//     }
// }

// fn find_min_bucket(buckets: &Vec<HashMap<usize, N>>) -> Option<usize> {
// 	let mut min_bucket = None;
// 	for (i, bucket) in buckets.iter().enumerate() {
// 		if bucket.len() > 0 {
// 			min_bucket = Some(i);
// 			break;
// 		}
// 	}
// 	min_bucket
// }

// pub fn seq_delta_stepping(s: &N) {
// 	let mut buckets = vec![HashMap::default(); 100000];

// 	s.set(0);
// 	buckets[0].insert(*s.key(), s.clone());

// 	while let Some(i) = find_min_bucket(&buckets) {
// 		let mut deleted_nodes = HashMap::default();
// 		while !buckets[i].is_empty() {
// 			let requests = find_requests(&buckets[i], EdgeKind::Light);
// 			for node in buckets[i].values() {
// 				deleted_nodes.insert(*node.key(), node.clone());
// 			}
// 			buckets[i].clear();
// 			relax_requests(requests, &mut buckets);
// 		}
// 		let requests = find_requests(&deleted_nodes, EdgeKind::Heavy);
// 		relax_requests(requests, &mut buckets);
// 	}
// }