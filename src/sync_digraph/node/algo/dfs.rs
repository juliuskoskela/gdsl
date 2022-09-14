use super::{method::*, path::*, *};
use ahash::AHashSet as HashSet;
use std::{fmt::Display, hash::Hash};

pub struct Dfs<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: Node<K, N, E>,
	target: Option<K>,
	method: Method<'a, K, N, E>,
	transpose: Transposition,
}

impl<'a, K, N, E> Dfs<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &Node<K, N, E>) -> Self {
		Dfs {
			root: root.clone(),
			target: None,
			method: Method::Empty,
			transpose: Transposition::Outbound,
		}
	}

	pub fn target(mut self, target: &K) -> Self {
		self.target = Some(target.clone());
		self
	}

	pub fn transpose(mut self) -> Self {
		self.transpose = Transposition::Inbound;
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

	pub fn search(&'a mut self) -> Option<Node<K, N, E>> {
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Transposition::Outbound => self.recurse_outbound_find(&mut visited, &mut queue),
			Transposition::Inbound => self.recurse_inbound_find(&mut visited, &mut queue),
		}
	}

	pub fn search_cycle(&'a mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();

		self.target = Some(self.root.key().clone());
		queue.push(self.root.clone());

		match self.transpose {
			Transposition::Outbound => {
				match self.recurse_outbound(&mut edges, &mut visited, &mut queue) {
					true => Some(Path::from_edge_tree(edges)),
					false => None,
				}
			}
			Transposition::Inbound => {
				match self.recurse_inbound(&mut edges, &mut visited, &mut queue) {
					true => Some(Path::from_edge_tree(edges)),
					false => None,
				}
			}
		}
	}

	pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Transposition::Outbound => {
				match self.recurse_outbound(&mut edges, &mut visited, &mut queue) {
					true => Some(Path::from_edge_tree(edges)),
					false => None,
				}
			}
			Transposition::Inbound => {
				match self.recurse_inbound(&mut edges, &mut visited, &mut queue) {
					true => Some(Path::from_edge_tree(edges)),
					false => None,
				}
			}
		}
	}

	fn recurse_outbound(
		&mut self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for edge in node.iter_out() {
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
						queue.push(v.clone());
						if self.recurse_outbound(result, visited, queue) {
							return true;
						}
					}
				}
			}
		}
		false
	}

	fn recurse_inbound(
		&mut self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for edge in node.iter_in() {
				let edge = edge.reverse();
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
						queue.push(v.clone());
						if self.recurse_inbound(result, visited, queue) {
							return true;
						}
					}
				}
			}
		}
		false
	}

	fn recurse_outbound_find(
		&mut self,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		if let Some(node) = queue.pop() {
			for edge in node.iter_out() {
				if self.method.exec(&edge) {
					let v = edge.target();
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						if let Some(ref t) = self.target {
							if v.key() == t {
								return Some(v.clone());
							}
						}
						queue.push(v.clone());
						match self.recurse_outbound_find(visited, queue) {
							Some(t) => return Some(t),
							None => continue,
						}
					}
				}
			}
		}
		None
	}

	fn recurse_inbound_find(
		&mut self,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		if let Some(node) = queue.pop() {
			for edge in node.iter_in() {
				let edge = edge.reverse();
				if self.method.exec(&edge) {
					let v = edge.target();
					if !visited.contains(v.key()) {
						visited.insert(v.key().clone());
						if let Some(ref t) = self.target {
							if v.key() == t {
								return Some(v.clone());
							}
						}
						queue.push(v.clone());
						match self.recurse_inbound_find(visited, queue) {
							Some(t) => return Some(t),
							None => continue,
						}
					}
				}
			}
		}
		None
	}
}
