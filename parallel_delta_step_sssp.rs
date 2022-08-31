#![allow(unused)]
use core::num;
use std::{
	sync::{
		Arc,
		Mutex,
		RwLock,
	},
	collections::{
		BTreeMap,
		BinaryHeap
	},
	cmp::Reverse,
};
use std::cell::Cell;
use std::io::Write;
use std::fs::File;
use rayon::prelude::IntoParallelRefIterator;
use rayon::ThreadPoolBuilder;
use rayon::ThreadPool;
use gdsl::async_digraph::*;

use gdsl::{
	async_digraph_connect as connect,
	async_digraph_node as node,
};

type N = Node<usize, Dist, u64>;
type E = Edge<usize, Dist, u64>;
type G = Graph<usize, Dist, u64>;

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

struct Requests {
	requests: Vec<Arc<RwLock<Vec<E>>>>,
	num_threads: usize,
}

impl Requests {
	fn new(num_threads: usize) -> Self {
		Self {
			requests: vec![Arc::new(RwLock::new(Vec::new())); num_threads],
			num_threads,
		}
	}

	fn generate_requests(&self, dbkt: &DeltaBucket, num_threads: usize, delta: u64) -> Requests {
		let heavy_requests = Requests::new(num_threads);
		for node in dbkt.bucket.read().unwrap().iter() {
			for (u, v, e) in node {
				match e <= delta {
					true => {
						heavy_requests.requests[v.key() % num_threads]
							.write()
							.unwrap()
							.push((u, v, e));
					}
					false => {
						self.requests[v.key() % num_threads]
							.write()
							.unwrap()
							.push((u, v, e));
					}
				}
			}
		}
		heavy_requests
	}
}

#[derive(Clone)]
struct DeltaBucket {
	bucket: Arc<RwLock<Vec<N>>>,
}

impl DeltaBucket {
	pub fn new() -> Self {
		Self {
			bucket: Arc::new(RwLock::new(Vec::new())),
		}
	}

	pub fn insert(&self, node: N) {
		self.bucket.write().unwrap().push(node);
	}

	pub fn remove(&self, node: N) {
		let mut bucket = self.bucket.write().unwrap();
		bucket.retain(|n| n != &node);
	}

	pub fn clear(&self) {
		self.bucket.write().unwrap().clear();
	}

	pub fn is_empty(&self) -> bool {
		self.bucket.read().unwrap().is_empty()
	}
}

struct DeltaHeap {
	heap: RwLock<Vec<Option<DeltaBucket>>>,
	tpool: ThreadPool,
	delta: u64,
}

impl DeltaHeap {
	pub fn new(delta: u64, num_cpus: usize) -> Arc<Self> {
		Arc::new(Self {
			heap: RwLock::new(Vec::new()),
			tpool: ThreadPoolBuilder::new()
				.num_threads(num_cpus)
				.build().unwrap(),
			delta,
		})
	}

	pub fn len(&self) -> usize {
		self.heap.read().unwrap().len()
	}

	pub fn resize(&self, new_size: usize) {
		match self.heap.write() {
			Ok(mut heap) => {
				heap.resize(new_size, None);
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn insert(&self, node: &N, idx: usize) {
		if idx >= self.len() {
			self.resize((idx + 1) * 2);
		}
		match self.heap.write() {
			Ok(mut heap) => {
				match heap[idx] {
					Some(ref mut bucket) => {
						bucket.insert(node.clone());
					}
					None => {
						let bucket = DeltaBucket::new();
						bucket.insert(node.clone());
						heap[idx] = Some(bucket);
					}
				}
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn remove(&self, node: &N, idx: usize) {
		match self.heap.write() {
			Ok(mut heap) => {
				match heap[idx] {
					Some(ref mut bucket) => {
						bucket.remove(node.clone());
					}
					None => {
						return;
					}
				}
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn pop_min(&self) -> Option<DeltaBucket> {
		let mut target_bucket = None;
		let mut del_idx = 0;
		match self.heap.read() {
			Ok(heap) => {
				for (i, bucket) in heap.iter().enumerate() {
					if bucket.is_some() {
						target_bucket = bucket.clone();
						del_idx = i;
					}
				}
			}
			Err(e) => { panic!("{:?}", e); }
		}
		match target_bucket {
			Some(bucket) => {
				match self.heap.write() {
					Ok(mut heap) => {
						heap[del_idx] = None;
					}
					Err(e) => { panic!("{:?}", e); }
				}
				return Some(bucket);
			}
			None => {
				return None;
			}
		}
	}

	fn relax(&self, v: &N, new_dist: u64) {
		let cur_dist = v.get();
		if new_dist < cur_dist {
			let old_idx = cur_dist / self.delta;
			let new_idx = new_dist / self.delta;
			if cur_dist < u64::MAX && old_idx != new_idx {
				self.remove(v, old_idx as usize);
			}
			v.set(new_dist);
			self.insert(v, new_idx as usize);
		}
	}

	fn relax_requests(&self, requests: &Requests) {
		for request in requests.requests.iter() {
			self.tpool.install(|| {
				for (u, v, e) in request.read().unwrap().iter() {
					self.relax(v, u.get() + e);
				}
			});
		}
	}
}

fn parallel_delta_step_sssp(source: &N, delta: u64, num_threads: usize) {
	let dh = DeltaHeap::new(delta, num_threads);

	dh.relax(source, 0);

	while let Some(bucket) = dh.pop_min() {
		let mut heavy = Vec::new();
		while !bucket.is_empty() {
			let mut requests = Requests::new(num_threads);
			let ri = requests.generate_requests(&bucket, num_threads, delta);
			bucket.clear();
			heavy.push(ri);
			dh.relax_requests(&requests);
		}
		for ri in heavy {
			dh.relax_requests(&ri);
		}
	}
}

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

fn create_graph(size: usize, avg_dgr: usize) -> G {
	let mut g = G::new();

    for i in 0..size {
        g.insert(node!(i, Dist::new(u64::MAX)));
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

fn main() {

	for i in 0..1000 {
		let g = create_graph(50, 5);
		let a = g.to_vec();
		let s = g[0].clone();

		dijkstra(&s);

		let mut a_dists = a.iter().map(|n| (*n.key(), n.get())).collect::<Vec<_>>();
		a_dists.sort_by(|(a, _), (b, _)| a.cmp(b));

		for node in &a {
			node.set(u64::MAX);
		}

		parallel_delta_step_sssp(&s, 5, 4);

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

		fn attr(field: &str, value: &str) -> (String, String) {
			(field.to_string(), value.to_string())
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
	println!("Done!");
}
