// From paper https://www.researchgate.net/publication/222719985_Delta-stepping_a_parallelizable_shortest_path_algorithm

use gdsl::async_digraph::*;
use gdsl::{
	async_digraph_node as node,
	async_digraph_connect as connect,
	async_digraph
};
// use std::cell::Mutex;
use std::sync::Mutex;
use std::io::Write;
use std::fs::File;
use rayon::prelude::*;
use rayon::iter::IntoParallelIterator;
use std::sync::Arc;
use std::sync::RwLock;

type N = Node<usize, Dist, u64>;
type E = Edge<usize, Dist, u64>;
type G = Graph<usize, Dist, u64>;

fn create_graph_dijkstra_error() -> Graph<usize, Dist, u64> {
	async_digraph![
		(usize, Dist) => [u64]
		(0, Dist::new(u64::MAX)) => [ (1, 4), (2, 1), (2, 5) ]
		(1, Dist::new(u64::MAX)) => [ (0, 1), (1, 4), (3, 1) ]
		(2, Dist::new(u64::MAX)) => [ (1, 1), (3, 4), (2, 1) ]
		(3, Dist::new(u64::MAX)) => [ (0, 4), (2, 2), (3, 3) ]
	]
}

fn attr(field: &str, value: &str) -> (String, String) {
	(field.to_string(), value.to_string())
}

const DELTA: usize = 3;

enum EdgeKind {
	Light,
	Heavy,
}

#[derive(Clone)]
pub struct Dist {
	value: Arc<Mutex<u64>>,
}

impl Dist {
	fn new(value: u64) -> Dist {
		Dist {
			value: Arc::new(Mutex::new(value)),
		}
	}

	fn get(&self) -> u64 {
		*self.value.lock().unwrap()
	}

	fn set(&self, value: u64) {
		*self.value.lock().unwrap() = value;
	}
}

impl std::cmp::PartialEq for Dist {
	fn eq(&self, other: &Dist) -> bool {
		self.get() == other.get()
	}
}

impl std::cmp::Eq for Dist {}

impl std::cmp::PartialOrd for Dist {
	fn partial_cmp(&self, other: &Dist) -> Option<std::cmp::Ordering> {
		self.get().partial_cmp(&other.get())
	}
}

impl std::cmp::Ord for Dist {
	fn cmp(&self, other: &Dist) -> std::cmp::Ordering {
		self.get().cmp(&other.get())
	}
}

unsafe impl Send for Dist {}

struct Buckets {
	buckets: RwLock<Vec<RwLock<Vec<N>>>>,
	delta: usize,
}

impl Buckets {
	pub fn new(delta: usize) -> Self {
		Self {
			buckets: RwLock::new(vec![vec![]]),
			delta,
		}
	}

	pub fn add(&self, n: &N, idx: usize) {
		if idx >= self.buckets.read().unwrap().len() {
			self.buckets.write().unwrap().resize(idx + 1, vec![]);
		}
		let mut buckets = self.buckets.write().unwrap();
		let bucket = buckets.get_mut(idx).unwrap();
		bucket.push(n.clone());
	}

	pub fn get(&self, idx: usize) -> Option<Vec<N>> {
		let r = self.buckets.read().unwrap();
		let r = r.get(idx);
		r.map(|v| *v)
	}

	pub fn get_mut(&mut self, idx: usize) -> Option<&mut Vec<N>> {
		self.buckets.read().unwrap().get_mut(idx)
	}

	pub fn remove(&mut self, n: &N, idx: usize) {
		match self.buckets.read().unwrap().get_mut(idx) {
			Some(bucket) => {
				bucket.retain(|x| x != n);
			},
			None => {
				return;
			}
		}
	}

	pub fn relax(&mut self, n: &N, new_dist: u64) {
		let cur_dist = n.get();
		if new_dist < cur_dist {
			let old_bucket = cur_dist as usize / self.delta;
			let new_bucket = new_dist as usize / self.delta;
			if cur_dist < u64::MAX {
				self.remove(&n, old_bucket);
			}
			self.add(&n, new_bucket);
			n.set(new_dist);
		}
	}

	pub fn relax_edges(&mut self, mut edges: Vec<E>) {
		edges.par_iter_mut().for_each(|(u, v, e)| {
			self.relax(v, u.get() + *e);
		});
	}

	pub fn find_edges(&self, bucket: &Vec<N>, kind: EdgeKind) -> Vec<E> {
		let mut edges = Vec::new();
		for node in bucket.iter() {
			for (u, v, e) in node {
				match kind {
					EdgeKind::Light => {
						if e <= self.delta as u64 {
							edges.push((u, v, e));
						}
					}
					EdgeKind::Heavy => {
						if e > self.delta as u64 {
							edges.push((u, v, e));
						}
					}
				}
			}
		}
		edges
	}

	pub fn pop(&self) -> Option<usize> {
		for (i, bucket) in self.buckets.read().unwrap().iter().enumerate() {
			if !bucket.is_empty() {
				return Some(i);
			}
		}
		None
	}
}

pub fn seq_dstep_sd(s: &N) {
	let mut buckets = Buckets::new(DELTA);
	s.set(0);
	buckets.add(s, 0);
	while let Some(idx) = buckets.pop() {
		let mut heavy_nodes = vec![];
		loop {
			let bucket = buckets.get(idx).unwrap();
			let edges = buckets.find_edges(&bucket, EdgeKind::Light);
			for node in bucket.iter() {
				heavy_nodes.push(node.clone());
			}
			let bucket = buckets.get_mut(idx).unwrap();
			bucket.clear();
			buckets.relax_edges(edges);
			let bucket = buckets.get(idx).unwrap();
			if bucket.is_empty() {
				break;
			}
		}
		let edges = buckets.find_edges(&heavy_nodes, EdgeKind::Heavy);
		buckets.relax_edges(edges);
	}
}

fn create_graph(size: usize, avg_dgr: usize) -> G {
	let mut g = G::new();

    for i in 0..size {
        g.insert(node!(i, Dist::new(u64::MAX)));
    }

	for (_, node) in g.iter() {
		for _ in 0..avg_dgr {
			connect!(&node => &g[rand::random::<usize>() % size], rand::random::<u64>() % 100 + 1);
		}
	}
	g
}

#[test]
fn compare_dijkstra() {
	let g = create_graph(1000, 3);
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

	seq_dstep_sd(&s);

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

	// let v = vec![1];

	// v.into_par_iter().map(|v|{v}).collect::<i32>();
	// v.par_iter().sum::<i32>();
}

// fn create_graph_vec_distance_2(size: usize, avg_dgr: usize) -> Vec<Node<usize, Mutex<u64>, u64>> {
// 	let mut g = Vec::new();

//     for i in 0..size {
//         g.push(node!(i, Mutex::new(u64::MAX)));
//     }

// 	for node in g.iter() {
// 		for _ in 0..avg_dgr {
// 			connect!(&node => &g[rand::random::<usize>() % size], rand::random::<u64>() % 100);
// 		}
// 	}
// 	g
// }

// fn create_graph_dijkstra() -> Graph<usize, Mutex<u64>, u64> {
// 	digraph![
// 		(usize, Mutex<u64>) => [u64]
// 		(0, Mutex::new(u64::MAX)) => [ (1, 4), (7, 8) ]
// 		(1, Mutex::new(u64::MAX)) => [ (0, 4), (7, 11), (2, 8) ]
// 		(2, Mutex::new(u64::MAX)) => [ (1, 8), (2, 2), (5, 4), (3, 7) ]
// 		(3, Mutex::new(u64::MAX)) => [ (2, 7), (5, 14), (4, 9) ]
// 		(4, Mutex::new(u64::MAX)) => [ (3, 9), (5, 10) ]
// 		(5, Mutex::new(u64::MAX)) => [ (6, 2), (2, 4), (3, 14), (4, 10) ]
// 		(6, Mutex::new(u64::MAX)) => [ (7, 1), (8, 6), (5, 2) ]
// 		(7, Mutex::new(u64::MAX)) => [ (0, 8), (1, 11), (8, 7), (6, 1) ]
// 		(8, Mutex::new(u64::MAX)) => [ (7, 7), (2, 2), (6, 6) ]
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