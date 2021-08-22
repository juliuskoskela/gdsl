///////////////////////////////////////////////////////////////////////////////
/// Graph
///////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::hash::Hash;
use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::fmt;
use crate::graph_node::GraphNode;
use crate::graph_edge::GraphEdge;

type GraphKeys<K> = HashMap<K, usize>;
type GraphNodes<K, N, E> = Vec<GraphNode<K, N, E>>;

#[derive(Clone, Debug)]
pub struct
Graph<K, N, E>
where
K: Hash + Eq + Clone + fmt::Debug,
N: Clone + fmt::Debug ,
E: Clone + fmt::Debug {
	nodes: GraphNodes<K, N, E>,
	keys: GraphKeys<K>,
	edge_count: usize,
}

impl<K, N, E>
Graph<K, N, E>
where
K: Hash + Eq + Clone + fmt::Debug,
N: Clone + fmt::Debug ,
E: Clone + fmt::Debug {
	pub fn new() -> Self {
		Self {
			nodes: GraphNodes::new(),
			keys: GraphKeys::new(),
			edge_count: 0,
		}
	}
	pub fn count_nodes(&self) -> usize {
		self.nodes.len()
	}
	pub fn count_edges(&self) -> usize {
		self.edge_count
	}
	pub fn add_node(&mut self, key: &K, arg: N) -> bool {
		if self.keys.contains_key(key) {
			println!("Graph contains key!");
			return false;
		}
		let mut node = GraphNode::new(key.clone(), arg);
		node.index = self.nodes.len();
		self.nodes.push(node);
		self.keys.insert(key.clone(), self.nodes.len() - 1);
		true
	}
	pub fn get_node(&self, key: &K) -> Option<GraphNode<K, N, E>> {
		if self.keys.contains_key(key) {
			let i = self.keys.get(key).unwrap();
			let n = self.nodes.get(*i).unwrap().clone();
			return Some(n);
		}
		None
	}
	pub fn update_node(&mut self, n: GraphNode<K, N, E>) {
		self.nodes[n.index] = n.clone();
	}
	pub fn get_node_arg(&self, key: &K) -> Option<N> {
		if self.keys.contains_key(key) {
			let i = self.keys.get(key).unwrap();
			let n = self.nodes.get(*i).unwrap().clone().arg;
			return Some(n);
		}
		None
	}
	pub fn update_node_arg(&mut self, key: &K, n: N) {
		self.nodes[*self.keys.get(key).unwrap()].arg = n;
	}
	pub fn connect(&mut self, u: &K, v: &K, arg: E) -> bool {
		if !self.keys.contains_key(u) || !self.keys.contains_key(v) {
			println!("Nodes do not exist!");
			return false;
		}
		let u_index = *self.keys.get(u).unwrap();
		let v_index = *self.keys.get(v).unwrap();
		let edge = GraphEdge::new(u_index, v_index, arg);
		let node = self.nodes.get_mut(u_index).unwrap();
		node.to.push(edge.clone());
		let node = self.nodes.get_mut(v_index).unwrap();
		node.from.push(edge);
		self.edge_count += 1;
		true
	}
	fn bfs_clean(&self, res: &Vec<&GraphEdge<E>>) {
		for edge in res.iter() {
			edge.open();
			let u = self.nodes.get(edge.u).unwrap();
			let v = self.nodes.get(edge.v).unwrap();
			u.open();
			v.open();
		}
	}
	pub fn bfs(&mut self, s: &K, t: &K) -> Option<Vec<&GraphEdge<E>>> {
		if !self.keys.contains_key(s) {
			println!("Nodes do not exist!");
			return None;
		}
		let mut res = Vec::new();
		let mut queue = VecDeque::<usize>::new();
		let source = *self.keys.get(s).unwrap();
		queue.push_back(source);
		while queue.len() > 0 {
			let ui = queue.pop_front().unwrap();
			let u = self.nodes.get(ui).unwrap();
			for edge in u.to.iter() {
				let v = self.nodes.get(edge.v).unwrap();
				if v.valid.load(Ordering::Relaxed) == true
				&& edge.valid.load(Ordering::Relaxed) == true {
					v.close();
					edge.close();
					queue.push_back(v.index);
					res.push(edge);
					if v.key == *t {
						self.bfs_clean(&res);
						return Some(res);
					}
				}
			}
		}
		self.bfs_clean(&res);
		Some(res)
	}
}
