//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::HashSet,
};

use crate::digraph::*;
use crate::digraph::node::*;
use crate::digraph::node::method::*;

//==== Ordering ===============================================================

pub struct DirectedOrdering<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: &'a DiNode<K, N, E>,
	method: Method<'a, K, N, E>,
	order: Order,
	transpose: Direction,
}


impl<'a, K, N, E> DirectedOrdering<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &'a DiNode<K, N, E>) -> Self {
		Self {
			root,
			method: Method::NullMethod,
			order: Order::Pre,
			transpose: Direction::Forward,
		}
	}

	pub fn pre(mut self) -> Self {
		self.order = Order::Pre;
		self
	}

	pub fn post(mut self) -> Self {
		self.order = Order::Post;
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

	pub fn collect_nodes(&mut self) -> Vec<DiNode<K, N, E>> {
		let mut nodes = vec![];
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Direction::Forward => {
				match self.order {
					Order::Pre => {
						self.preorder_forward(&mut edges, &mut visited, &mut queue);
						nodes.push(self.root.clone());
						let mut coll = edges.iter().map(|(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
					},
					Order::Post => {
						self.postorder_forward(&mut edges, &mut visited, &mut queue);
						let mut coll = edges.iter().map(|(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
						nodes.push(self.root.clone());
					},
				}
			},
			Direction::Backward => {
				match self.order {
					Order::Pre => {
						self.preorder_backward(&mut edges, &mut visited, &mut queue);
						nodes.push(self.root.clone());
						let mut coll = edges.iter().map(|(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
					},
					Order::Post => {
						self.postorder_backward(&mut edges, &mut visited, &mut queue);
						let mut coll = edges.iter().map(|(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
						nodes.push(self.root.clone());
					},
				}
			},
		}
		nodes
	}

	pub fn collect_edges(&mut self) -> Vec<DiEdge<K, N, E>> {
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Direction::Forward => {
				match self.order {
					Order::Pre => {
						self.preorder_forward(&mut edges, &mut visited, &mut queue);
					},
					Order::Post => {
						self.postorder_forward(&mut edges, &mut visited, &mut queue);
					},
				}
			},
			Direction::Backward => {
				match self.order {
					Order::Pre => {
						self.preorder_backward(&mut edges, &mut visited, &mut queue);
					},
					Order::Post => {
						self.postorder_backward(&mut edges, &mut visited, &mut queue);
					},
				}
			},
		}
		edges
	}

	fn preorder_forward(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<DiNode<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for (u, v, e) in node.iter_outbound() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						result.push((u, v.clone(), e));
						self.preorder_forward(
							result,
							visited,
							queue);
					}
				}
			}
		}
		false
	}

	fn preorder_backward(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<DiNode<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for (v, u, e) in node.iter_inbound() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						result.push((u, v.clone(), e));
						self.preorder_forward(
							result,
							visited,
							queue);
					}
				}
			}
		}
		false
	}

	fn postorder_forward(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<DiNode<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for (u, v, e) in node.iter_outbound() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						self.postorder_forward(
							result,
							visited,
							queue);
						result.push((u, v.clone(), e));
					}
				}
			}
		}
		false
	}

	fn postorder_backward(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<DiNode<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for (v, u, e) in node.iter_inbound() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						self.postorder_forward(
							result,
							visited,
							queue);
						result.push((u, v.clone(), e));
					}
				}
			}
		}
		false
	}
}
