//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
};

use ahash::HashSet as HashSet;

use super::*;
use super::method::*;

//==== Ordering ===============================================================

pub struct Order<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: &'a Node<K, N, E>,
	method: Method<'a, K, N, E>,
	order: Ordering,
}


impl<'a, K, N, E> Order<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &'a Node<K, N, E>) -> Self {
		Self {
			root,
			method: Method::NullMethod,
			order: Ordering::Pre,
		}
	}

	pub fn pre(mut self) -> Self {
		self.order = Ordering::Pre;
		self
	}

	pub fn post(mut self) -> Self {
		self.order = Ordering::Post;
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

	pub fn search_nodes(&mut self) -> Vec<Node<K, N, E>> {
		let mut nodes = vec![];
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.order {
			Ordering::Pre => {
				self.recurse_preorder(&mut edges, &mut visited, &mut queue);
				nodes.push(self.root.clone());
				let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
				nodes.append(&mut coll);
			},
			Ordering::Post => {
				self.recurse_postorder(&mut edges, &mut visited, &mut queue);
				let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
				nodes.append(&mut coll);
				nodes.push(self.root.clone());
			},
		}
		nodes
	}

	pub fn search_edges(&mut self) -> Vec<Edge<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.order {
			Ordering::Pre => {
				self.recurse_preorder(&mut edges, &mut visited, &mut queue);
			},
			Ordering::Post => {
				self.recurse_postorder(&mut edges, &mut visited, &mut queue);
			},
		}
		edges
	}

	fn recurse_preorder(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for Edge(u, v, e) in node.iter() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&Edge(u.clone(), v.clone(), e.clone())) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						result.push(Edge(u, v.clone(), e));
						self.recurse_preorder(
							result,
							visited,
							queue);
					}
				}
			}
		}
		false
	}

	fn recurse_postorder(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for Edge(u, v, e) in node.iter() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&Edge(u.clone(), v.clone(), e.clone())) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						self.recurse_postorder(
							result,
							visited,
							queue);
						result.push(Edge(u, v.clone(), e));
					}
				}
			}
		}
		false
	}
}
