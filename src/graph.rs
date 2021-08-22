use std::collections::HashMap;
use std::hash::Hash;
use std::collections::VecDeque;
use std::fmt;
use crate::graph_node::GraphNode;
use crate::graph_edge::GraphEdge;
use crate::graph_types::Lock;

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
	pub fn add_node(&mut self, key: K, arg: N) -> bool {
		if self.keys.contains_key(&key) {
			println!("Graph already contains key!");
			return false;
		}
		let node = GraphNode::new(key.clone(), arg, self.nodes.len());
		self.keys.insert(key, node.get_index());
		self.nodes.push(node);
		true
	}
	pub fn get_node(&self, key: &K) -> Option<&GraphNode<K, N, E>> {
		if self.keys.contains_key(key) {
			let i = self.keys[key];
			let n = self.nodes.get(i).unwrap();
			return Some(n);
		}
		None
	}
	pub fn get_node_mut(&mut self, key: &K) -> Option<&mut GraphNode<K, N, E>> {
		if self.keys.contains_key(key) {
			let i = self.keys[key];
			let n = self.nodes.get_mut(i).unwrap();
			return Some(n);
		}
		None
	}
	pub fn get_edge(&self, source: &K, target: &K) -> Option<&GraphEdge<E>> {
		if self.keys.contains_key(source) && self.keys.contains_key(target) {
			let s = self.keys[source];
			let t = self.keys[target];
			let res = self.nodes[s].find_edge_to(t);
			return res;
		}
		None
	}
	pub fn get_edge_mut(&mut self, source: &K, target: &K) -> Option<&mut GraphEdge<E>> {
		if self.keys.contains_key(source) && self.keys.contains_key(target) {
			let s = self.keys[source];
			let t = self.keys[target];
			let res = self.nodes[s].find_edge_to_mut(t);
			return res;
		}
		None
	}
	pub fn add_edge(&mut self, source: &K, target: &K, arg: E) -> bool {
		if !self.keys.contains_key(source) || !self.keys.contains_key(target) {
			println!("Nodes do not exist!");
			return false;
		}
		let t_index = *self.keys.get(source).unwrap();
		let s_index = *self.keys.get(target).unwrap();
		let edge = GraphEdge::new(t_index, s_index, arg);
		let node = self.nodes.get_mut(t_index).unwrap();
		node.to.push(edge.clone());
		let node = self.nodes.get_mut(s_index).unwrap();
		node.from.push(edge);
		self.edge_count += 1;
		true
	}
	fn bfs_clean(&self, res: &Vec<&GraphEdge<E>>) {
		for edge in res.iter() {
			edge.lock_open();
			let u = self.nodes.get(edge.get_source()).unwrap();
			let v = self.nodes.get(edge.get_target()).unwrap();
			u.lock_open();
			v.lock_open();
		}
	}
	pub fn bfs(&self, s: &K, t: &K) -> Option<Vec<&GraphEdge<E>>> {
		if !self.keys.contains_key(s) || !self.keys.contains_key(t) {
			println!("Nodes do not exist!");
			return None;
		}
		let mut res = Vec::new();
		let mut queue = VecDeque::<usize>::new();
		let source = self.keys[s];
		let target = self.keys[t];
		queue.push_back(source);
		while queue.len() > 0 {
			let u_index = queue.pop_front().unwrap();
			let u = &self.nodes[u_index];
			for edge in u.to.iter() {
				let v = &self.nodes[edge.get_target()];
				if v.lock_try() == Lock::OPEN
				&& edge.lock_try() == Lock::OPEN {
					v.lock_close();
					edge.lock_close();
					queue.push_back(v.get_index());
					res.push(edge);
					if v.get_index() == target {
						self.bfs_clean(&res);
						return Some(res);
					}
				}
			}
		}
		self.bfs_clean(&res);
		Some(res)
	}
	pub fn shortest_path(&self, s: &K, t: &K) -> Option<Vec<&GraphEdge<E>>> {
		let mut res = Vec::new();
		let bfs = self.bfs(s, t);
		if bfs.is_none() {
			println!("Path not found between s and t!");
			return None;
		}
		let mut edges = bfs.unwrap();
		res.push(edges.pop().unwrap());
		let mut i = 0;
		for e in edges.iter().rev() {
			let target = res[i].get_source();
			if e.get_target() == target {
				res.push(e);
				i += 1;
			}
		}
		res.reverse();
		Some(res)
	}
}
