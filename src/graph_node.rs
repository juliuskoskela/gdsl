use std::hash::Hash;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fmt::Debug;
use crate::graph_edge::GraphEdge;
use crate::graph_types::Lock;

type NodeEdges<E> = Vec<GraphEdge<E>>;

#[derive(Debug)]
pub struct
GraphNode<K, N, E>
where
K: Hash + Eq + Clone + Debug,
N: Clone + Debug ,
E: Clone + Debug {
	arg: N,
	key: K,
	index: usize,
	pub to: NodeEdges<E>,
	pub from: NodeEdges<E>,
	lock: AtomicBool,
}

impl<K, N, E> Clone
for GraphNode<K, N, E>
where
K: Hash + Eq + Clone + Debug,
N: Clone + Debug ,
E: Clone + Debug {
	fn clone(&self) -> Self {
		GraphNode {
			arg: self.arg.clone(),
			key: self.key.clone(),
			index: self.index,
			to: self.to.clone(),
			from: self.from.clone(),
			lock: AtomicBool::new(self.lock.load(Ordering::Relaxed)),
		}
	}
}

impl<K, N, E>
GraphNode<K, N, E>
where
K: Hash + Eq + Clone + Debug,
N: Clone + Debug ,
E: Clone + Debug {
	pub fn new(key: K, arg: N, index: usize) -> Self {
		Self {
			arg,
			key,
			index,
			to: NodeEdges::new(),
			from: NodeEdges::new(),
			lock: AtomicBool::new(false),
		}
	}
	pub fn lock_open(&self) {
		self.lock.store(false, Ordering::Relaxed)
	}
	pub fn lock_close(&self) {
		self.lock.store(true, Ordering::Relaxed)
	}
	pub fn lock_try(&self) -> Lock {
		let lock_bool = self.lock.load(Ordering::Relaxed);
		if lock_bool == false {
			return Lock::OPEN;
		}
		else {
			return Lock::CLOSED;
		}
	}
	pub fn get_arg_mut(&mut self) -> &mut N {
		&mut self.arg
	}
	pub fn get_arg(&self) -> &N {
		&self.arg
	}
	pub fn get_key(&self) -> &K {
		&self.key
	}
	pub fn get_index(&self) -> usize {
		self.index
	}
	pub fn find_edge_to(&self, target: usize) -> Option<&GraphEdge<E>> {
		for edge in self.to.iter() {
			if edge.get_target() == target {
				return Some(edge);
			}
		}
		None
	}
	pub fn find_edge_to_mut(&mut self, target: usize) -> Option<&mut GraphEdge<E>> {
		for edge in self.to.iter_mut() {
			if edge.get_target() == target {
				return Some(edge);
			}
		}
		None
	}
	pub fn find_edge(&self, source: usize) -> Option<&GraphEdge<E>> {
		for edge in self.from.iter() {
			if edge.get_source() == source {
				return Some(edge);
			}
		}
		None
	}
	pub fn find_edge_from(&mut self, source: usize) -> Option<&mut GraphEdge<E>> {
		for edge in self.from.iter_mut() {
			if edge.get_source() == source {
				return Some(edge);
			}
		}
		None
	}
}