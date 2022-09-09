//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	// collections::HashSet
};

use ahash::HashSet as HashSet;
use crate::ungraph::node::*;

use self::{method::*, path::*};

//==== DFS ====================================================================

pub struct DFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: Node<K, N, E>,
	target: Option<&'a K>,
	method: Method<'a, K, N, E>,
}

impl<'a, K, N, E> DFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &Node<K, N, E>) -> Self {
		DFS {
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

	fn recurse_adjacent(&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for Edge(u, v, e) in node.iter() {
				if self.method.exec(&Edge(u.clone(), v.clone(), e.clone())) {
					if visited.contains(v.key()) == false {
						result.push(Edge(u, v.clone(), e));
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						visited.insert(v.key().clone());
						queue.push(v.clone());
						if self.recurse_adjacent(result, visited, queue) {
							return true;
						}
					}
				}
			}
		}
		false
	}

	fn recurse_adjacent_find(&self,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		if let Some(node) = queue.pop() {
			for Edge(u, v, e) in node.iter() {
				if self.method.exec(&Edge(u.clone(), v.clone(), e.clone())) {
					if visited.contains(v.key()) == false {
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return Some(v);
						}
						visited.insert(v.key().clone());
						queue.push(v.clone());
						match self.recurse_adjacent_find(visited, queue) {
							Some(t) => return Some(t),
							None => continue,
						}
					}
				}
			}
		}
		None
	}

	pub fn search(&'a mut self) -> Option<Node<K, N, E>> {
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		return self.recurse_adjacent_find(&mut visited, &mut queue);
	}

	pub fn search_cycle(&'a mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();

		self.target = Some(self.root.key());
		queue.push(self.root.clone());

		if self.recurse_adjacent(&mut edges, &mut visited, &mut queue) {
			Some(Path::from_edge_tree(edges))
		} else {
			None
		}
	}

	pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		if self.recurse_adjacent(&mut edges, &mut visited, &mut queue) {
			Some(Path::from_edge_tree(edges))
		} else {
			None
		}
	}
}