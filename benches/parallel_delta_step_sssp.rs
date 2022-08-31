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
		BinaryHeap,
		HashSet,
		HashMap,
	},
	cmp::Reverse,
	hash::{
		Hash,
		Hasher,
	},
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
	async_digraph_connect as connect,
};

pub type Dist = Cell<u64>;
type N = Node<usize, Dist, u64>;
type E = Edge<usize, Dist, u64>;
type G = Graph<usize, Dist, u64>;

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
	semaphore: Arc<AtomicUsize>,
	requests: Arc<Requests>,
	num_threads: usize,
	sigterm: Sigterm,
}

impl DeltaThreadPool {
	fn new(num_threads: usize) -> Self {
		Self {
			threads: vec![],
			semaphore: Arc::new(AtomicUsize::new(0)),
			requests: Requests::new(num_threads),
			num_threads,
			sigterm: Sigterm::new(),
		}
	}

	pub fn park(&mut self, heap: Arc<DeltaHeap>, thread_idx: usize) {
		let requests = self.requests.clone();
		let sigterm = self.sigterm.clone();
		let semaphore = self.semaphore.clone();

		let t = std::thread::Builder::new()
			.name(format!["T: {}", thread_idx])
			.spawn(move || {
				let tcur = std::thread::current();
				let tname = tcur.name().unwrap();
				loop {
					std::thread::park();
					if sigterm.is_terminated() {
						break;
					}
					let request = requests.get(tname);
					// println!("ThreadIdx = {}", thread_idx);
					// println!("{}: RELAX PHASE: request count: {}", tname, request.read().unwrap().len());
					for (u, v, e) in request.read().unwrap().iter() {
						// println!("{} relaxing edge ({} -> {}: {})", tname, u.key(), v.key(), e);
						heap.relax(&v, u.get() + e, tname);
					}
					// println!("{} PHASE FINISHED", tname);
					semaphore.fetch_sub(1, Ordering::Relaxed);
				}
			})
			.unwrap();

		self.threads.push(t);
	}

	pub fn build(delta_heap: Arc<DeltaHeap>, num_threads: usize)  -> Self
	{
		let mut pool = Self::new(num_threads);
		for i in 0..pool.num_threads {
			pool.park(delta_heap.clone(), i);
		}
		pool
	}

	pub fn run(&self) {
		for t in self.threads.iter() {
			self.semaphore.fetch_add(1, Ordering::Relaxed);
			t.thread().unpark();
		}
		while self.semaphore.load(Ordering::Relaxed) > 0 {
			continue;
		}
	}

	pub fn terminate(&mut self) {
		self.sigterm.terminate();
		for t in self.threads.iter_mut() {
			t.thread().unpark();
		}
	}
}

struct Requests {
	requests: HashMap<String, Arc<RwLock<Vec<E>>>>,
	num_threads: usize,
	tnames: Vec<String>,
}

impl Requests {
	fn new(num_threads: usize) -> Arc<Self> {
		let mut new = Self {
			requests: HashMap::with_capacity(num_threads),
			num_threads,
			tnames: vec![],
		};

		for i in 0..num_threads {
			let new_tname = format!["T: {}", i];
			new.tnames.push(new_tname.clone());
			new.requests.insert(new_tname, Arc::new(RwLock::new(vec![])));
		}
		Arc::new(new)
	}

	fn get(&self, thread: &str) -> Arc<RwLock<Vec<E>>> {
		match self.requests.get(thread) {
			Some(r) => r.clone(),
			None => panic!("No thread named {} exists!", thread),
		}
	}

	pub fn clear(&self) {
		for (_, r) in self.requests.iter() {
			r.write().unwrap().clear();
		}
	}

	fn generate_requests(&self, bucket: &mut Vec<N>, heavy_requests: &Requests, delta: u64) {
		self.clear();
		while let Some(node) = bucket.pop() {
			for (u, v, e) in &node {
				match e <= delta {
					false => {
						let cur_tname = &self.tnames[v.key() % self.num_threads];
						// println!("T: Main GEN He for ({} -> {}: {}) for T({})", u.key(), v.key(), e, cur_tname);
						heavy_requests.requests[cur_tname]
							.write()
							.unwrap()
							.push((u, v, e));
					}
					true => {
						let cur_tname = &self.tnames[v.key() % self.num_threads];
						// println!("T: Main GEN Li for ({} -> {}: {}) for T({})", u.key(), v.key(), e, cur_tname);
						self.requests[cur_tname]
							.write()
							.unwrap()
							.push((u, v, e));
					}
				}
			}
		}
	}
}

struct DeltaHeap {
	heap: HashMap<String, RwLock<Vec<Vec<N>>>>,
	delta: u64,
	num_threads: usize,
}

impl DeltaHeap {
	pub fn new(delta: u64, num_threads: usize) -> Arc<Self> {
		let mut heap = HashMap::new();
		for i in 0..num_threads {
			heap.insert(format!["T: {}", i], RwLock::new(Vec::new()));
		}
		Arc::new(Self {
			heap,
			delta,
			num_threads,
		})
	}

	pub fn len(&self, thread: &str) -> usize {
		self.heap[thread].read().unwrap().len()
	}

	pub fn resize(&self, new_size: usize, thread: &str) {
		match self.heap[thread].write() {
			Ok(mut heap) => {
				heap.resize(new_size, Vec::new());
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn insert(&self, node: &N, idx: usize, thread: &str) {
		if idx >= self.len(thread) {
			self.resize((idx + 1) * 2, thread);
		}
		match self.heap[thread].write() {
			Ok(mut heap) => {
				heap[idx].push(node.clone());
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn remove(&self, node: &N, idx: usize, thread: &str) {
		match self.heap[thread].write() {
			Ok(mut heap) => {
				if let Some(bucket) = heap.get_mut(idx) {
					bucket.retain(|n| n != node);
				}
			}
			Err(e) => { panic!("{:?}", e); }
		}
	}

	pub fn get_level(&self, idx: usize) -> Vec<N> {
		let mut level = Vec::new();
		for (_, heap) in self.heap.iter() {
			match heap.write() {
				Ok(mut heap) => {
					if let Some(bucket) = heap.get_mut(idx) {
						level.extend(bucket.iter().cloned());
						bucket.clear();
					}
				}
				Err(e) => { panic!("{:?}", e); }
			}
		}
		level
	}

	pub fn bucket_is_empty(&self, idx: usize) -> bool {
		let mut is_empty = self.num_threads;
		for (_, heap) in self.heap.iter() {
			if let Ok(heap) = heap.read() {
				if let Some(bucket) = heap.get(idx) {
					if bucket.is_empty() {
						is_empty -= 1;
					}
				} else {
					is_empty -= 1;
				}
			}
		}
		if is_empty == 0 {
			true
		} else {
			false
		}
	}

	pub fn min_idx(&self) -> Option<usize> {
		let tallest_heap_len = self.heap
			.iter()
			.map(|(k, h)| h.read().unwrap().len())
			.max()
			.unwrap();

		let mut min_idx = 0;
		while min_idx < tallest_heap_len {
			for (key, heap) in self.heap.iter() {
				if let Ok(heap) = heap.read() {
					if let Some(bucket) = heap.get(min_idx) {
						if !bucket.is_empty() {
							return Some(min_idx);
						}
					}
				}
			}
			min_idx += 1;
		}
		None
	}

	fn relax(&self, v: &N, new_dist: u64, thread: &str) {
		let cur_dist = v.get();
		if new_dist < cur_dist {
			let old_idx = cur_dist / self.delta;
			let new_idx = new_dist / self.delta;
			if cur_dist < u64::MAX && old_idx != new_idx {
				self.remove(v, old_idx as usize, thread);
			}
			v.set(new_dist);
			self.insert(v, new_idx as usize, thread);
			// let cur_dist_str = if cur_dist == u64::MAX {
			// 	"(inf)".to_string()
			// } else {
			// 	cur_dist.to_string()
			// };
			// println!("DISTANCE UPDATED: {} from {} to {}", v.key(), cur_dist_str, new_dist);
		}
	}
}

pub fn parallel_delta_step_sssp(source: &N, delta: u64, num_threads: usize) {
	let delta_heap = DeltaHeap::new(delta, num_threads);
	let mut light_pool = DeltaThreadPool::build(delta_heap.clone(), num_threads);
	let mut heavy_pool = DeltaThreadPool::build(delta_heap.clone(), num_threads);

	delta_heap.relax(source, 0, "T: 0");

	while let Some(cur_idx) = delta_heap.min_idx() {
		loop {
			light_pool.requests.generate_requests(&mut delta_heap.get_level(cur_idx), &heavy_pool.requests, delta);
			// println!("PHASE: relax light requests");
			light_pool.run();
			if delta_heap.bucket_is_empty(cur_idx) {
				break;
			}
		}
		// println!("PHASE: relax heavy requests");
		heavy_pool.run();
		heavy_pool.requests.clear();
	}
	light_pool.terminate();
	heavy_pool.terminate();
}

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

pub fn create_graph_vec_distance_async(size: usize, avg_dgr: usize) -> Vec<N> {
	let mut g = Vec::new();

    for i in 0..size {
        g.push(node!(i, Dist::new(u64::MAX)));
    }

	for node in g.iter() {
		let cur_dgr = rand::random::<usize>() % avg_dgr + 1;
		for _ in 0..cur_dgr {
			connect!(&node => &g[rand::random::<usize>() % size], rand::random::<u64>() % 10 + 1);
		}
	}
	g
}

#[test]
fn test_1() {
	let g = create_graph_vec_distance_async(10, 5);
	let s = g[0].clone();

	parallel_delta_step_sssp(&s, 2, 4);
}

#[test]
fn test_parallel_delta_step_sssp() {

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
