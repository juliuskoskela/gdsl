use crate::core::*;
use std::{
	collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait Graph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn new() -> Self;
	fn directed() -> bool;
    fn add_node(&mut self, key: K, data: N) -> bool;
	fn get_node(&self, node: &K) -> Option<&ArcNode<K, N, E>>;
	fn iter_nodes(&self, f: &dyn Fn (ArcNode<K, N, E>));
    fn add_edge(&mut self, source: &K, target: &K, data: E) -> bool;
	fn del_edge(&mut self, source: &K, target: &K) -> bool;
	fn get_edge(&self, source: &K, target: &K) -> Option<ArcEdge<K, N, E>>;
	fn node_count(&self) -> usize;
	fn edge_count(&self) -> usize;

	// ========================================================================

	fn size_of(&self) -> usize {
		(self.node_count() * std::mem::size_of::<Node<K, N, E>>())
		+ (self.edge_count() * std::mem::size_of::<Edge<K, N, E>>())
	}

	fn depth_first<F>(&self, source: &K, f: F) -> Option<Vec<WeakEdge<K, N, E>>>
	where
		F: Fn (&ArcEdge<K, N, E>) -> Traverse,
	{
		match self.get_node(source) {
			Some(s) => {
				match Self::directed() {
					true => { directed_depth_traversal(&s, f) }
					false => { undirected_depth_traversal(&s, f) }
				}
			}
			None => None
		}
	}

	fn breadth_first<F>(&self, source: &K, f: F) -> Option<Vec<WeakEdge<K, N, E>>>
	where
		F: Fn (&ArcEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
	{
		match self.get_node(source) {
			Some(s) => {
				match Self::directed() {
					true => { parallel_directed_breadth_traversal(&s, f) }
					false => { parallel_undirected_breadth_traversal(&s, f) }
				}
			}
			None => None
		}
	}

	fn print_nodes(&self) {
		self.iter_nodes(&| node | {
			println!("	{}", node);
		})
	}

	fn print_edges(&self) {
		let sign;
		match Self::directed() {
			true => { sign = "->"}
			false => { sign = "--" }
		}
		self.iter_nodes(&| node | {
			for edge in node.outbound().iter() {
				println!("	{} {} {} [label = \"{}\"]",
				edge.source().key(),
				sign,
				edge.target().key(),
				edge.load())
			}
		})
	}

	fn print_graph(&self) {
		let name;
		match Self::directed() {
			true => { name = "digraph"}
			false => { name = "graph" }
		}
		println!("{} {{", name);
		self.print_nodes();
		self.print_edges();
		println!("}}");
	}
}

pub struct Ungraph<K, N = Empty, E = Empty>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    nodes: HashMap<K, ArcNode<K, N, E>>,
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
            let node = ArcNode::new(Node::new(key.clone(), data));
            self.nodes.insert(key, node);
            true
        }
    }

	fn get_node(&self, node: &K) -> Option<&ArcNode<K, N, E>> {
		self.nodes.get(node)
	}

	fn iter_nodes(&self, f: &dyn Fn (ArcNode<K, N, E>)) {
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

	fn get_edge(&self, source: &K, target: &K) -> Option<ArcEdge<K, N, E>> {
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