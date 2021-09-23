/// Icludes

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

/// Digraph

pub struct Digraph<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display + Sync + Send,
	N: Clone + Debug + Display + Sync + Send,
	E: Clone + Debug + Display + Sync + Send,
{
	nodes: NodeRefPool<K, N, E>
}

/// Digraph: Implementations

impl<K, N, E> Default for Digraph<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display + Sync + Send,
	N: Clone + Debug + Display + Sync + Send,
	E: Clone + Debug + Display + Sync + Send,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, N, E> Digraph<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display + Sync + Send,
	N: Clone + Debug + Display + Sync + Send,
	E: Clone + Debug + Display + Sync + Send,
{
	pub fn new() -> Self { Self { nodes: NodeRefPool::new() } }

	pub fn node_count(&self) -> usize {	self.nodes.len() }

	pub fn insert(&mut self, key: K, data: N) -> bool {
		if self.nodes.contains_key(&key) {
			let node = self.nodes[&key].clone();
			node.store(data);
			false
		} else {
			let node = NodeRef::new(Node::new(key.clone(), data));
			self.nodes.insert(key, node);
			true
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

	pub fn get_leaves(&self) -> Vec<NodeRef<K, N, E>> {
		let mut res: Vec<NodeRef<K, N, E>> = vec![];
		for n in self.nodes.values() {
			if n.outbound.borrow().list.is_empty() {
				res.push(n.clone());
			}
		}
		res
	}

	pub fn bfs(&self, source: &K, target: &K) -> Option<EdgeList<K, N, E>> {
		let s = self.nodes.get(source);
		let t = self.nodes.get(target);

		match s {
			Some(ss) => {
				match t {
					Some(tt) => { ss.traverse_breadth(tt) }
					None => { None }
				}
			}
			None => { None }
		}
	}

	pub fn shortest_path(&self, source: &K, target: &K) -> Option<EdgeList<K, N, E>> {
		let s = self.nodes.get(source);
		let t = self.nodes.get(target);

		match s {
			Some(ss) => {
				match t {
					Some(tt) => { ss.shortest_path(tt) }
					None => { None }
				}
			}
			None => { None }
		}
	}
}

