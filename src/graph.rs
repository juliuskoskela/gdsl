use std::fmt::Display;
use std::collections::HashMap;
use std::hash::Hash;
use crate::node::*;
use crate::*;

pub trait FmtDot {
	fn fmt_dot(&self) -> String;
}

impl FmtDot for Empty {
	fn fmt_dot(&self) -> String {
		String::new()
	}
}

pub struct Graph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	nodes: HashMap<K, Node<K, N, E>>,
}

impl<'a, K, N, E> Graph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new() -> Self {
		Self {
			nodes: HashMap::new(),
		}
	}

	pub fn contains(&self, key: &K) -> bool {
		self.nodes.contains_key(key)
	}

	pub fn len(&self) -> usize {
		self.nodes.len()
	}

	pub fn get(&self, key: &K) -> Option<Node<K, N, E>> {
		self.nodes.get(key).map(|node| node.clone())
	}

	pub fn insert(&mut self, node: Node<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	pub fn remove(&mut self, node: Node<K, N, E>) -> Option<Node<K, N, E>> {
		self.nodes.remove(node.key())
	}

	pub fn roots(&self) -> Vec<Node<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn leafs(&self) -> Vec<Node<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn orphans(&self) -> Vec<Node<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty() && node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn fmt_dot(&self) -> String
	 where
		N: FmtDot,
		E: FmtDot,
	{
		let mut s = String::new();
		s.push_str("digraph {\n");
		for node in self.nodes.values() {
			s.push_str(&format!("    {} [ label=\"{}: {}\" ]\n", node.key(), node.key(), node.fmt_dot()));
		}
		for node in self.nodes.values() {
			for edge in node {
				s.push_str(&format!("    {} -> {} [ label=\"{}\" ]\n", edge.source().key(), edge.target().key(), edge.fmt_dot()));
			}
		}
		s.push_str("}");
		s
	}
}

impl<'a, K, N, E> std::ops::Index<K> for Graph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = Node<K, N, E>;

	fn index(&self, key: K) -> &Self::Output {
		&self.nodes[&key]
	}
}