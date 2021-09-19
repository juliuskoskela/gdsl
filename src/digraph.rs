///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	fmt:: {
		Debug,
		Display,
	},
	hash::Hash,
};

use crate::global::*;
use crate::node::*;
use crate::edge_list::*;

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// Digraph

pub struct Digraph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	nodes: VertexPool<K, N, E>
}

///////////////////////////////////////////////////////////////////////////////
///
/// Digraph: Implementations

impl<K, N, E> Digraph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	pub fn new() -> Self {
		Self {
			nodes: VertexPool::new()
		}
	}

	pub fn insert(&mut self, key: K, data: N) {
		if self.nodes.contains_key(&key) {
			let node = self.nodes[&key].clone();
			node.store(data.clone());
		} else {
			let node = Vertex::new(Node::new(key.clone(), data.clone()));
			self.nodes.insert(key.clone(), node);
		}
	}

	pub fn connect(&self, source: &K, target: &K, data: E) {
		if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
			connect(&self.nodes[source], &self.nodes[target], data);
		}
	}

	pub fn disconnect(&self, source: &K, target: &K) {
		if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
			disconnect(&self.nodes[source], &self.nodes[target]);
		}
	}

	pub fn bfs(&self, source: &K, target: &K) -> Option<EdgeList<K, N, E>> {
		if self.nodes.contains_key(source) && self.nodes.contains_key(source) {
			Some(self.nodes[source].bfs_directed(&self.nodes[target]))
		} else {
			None
		}
	}
}

///////////////////////////////////////////////////////////////////////////////
