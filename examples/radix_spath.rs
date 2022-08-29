use std::{
	sync::{
		Arc,
		Mutex,
	},
	collections::{
		BTreeMap,
		BinaryHeap
	},
};

use gdsl::async_digraph::*;

use gdsl::{
	async_digraph_connect as connect,
	async_digraph_node as node,
};

type N = Node<usize, Dist, u64>;
type E = Edge<usize, Dist, u64>;

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



fn main() {
	let mut a = BTreeMap::new();

	a.insert(0, 'A');
	a.insert(1, 'B');
	a.insert(2, 'C');

	let b = a.remove(&1);

	println!("{:?}", a);
	println!("{:?}", b);
}