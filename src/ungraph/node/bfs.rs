//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::{HashSet, VecDeque}
};

use crate::ungraph::node::*;
use crate::ungraph::node::method::*;

pub struct BFS<'a, K, N, E>
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

	fn loop_adjacent(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut VecDeque<Node<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_front() {
			for (u, v, e) in node.iter_adjacent() {
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

	pub fn find(&mut self) -> Option<Node<K, N, E>> {
		let path = self.path_nodes();
		match path {
			Some(path) => Some(path.last().unwrap().clone()),
			None => None,
		}
	}

	fn backtrack_edge_tree(edge_tree: Vec<Edge<K, N, E>>) -> Vec<Edge<K, N, E>> {
		let mut path = Vec::new();

		let len = edge_tree.len() - 1;
		let w = edge_tree[len].clone();
		path.push(w.clone());
		let mut i = 0;
		for (u, v, e) in edge_tree.iter().rev() {
			let (s, _, _) = &path[i];
			if s == v {
				path.push((u.clone(), v.clone(), e.clone()));
				i += 1;
			}
		}
		path.reverse();
		path
	}

	pub fn cycle(&'a mut self) -> Option<Vec<Node<K, N, E>>> {
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		self.target = Some(self.root.key());
		queue.push_back(self.root.clone());

		if self.loop_adjacent(&mut edges, &mut visited, &mut queue) {
			return Some(Self::backtrack_edge_tree(edges).iter().map(|(_, v, _)| v.clone()).collect());
		}
		None
	}

	pub fn path_edges(&mut self) -> Option<Vec<Edge<K, N, E>>> {
		let mut edges = vec![];
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		visited.insert(self.root.key().clone());

		if self.loop_adjacent(&mut edges, &mut visited, &mut queue) {
			return Some(Self::backtrack_edge_tree(edges));
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