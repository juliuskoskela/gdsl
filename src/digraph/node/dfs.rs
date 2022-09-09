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
	target: Option<&'a K>,
	method: Method<'a, K, N, E>,
	transpose: Transposition,
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
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Transposition::Outbound => {
				return self.recurse_outbound_find(&mut visited, &mut queue);
			}
			Transposition::Inbound => {
				return self.recurse_inbound_find(&mut visited, &mut queue);
			}
		}
	}

	pub fn search_cycle(&'a mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();
		let target_found;

		self.target = Some(self.root.key());
		queue.push(self.root.clone());

		match self.transpose {
			Transposition::Outbound => {
				target_found = self.recurse_outbound(&mut edges, &mut visited, &mut queue);
			}
			Transposition::Inbound => {
				target_found = self.recurse_inbound(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			return Some(Path::from_edge_tree(edges));
		}
		None
	}

	pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();
		let target_found;

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Transposition::Outbound => {
				target_found = self.recurse_outbound(&mut edges, &mut visited, &mut queue);
			}
			Transposition::Inbound => {
				target_found = self.recurse_inbound(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			return Some(Path::from_edge_tree(edges));
		}
		None
	}

	fn recurse_outbound(&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for edge in node.iter_out() {
				let v = edge.1.clone();
				if self.method.exec(&edge) {
					if visited.contains(v.key()) == false {
						result.push(edge);
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						visited.insert(v.key().clone());
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

	fn recurse_inbound(&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for edge in node.iter_in() {
				let edge = edge.reverse();
				let v = edge.1.clone();
				if self.method.exec(&edge) {
					if visited.contains(v.key()) == false {
						result.push(edge);
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						visited.insert(v.key().clone());
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

	fn recurse_outbound_find(&self,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		if let Some(node) = queue.pop() {
			for edge in node.iter_out() {
				let v = edge.1.clone();
				if self.method.exec(&edge) {
					if visited.contains(v.key()) == false {
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return Some(v);
						}
						visited.insert(v.key().clone());
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

	fn recurse_inbound_find(&self,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> Option<Node<K, N, E>> {
		if let Some(node) = queue.pop() {
			for edge in node.iter_in() {
				let edge = edge.reverse();
				let v = edge.1.clone();
				if self.method.exec(&edge) {
					if visited.contains(v.key()) == false {
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return Some(v);
						}
						visited.insert(v.key().clone());
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