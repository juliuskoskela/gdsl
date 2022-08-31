use core::num;
use std::{
	sync::{
		Arc,
		RwLock,
		atomic::{
			AtomicBool,
			AtomicUsize,
			Ordering,
		},
	},
	collections::{
		BinaryHeap
	},
	cmp::Reverse,
};
use std::cell::Cell;
use std::io::Write;
use std::fs::File;
// use rayon::prelude::IntoParallelRefIterator;
// use rayon::ThreadPoolBuilder;
// use rayon::ThreadPool;
use gdsl::async_digraph::*;

use gdsl::{
	async_digraph_node as node,
};

type N = Node<usize, Dist, u64>;
type E = Edge<usize, Dist, u64>;
type G = Graph<usize, Dist, u64>;

#[derive(Clone)]
pub struct Dist {
	dist: Arc<Cell<u64>>,
}

impl Dist {
	pub fn new(dist: u64) -> Self {
		Self {
			dist: Arc::new(Cell::new(dist)),
		}
	}

	pub fn get(&self) -> u64 {
		self.dist.get()
	}

	pub fn set(&self, dist: u64) {
		self.dist.set(dist)
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

#[derive(Clone)]
struct Sigterm {
	flag: Arc<AtomicBool>,
}

impl Sigterm {
	pub fn new() -> Self {
		Self {
			flag: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn terminate(&self) {
		self.flag.store(true, Ordering::Relaxed);
	}

	pub fn is_terminated(&self) -> bool {
		self.flag.load(Ordering::Relaxed)
	}
}

struct DeltaThreadPool {
	threads: Vec<std::thread::JoinHandle<()>>,
	sync_outer: Arc<AtomicUsize>,
	sync_inner: Arc<AtomicUsize>,
	lr: Arc<Requests>,
	hr: Arc<Requests>,
	num_threads: usize,
	delta: u64,
	sigterm: Sigterm,
}

impl DeltaThreadPool {
	fn new(num_threads: usize, delta: u64) -> Self {
		Self {
			threads: vec![],
			sync_outer: Arc::new(AtomicUsize::new(0)),
			sync_inner: Arc::new(AtomicUsize::new(0)),
			lr: Arc::new(Requests::new(num_threads)),
			hr: Arc::new(Requests::new(num_threads)),
			num_threads,
			delta,
			sigterm: Sigterm::new(),
		}
	}

	// pub fn park(&mut self, delta_heap: Arc<DeltaHeap>, thread_idx: usize) {
	// 	let requests = self.requests.get(thread_idx);
	// 	let sigterm = self.sigterm.clone();
	// 	let semaphore = self.semaphore.clone();
	// 	let t = std::thread::Builder::new()
	// 		.name(format!["T: {}", thread_idx])
	// 		.spawn(move || {
	// 			loop {
	// 				std::thread::park();
	// 				if sigterm.is_terminated() {
	// 					break;
	// 				}
	// 				while let Some((u, v, e)) = requests.write().unwrap().pop() {
	// 					println!("{:?} relax {} -> {}: {}", std::thread::current().id(), u.key(), v.key(), e);
	// 					delta_heap.relax(&v, u.get() + e);
	// 				}
	// 				println!("----");
	// 				semaphore.fetch_sub(1, Ordering::Relaxed);
	// 			}
	// 		})
	// 		.unwrap();
	// 	self.threads.push(t);
	// }

	pub fn park(&mut self, thread_heap: Arc<DeltaHeap>, thread_idx: usize) {
		let lr = self.lr.clone();
		let hr = self.hr.clone();
		let delta = self.delta;
		// let sigterm = self.sigterm.clone();
		let sync_outer = self.sync_outer.clone();
		let sync_inner = self.sync_inner.clone();

		let t = std::thread::Builder::new()
			.name(format!["T: {}", thread_idx])
			.spawn(move || {
				let local_lr = lr.get(thread_idx);
				let local_hr = hr.get(thread_idx);
				loop {
					std::thread::park();
					sync_outer.fetch_add(1, Ordering::Relaxed);
					while let Some(cur_idx) = thread_heap.min_idx() {
						loop {
							lr.generate_requests(&thread_heap.get(cur_idx), &hr, delta);
							println!("PHASE: relax light requests");
							sync_inner.fetch_add(1, Ordering::Relaxed);
							while let Some((u, v, e)) = local_lr.write().unwrap().pop() {
								println!("{:?} relax {} -> {}: {}", std::thread::current().id(), u.key(), v.key(), e);
								thread_heap.relax(&v, u.get() + e);
							}
							sync_inner.fetch_sub(1, Ordering::Relaxed);
							while sync_inner.load(Ordering::Relaxed) > 0 {
								continue;
							}
							if thread_heap.is_empty(cur_idx) {
								break;
							}
						}
						println!("PHASE: relax heavy requests");
						sync_inner.fetch_add(1, Ordering::Relaxed);
						while let Some((u, v, e)) = local_hr.write().unwrap().pop() {
							println!("{:?} relax {} -> {}: {}", std::thread::current().id(), u.key(), v.key(), e);
							thread_heap.relax(&v, u.get() + e);
						}
						sync_inner.fetch_sub(1, Ordering::Relaxed);
						while sync_inner.load(Ordering::Relaxed) > 0 {
							continue;
						}
					}
					sync_outer.fetch_sub(1, Ordering::Relaxed);
					while sync_outer.load(Ordering::Relaxed) > 0 {
						continue;
					}
				}
			})
			.unwrap();
		self.threads.push(t);
	}

	pub fn build(source: &N, num_threads: usize, delta: u64)  -> Self
	{
		let mut thread_heaps = vec![];
		for _ in 0..num_threads {
			thread_heaps.push(DeltaHeap::new(delta));
		}
		thread_heaps[0].relax(source, 0);
		// println!("DOES IT REACH HERE?");
		let mut pool = Self::new(num_threads, delta);
		for i in 0..pool.num_threads {
			pool.park(thread_heaps[i].clone(), i);
		}
		pool
	}

	pub fn run(&self) {
		for t in self.threads.iter() {
			// self.semaphore.fetch_add(1, Ordering::Relaxed);
			t.thread().unpark();
		}
		// while self.semaphore.load(Ordering::Relaxed) > 0 {
		// 	continue;
		// }
	}

	pub fn terminate(&self) {
		self.sigterm.terminate();
		for t in self.threads.iter() {
			t.thread().unpark();
		}
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

	fn get(&self, i: usize) -> Arc<RwLock<Vec<E>>> {
		match self.requests.get(i) {
			Some(r) => r.clone(),
			None => panic!("No request at index {}", i),
		}
	}

	fn generate_requests(&self, dbkt: &DeltaBucket, heavy_requests: &Requests, delta: u64) {
		let mut bucket_lock = dbkt.bucket.write().unwrap();
		while let Some(node) = bucket_lock.pop() {
			for (_, v, e) in &node {
				match e <= delta {
					false => {
						heavy_requests.requests[v.key() % self.num_threads]
							.write()
							.unwrap()
							.push((node.clone(), v, e));
					}
					true => {
						self.requests[v.key() % self.num_threads]
							.write()
							.unwrap()
							.push((node.clone(), v, e));
					}
				}
			}
		}
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
	heap: RwLock<Vec<DeltaBucket>>,
	delta: u64,
}

impl DeltaHeap {
	pub fn new(delta: u64) -> Arc<Self> {
		Arc::new(Self {
			heap: RwLock::new(Vec::new()),
			delta,
		})
	}

	pub fn len(&self) -> usize {
		self.heap.read().unwrap().len()
	}

	pub fn resize(&self, new_size: usize) {
		match self.heap.write() {
			Ok(mut heap) => {
				heap.resize(new_size, DeltaBucket::new());
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn insert(&self, node: &N, idx: usize) {
		if idx >= self.len() {
			self.resize((idx + 1) * 2);
		}
		match self.heap.read() {
			Ok(heap) => {
				heap[idx].insert(node.clone());
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn remove(&self, node: &N, idx: usize) {
		match self.heap.read() {
			Ok(heap) => {
				heap[idx].remove(node.clone());
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn get(&self, idx: usize) -> DeltaBucket {
		match self.heap.read() {
			Ok(heap) => {
				heap[idx].clone()
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn is_empty(&self, idx: usize) -> bool {
		match self.heap.read() {
			Ok(heap) => {
				heap[idx].is_empty()
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn min_idx(&self) -> Option<usize> {
		match self.heap.read() {
			Ok(heap) => {
				for (i, bucket) in heap.iter().enumerate() {
					if !bucket.is_empty() {
						return Some(i);
					}
				}
			}
			Err(e) => { panic!("{:?}", e); }
		}
		None
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
			println!("improved {} from {} to {}", v.key(), cur_dist, new_dist);
		}
	}
}

pub fn parallel_delta_step_sssp(source: &N, delta: u64, num_threads: usize) {
	let pool = DeltaThreadPool::build(source, num_threads, delta);
	pool.run();
	// pool.terminate();
	// delta_heap.relax(source, 0);

	// while let Some(cur_idx) = delta_heap.min_idx() {
	// 	loop {
	// 		light_pool.requests.generate_requests(&delta_heap.get(cur_idx), &heavy_pool.requests, delta);
	// 		println!("PHASE: relax light requests");
	// 		light_pool.run();
	// 		if delta_heap.is_empty(cur_idx) {
	// 			break;
	// 		}
	// 	}
	// 	println!("PHASE: relax heavy requests");
	// 	heavy_pool.run();
	// }
	// light_pool.terminate();
	// heavy_pool.terminate();
}

#[test]
fn test_1() {
	let g = create_graph(20, 10);
	let s = g[0].clone();

	parallel_delta_step_sssp(&s, 2, 4);
}

#[test]
fn test_parallel_delta_step_sssp() {

	for i in 0..1000 {
		let g = create_graph(10, 5);
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