use std::{fmt::Display, hash::Hash, collections::VecDeque};
use super::{*, method::*, path::*};
use ahash::AHashSet as HashSet;

pub struct BFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: Node<K, N, E>,
	target: Option<K>,
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
			method: Method::Empty,
		}
	}

	pub fn target(mut self, target: &'a K) -> Self {
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

	fn loop_adjacent(
		&mut self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_front() {
			for edge in node.iter() {
				if self.method.exec(&edge) {
					let v = edge.target().clone();
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						result.push(edge);
						if let Some(ref t) = self.target {
							if v.key() == t {
								return true;
							}
						}
						queue.push_back(v.clone());
					}
				}
			}
		}
		false
	}

	fn loop_adjacent_find(
		&mut self,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		while let Some(node) = queue.pop_front() {
			for edge in node.iter() {
				if self.method.exec(&edge) {
					let v = edge.target();
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						if let Some(ref t) = self.target {
							if v.key() == t {
								return Some(v.clone());
							}
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

		self.target = Some(self.root.key().clone());
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
