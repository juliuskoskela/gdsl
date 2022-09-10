use std::{fmt::Display, hash::Hash};
use super::{*, method::*, path::*};
use ahash::AHashSet as HashSet;

pub struct DFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: Node<K, N, E>,
	target: Option<K>,
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

	pub fn target(mut self, target: &K) -> Self {
		self.target = Some(target.clone());
		self
	}

	pub fn for_each(mut self, f: ForEach<'a, K, N, E>) -> Self {
		self.method = Method::ForEach(f);
		self
	}

	pub fn filter(mut self, f: Filter<'a, K, N, E>) -> Self {
		self.method = Method::Filter(f);
		self
	}

	fn recurse_adjacent(&mut self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for edge in node.iter() {
				if self.method.exec(&edge) {
					let v = edge.target().clone();
					if visited.contains(v.key()) == false {
						visited.insert(v.key().clone());
						result.push(edge);
						if let Some(ref t) = self.target {
							if v.key() == t {
								return true;
							}
						}
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

	fn recurse_adjacent_find(&mut self,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		if let Some(node) = queue.pop() {
			for edge in node.iter() {
				if self.method.exec(&edge) {
					let v = edge.target();
					if visited.contains(v.key()) == false {
						visited.insert(v.key().clone());
						if let Some(ref t) = self.target {
							if v.key() == t {
								return Some(v.clone());
							}
						}
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

		self.target = Some(self.root.key().clone());
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