//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::HashSet
};

use crate::digraph::node::*;
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
	transpose: IO,
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
			transpose: IO::Outbound,
		}
	}

	pub fn target(mut self, target: &'a K) -> Self {
		self.target = Some(target);
		self
	}

	pub fn transpose(mut self) -> Self {
		self.transpose = IO::Inbound;
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

	fn recurse_outbound(&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for (u, v, e) in node.iter_out() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						result.push((u, v.clone(), e));
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
			for (v, u, e) in node.iter_in() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						result.push((u, v.clone(), e));
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

	pub fn find(&mut self) -> Option<Node<K, N, E>> {
		let path = self.path_nodes();
		match path {
			Some(path) => Some(path.last().unwrap().clone()),
			None => None,
		}
	}

	pub fn cycle(&'a mut self) -> Option<Vec<Node<K, N, E>>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::new();
		let target_found;

		self.target = Some(self.root.key());
		queue.push(self.root.clone());

		match self.transpose {
			IO::Outbound => {
				target_found = self.recurse_outbound(&mut edges, &mut visited, &mut queue);
			}
			IO::Inbound => {
				target_found = self.recurse_inbound(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			let edge_path = backtrack_edge_tree(edges);
			return Some(edge_path.iter().map(|(_, v, _)| v.clone()).collect());
		}
		None
	}

	pub fn path_edges(&mut self) -> Option<Vec<Edge<K, N, E>>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::new();
		let target_found;

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			IO::Outbound => {
				target_found = self.recurse_outbound(&mut edges, &mut visited, &mut queue);
			}
			IO::Inbound => {
				target_found = self.recurse_inbound(&mut edges, &mut visited, &mut queue);
			}
		}
		if target_found {
			return Some(backtrack_edge_tree(edges));
		}
		None
	}

	pub fn path_nodes(&mut self) -> Option<Vec<Node<K, N, E>>> {
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