//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::VecDeque
};

use ahash::AHashSet as HashSet;

use crate::digraph::node::*;
use self::{method::*, path::*};

pub struct BFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: Node<K, N, E>,
	target: Option<&'a K>,
	method: Method<'a, K, N, E>,
	transpose: Transposition,
}

impl<'a, K, N, E> BFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &Node<K, N, E>) -> Self {
		BFS {
			root: root.clone(),
			target: None,
			method: Method::NullMethod,
			transpose: Transposition::Outbound,
		}
	}

	pub fn target(mut self, target: &'a K) -> Self {
		self.target = Some(target);
		self
	}

	pub fn transpose(mut self) -> Self {
		self.transpose = Transposition::Inbound;
		self
	}

	pub fn map(mut self, f: Map<'a, K, N, E>) -> Self {
		self.method = Method::Map(f);
		self
	}

	pub fn filter(mut self, f: Filter<'a, K, N, E>) -> Self {
		self.method = Method::Filter(f);
		self
	}

	pub fn filter_map(mut self, f: FilterMap<'a, K, N, E>) -> Self {
		self.method = Method::FilterMap(f);
		self
	}

	pub fn search(&'a mut self) -> Option<Node<K, N, E>> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::default();

		queue.push_back(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Transposition::Outbound => {
				return self.loop_outbound_find(&mut visited, &mut queue);
			}
			Transposition::Inbound => {
				return self.loop_inbound_find(&mut visited, &mut queue);
			}
		}
	}

	pub fn search_cycle(&'a mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::default();
		let target_found;

		self.target = Some(self.root.key());
		queue.push_back(self.root.clone());

		match self.transpose {
			Transposition::Outbound => {
				target_found = self.loop_outbound(&mut edges, &mut visited, &mut queue);
			}
			Transposition::Inbound => {
				target_found = self.loop_inbound(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			return Some(Path::from_edge_tree(edges));
		}
		None
	}

	pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::default();
		let target_found;

		queue.push_back(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Transposition::Outbound => {
				target_found = self.loop_outbound(&mut edges, &mut visited, &mut queue);
			}
			Transposition::Inbound => {
				target_found = self.loop_inbound(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			return Some(Path::from_edge_tree(edges));
		}
		None
	}

	fn loop_outbound(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_front() {
			for (u, v, e) in node.iter_out() {
				if self.method.exec(&u, &v, &e) {
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						result.push((u, v.clone(), e));
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						queue.push_back(v.clone());
					}
				}
			}
		}
		false
	}

	fn loop_inbound(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_front() {
			for (v, u, e) in node.iter_in() {
				if self.method.exec(&u, &v, &e) {
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						result.push((u, v.clone(), e));
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						queue.push_back(v.clone());
					}
				}
			}
		}
		false
	}

	fn loop_outbound_find(
		&self,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		while let Some(node) = queue.pop_front() {
			for (u, v, e) in node.iter_out() {
				if self.method.exec(&u, &v, &e) {
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return Some(v);
						}
						queue.push_back(v.clone());
					}
				}
			}
		}
		None
	}

	fn loop_inbound_find(
		&self,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		while let Some(node) = queue.pop_front() {
			for (v, u, e) in node.iter_in() {
				if self.method.exec(&u, &v, &e) {
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return Some(v);
						}
						queue.push_back(v.clone());
					}
				}
			}
		}
		None
	}
}
