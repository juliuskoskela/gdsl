//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::VecDeque
};

use ahash::HashSet as HashSet;

use crate::sync_ungraph::node::*;
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
		}
	}

	pub fn target(mut self, target: &'a K) -> Self {
		self.target = Some(target);
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

	fn loop_adjacent(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_front() {
			for Edge(u, v, e) in node.iter() {
				if self.method.exec(&u, &v, &e) {
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						result.push(Edge(u, v.clone(), e));
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

	fn loop_adjacent_find(
		&self,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		while let Some(node) = queue.pop_front() {
			for Edge(u, v, e) in node.iter() {
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

	pub fn search(&'a mut self) -> Option<Node<K, N, E>> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::default();

		queue.push_back(self.root.clone());
		visited.insert(self.root.key().clone());

		return self.loop_adjacent_find(&mut visited, &mut queue);
	}

	pub fn search_cycle(&'a mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::default();

		self.target = Some(self.root.key());
		queue.push_back(self.root.clone());

		if self.loop_adjacent(&mut edges, &mut visited, &mut queue) {
			Some(Path::from_edge_tree(edges))
		} else {
			None
		}
	}

	pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::default();

		queue.push_back(self.root.clone());
		visited.insert(self.root.key().clone());

		if self.loop_adjacent(&mut edges, &mut visited, &mut queue) {
			Some(Path::from_edge_tree(edges))
		} else {
			None
		}
	}
}
