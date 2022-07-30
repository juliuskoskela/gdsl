//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::{HashSet, VecDeque}
};

use crate::digraph::*;
use crate::digraph::node::method::*;

pub struct BFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: DiNode<K, N, E>,
	target: Option<&'a K>,
	method: Method<'a, K, N, E>,
	transpose: Direction,
}

impl<'a, K, N, E> BFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &DiNode<K, N, E>) -> Self {
		BFS {
			root: root.clone(),
			target: None,
			method: Method::NullMethod,
			transpose: Direction::Forward,
		}
	}

	pub fn target(mut self, target: &'a K) -> Self {
		self.target = Some(target);
		self
	}

	pub fn transpose(mut self) -> Self {
		self.transpose = Direction::Backward;
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

	fn forward(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<DiNode<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_front() {
			for (u, v, e) in node.iter_outbound() {
				if !visited.contains(v.key()) {
					if self.method.exec(&u, &v, &e) {
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

	fn backward(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<DiNode<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_front() {
			for (v, u, e) in node.iter_outbound() {
				if !visited.contains(v.key()) {
					if self.method.exec(&u, &v, &e) {
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

	pub fn find(&mut self) -> Option<DiNode<K, N, E>> {
		let path = self.path_nodes();
		match path {
			Some(path) => Some(path.last().unwrap().clone()),
			None => None,
		}
	}

	pub fn cycle(&'a mut self) -> Option<Vec<DiNode<K, N, E>>> {
		let mut result = vec![];
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();
		let target_found;

		self.target = Some(self.root.key());
		queue.push_back(self.root.clone());

		match self.transpose {
			Direction::Forward => {
				target_found = self.forward(&mut edges, &mut visited, &mut queue);
			}
			Direction::Backward => {
				target_found = self.backward(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			let mut nodes = edges.iter().map(|(_, v, _)| v.clone()).collect();
			result.append(&mut nodes);
			return Some(result);
		}
		None
	}

	pub fn path_edges(&mut self) -> Option<Vec<DiEdge<K, N, E>>> {
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();
		let target_found;

		queue.push_back(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Direction::Forward => {
				target_found = self.forward(&mut edges, &mut visited, &mut queue);
			}
			Direction::Backward => {
				target_found = self.backward(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			return Some(edges);
		}
		None
	}

	pub fn path_nodes(&mut self) -> Option<Vec<DiNode<K, N, E>>> {
		let edges = self.path_edges();
		match edges {
			Some(edges) => {
				let mut nodes = vec![self.root.clone()];
				for (_, v, _) in edges {
					nodes.push(v.clone());
				}
				return Some(nodes);
			}
			None => {
				return None;
			}
		}
	}
}