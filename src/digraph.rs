/// Icludes
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::results::*;
use crate::edge::*;
use crate::global::*;
use crate::node::*;

/// Digraph

pub struct Digraph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    nodes: NodeRefPool<K, N, E>,
	edge_count: usize,
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
    pub fn new() -> Self {
        Self {
            nodes: NodeRefPool::new(),
			edge_count: 0,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

	pub fn edge_count(&self) -> usize {
        self.edge_count
    }

	pub fn bytesize(&self) -> usize {
		let node_size = std::mem::size_of::<Node<K, N, E>>();
		let edge_size = std::mem::size_of::<Edge<K, N, E>>();
		(self.node_count() * node_size) + (self.edge_count() * edge_size)
	}

	pub fn print(&self) {
		for n in self.nodes.iter() {
			println!("{}", n.1);
		}
	}

	pub fn node(&self, key: &K) -> Option<&NodeRef<K, N, E>> {
		self.nodes.get(key)
	}

	pub fn edge(&self, source: &K, target: &K) -> Option<EdgeRef<K, N, E>> {
		let s = self.nodes.get(source);
		let t = self.nodes.get(target);
		match s {
            Some(ss) => match t {
                Some(tt) => ss.outbound().find(ss, tt),
                None => None,
            },
            None => None,
        }
	}

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

    pub fn connect(&mut self, source: &K, target: &K, data: E) {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if connect(&self.nodes[source], &self.nodes[target], data) {
				self.edge_count += 1;
			}
        }
    }

    pub fn disconnect(&mut self, source: &K, target: &K) {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if disconnect(&self.nodes[source], &self.nodes[target]) {
				self.edge_count -= 1;
			}
        }
    }

    pub fn get_leaves(&self) -> Vec<NodeRef<K, N, E>> {
        let mut res: Vec<NodeRef<K, N, E>> = vec![];
        for n in self.nodes.values() {
            if n.outbound().is_empty() {
                res.push(n.clone());
            }
        }
        res
    }

	pub fn depth_first(
		&self,
		source: &K,
		target: &K,
		f: fn (&EdgeRef<K, N, E>, &NodeRef<K, N, E>) -> Traverse
	) -> Option<Results<K, N, E>> {
		let s = self.node(source);
		let t = self.node(target);

		match s {
            Some(ss) => match t {
                Some(tt) => Some(depth_traversal_directed(ss, tt, f)),
                None => None,
            },
            None => None,
        }
	}

	pub fn breadth_first(
		&self,
		source: &K,
		target: &K,
		f: fn (&EdgeRef<K, N, E>) -> Traverse
	) -> Option<Results<K, N, E>> {
		let s = self.node(source);
		let t = self.node(target);

		match s {
            Some(ss) => match t {
                Some(tt) => breadth_traversal_directed(ss, tt, f),
                None => None,
            },
            None => None,
        }
	}
}
