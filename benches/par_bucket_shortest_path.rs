// From paper https://www.researchgate.net/publication/222719985_Delta-stepping_a_parallelizable_shortest_path_algorithm
use gdsl::async_digraph::*;
use gdsl::{
	async_digraph_node as node,
	async_digraph_connect as connect,
};
use rayon::prelude::IntoParallelRefIterator;
use std::io::Write;
use std::fs::File;
use std::cell::Cell;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex, RwLock};

type N = Node<usize, Dist, u64>;
type E = Edge<usize, Dist, u64>;
type G = Graph<usize, Dist, u64>;

fn attr(field: &str, value: &str) -> (String, String) {
	(field.to_string(), value.to_string())
}

const DELTA: usize = 100;

#[derive(Clone)]
pub struct Dist {
	dist: Arc<Mutex<u64>>,
}

impl Dist {
	pub fn new(dist: u64) -> Self {
		Self {
			dist: Arc::new(Mutex::new(dist)),
		}
	}

	pub fn get(&self) -> u64 {
		*self.dist.lock().unwrap()
	}

	pub fn set(&self, dist: u64) {
		*self.dist.lock().unwrap() = dist;
	}
}

impl std::cmp::PartialEq for Dist {
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl std::cmp::Eq for Dist {}

impl std::cmp::PartialOrd for Dist {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.get().cmp(&other.get()))
	}
}

impl std::cmp::Ord for Dist {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.get().cmp(&other.get())
	}
}

type Bucket = Arc<RwLock<Vec<Option<N>>>>;

struct Buckets {
	buckets: RwLock<Vec<Bucket>>,
	delta: usize,
}

impl Buckets {
	pub fn new(delta: usize) -> Self {
		Self {
			buckets: RwLock::new(vec![Arc::new(RwLock::new(vec![])); delta]),
			delta,
		}
	}

	pub fn add(&self, n: &N, idx: usize) {
		match self.buckets.write() {
			Ok(mut buckets) => {
				while idx >= buckets.len() {
					buckets.push(Arc::new(RwLock::new(vec![])));
				}
				buckets[idx].write().unwrap().push(Some(n.clone()));
			}
			Err(e) => {
				println!("{:?}", e);
			}
		}
	}

	pub fn get(&self, idx: usize) -> Option<Bucket> {
		let bucket = self.buckets.read().unwrap();
		let bucket = bucket.get(idx);
		match bucket {
			Some(bucket) => Some(bucket.clone()),
			None => None,
		}
	}

	pub fn remove(&self, n: &N, idx: usize) {
		match self.buckets.read() {
			Ok(buckets) => {
				match buckets.get(idx) {
					Some(bucket) => {
						match bucket.write() {
							Ok(mut bucket) => {
								bucket.iter_mut().find(|x| x.as_ref().map(|x| x == n).unwrap_or(false)).map(|x| {
									*x = None;
								});
							}
							Err(e) => {
								panic!("{:?}", e);
							}
						}
					}
					None => {
						return;
					}
				}
			}
			Err(e) => {
				panic!("{:?}", e);
			}
		}
	}

	pub fn relax(&self, n: &N, new_dist: u64) {
		match n.dist.lock() {
			Ok(mut dist) => {
				if new_dist < *dist {
					let old_bucket = *dist as usize / self.delta;
					let new_bucket = new_dist as usize / self.delta;
					if *dist < u64::MAX {
						self.remove(&n, old_bucket);
					}
					*dist = new_dist;
					self.add(&n, new_bucket);
				}
			}
			Err(e) => {
				panic!("{:?}", e);
			}
		}
	}

	pub fn relax_edges(&mut self, edges: Vec<E>) {
		edges.par_iter().for_each(|(u, v, e)| {
			self.relax(v, u.get() + e);
		});
	}

	pub fn find_edges_light(&self, bucket: &Bucket) -> Vec<E> {
		let mut edges = Vec::new();
		let bucket = bucket.read().unwrap();
		for node in bucket.iter() {
			let node = node.as_ref();
			if node.is_none() {
				continue;
			}
			let node = node.unwrap();
			for (u, v, e) in node {
				if e <= self.delta as u64 {
					edges.push((u, v, e));
				}
			}
		}
		edges
	}

	pub fn find_edges_heavy(&self, bucket: &Vec<Option<N>>) -> Vec<E> {
		let mut edges = Vec::new();
		for node in bucket.iter() {
			let node = node.as_ref();
			if node.is_none() {
				continue;
			}
			let node = node.unwrap();
			for (u, v, e) in node {
				if e > self.delta as u64 {
					edges.push((u, v, e));
				}
			}
		}
		edges
	}

	pub fn pop(&self) -> Option<usize> {
		let buckets = self.buckets.read().unwrap();
		for (i, bucket) in buckets.iter().enumerate() {
			let bucket = bucket.read().unwrap();
			if !bucket.is_empty() {
				return Some(i);
			}
		}
		None
	}
}

fn relax_edges(buckets: &Buckets, edges: Vec<E>) {
	edges.par_iter().for_each(|(u, v, e)| {
		buckets.relax(&v, u.get() + e);
	});
}

pub fn par_seq_dstep_sd(s: &N, delta: usize) {
	let mut buckets = Buckets::new(delta);
	buckets.relax(s, 0);
	while let Some(idx) = buckets.pop() {
		let mut heavy_nodes = vec![];
		loop {
			let bucket = buckets.get(idx).unwrap();
			let edges = buckets.find_edges_light(&bucket);
			for node in bucket.read().unwrap().iter() {
				heavy_nodes.push(node.clone());
			}
			let bucket = buckets.get(idx).unwrap();
			bucket.write().unwrap().clear();
			buckets.relax_edges(edges);
			let bucket = buckets.get(idx).unwrap();
			if bucket.read().unwrap().is_empty() {
				break;
			}
		}
		let edges = buckets.find_edges_heavy(&heavy_nodes);
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

	par_seq_dstep_sd(&s, DELTA);

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

#[test]
fn par_iter_edges() {
	let v: Vec<Edge<usize, usize, usize>> = Vec::new();

	let iterator = v.into_par_iter();
}

// fn create_graph_dijkstra_error() -> Graph<usize, Cell<u64>, u64> {
// 	digraph![
// 		(usize, Cell<u64>) => [u64]
// 		(0, Cell::new(u64::MAX)) => [ (1, 4), (2, 1), (2, 5) ]
// 		(1, Cell::new(u64::MAX)) => [ (0, 1), (1, 4), (3, 1) ]
// 		(2, Cell::new(u64::MAX)) => [ (1, 1), (3, 4), (2, 1) ]
// 		(3, Cell::new(u64::MAX)) => [ (0, 4), (2, 2), (3, 3) ]
// 	]
// }

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