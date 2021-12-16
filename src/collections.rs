use crate::core::*;
use crate::graph::*;
use std::{
	collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
	sync::{Arc}
};

pub struct Ungraph<K, N = Empty, E = Empty>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    nodes: HashMap<K, Arc<Node<K, N, E>>>,
	edge_count: usize,
}

impl<K, N, E> Graph<K, N, E> for Ungraph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	fn new() -> Self {
        Self {
            nodes: HashMap::new(),
			edge_count: 0,
        }
    }

	fn directed() -> bool {
		false
	}

	fn add_node(&mut self, key: K, data: N) -> bool {
        if self.nodes.contains_key(&key) {
            false
        } else {
            let node = Arc::new(Node::new(key.clone(), data));
            self.nodes.insert(key, node);
            true
        }
    }

	fn get_node(&self, node: &K) -> Option<Arc<Node<K, N, E>>> {
		match self.nodes.get(node) {
			Some(n) => { Some(n.clone()) }
			None => None
		}
	}

	fn iter_nodes(&self, f: &dyn Fn (Arc<Node<K, N, E>>)) {
		for (_, node) in self.nodes.iter() {
			f(node.clone());
		}
	}

    fn add_edge(&mut self, source: &K, target: &K, data: E) -> bool {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if connect(&self.nodes[source], &self.nodes[target], data) {
				self.edge_count += 1;
				return true;
			}
        }
		false
	}

	fn del_edge(&mut self, source: &K, target: &K) -> bool {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if disconnect(&self.nodes[source], &self.nodes[target]) {
				self.edge_count -= 1;
				return true;
			}
        }
		false
    }

	fn get_edge(&self, source: &K, target: &K) -> Option<Arc<Edge<K, N, E>>> {
		let s = self.nodes.get(source);
		let t = self.nodes.get(target);
		match s {
            Some(ss) => match t {
                Some(tt) => ss.find_outbound(tt),
                None => None,
            },
            None => None,
        }
	}

	fn node_count(&self) -> usize {
        self.nodes.len()
    }
	fn edge_count(&self) -> usize {
		self.edge_count
    }
}

pub struct Digraph<K, N = Empty, E = Empty>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    nodes: HashMap<K, Arc<Node<K, N, E>>>,
	edge_count: usize,
}

impl<K, N, E> Graph<K, N, E> for Digraph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	fn new() -> Self {
        Self {
            nodes: HashMap::new(),
			edge_count: 0,
        }
    }

	fn directed() -> bool {
		true
	}

	fn add_node(&mut self, key: K, data: N) -> bool {
        if self.nodes.contains_key(&key) {
            false
        } else {
            let node = Arc::new(Node::new(key.clone(), data));
            self.nodes.insert(key, node);
            true
        }
    }

	fn get_node(&self, node: &K) -> Option<Arc<Node<K, N, E>>>  {
		match self.nodes.get(node) {
			Some(n) => { Some(n.clone()) }
			None => None
		}
	}

	fn iter_nodes(&self, f: &dyn Fn (Arc<Node<K, N, E>>)) {
		for (_, node) in self.nodes.iter() {
			f(node.clone());
		}
	}

    fn add_edge(&mut self, source: &K, target: &K, data: E) -> bool {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if connect(&self.nodes[source], &self.nodes[target], data) {
				self.edge_count += 1;
				return true;
			}
        }
		false
	}

	fn del_edge(&mut self, source: &K, target: &K) -> bool {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if disconnect(&self.nodes[source], &self.nodes[target]) {
				self.edge_count -= 1;
				return true;
			}
        }
		false
    }

	fn get_edge(&self, source: &K, target: &K) -> Option<Arc<Edge<K, N, E>>> {
		let s = self.nodes.get(source);
		let t = self.nodes.get(target);
		match s {
            Some(ss) => match t {
                Some(tt) => ss.find_outbound(tt),
                None => None,
            },
            None => None,
        }
	}

	fn node_count(&self) -> usize {
        self.nodes.len()
    }
	fn edge_count(&self) -> usize {
		self.edge_count
    }
}
