use std::hash::Hash;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fmt;
use crate::graph_edge::GraphEdge;

type NodeEdges<E> = Vec<GraphEdge<E>>;

#[derive(Debug)]
pub struct
GraphNode<K, N, E>
where
K: Hash + Eq + Clone + fmt::Debug,
N: Clone + fmt::Debug ,
E: Clone + fmt::Debug {
	pub arg: N,
	pub key: K,
	pub index: usize,
	pub to: NodeEdges<E>,
	pub from: NodeEdges<E>,
	pub valid: AtomicBool,
}

impl<K, N, E> Clone
for GraphNode<K, N, E>
where
K: Hash + Eq + Clone + fmt::Debug,
N: Clone + fmt::Debug ,
E: Clone + fmt::Debug {
	fn clone(&self) -> Self {
		GraphNode {
			arg: self.arg.clone(),
			key: self.key.clone(),
			index: self.index,
			to: self.to.clone(),
			from: self.from.clone(),
			valid: AtomicBool::new(self.valid.load(Ordering::Relaxed)),
		}
	}
}

impl<K, N, E>
GraphNode<K, N, E>
where
K: Hash + Eq + Clone + fmt::Debug,
N: Clone + fmt::Debug ,
E: Clone + fmt::Debug {
	pub fn new(key: K, arg: N) -> Self {
		Self {
			arg,
			key,
			index: 0,
			to: NodeEdges::new(),
			from: NodeEdges::new(),
			valid: AtomicBool::new(true),
		}
	}
	pub fn open(&self) {
		self.valid.store(true, Ordering::Relaxed)
	}
	pub fn close(&self) {
		self.valid.store(false, Ordering::Relaxed)
	}
}