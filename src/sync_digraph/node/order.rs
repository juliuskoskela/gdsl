use std::{fmt::Display, hash::Hash};
use super::{*, method::*};
use ahash::AHashSet as HashSet;

pub struct Order<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: &'a Node<K, N, E>,
	method: Method<'a, K, N, E>,
	order: Ordering,
	transpose: Transposition,
}


impl<'a, K, N, E> Order<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn preorder(root: &'a Node<K, N, E>) -> Self {
		Self {
			root,
			method: Method::NullMethod,
			order: Ordering::Pre,
			transpose: Transposition::Outbound,
		}
	}

	pub fn postroder(root: &'a Node<K, N, E>) -> Self {
		Self {
			root,
			method: Method::NullMethod,
			order: Ordering::Post,
			transpose: Transposition::Outbound,
		}
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

	pub fn search_nodes(&mut self) -> Vec<Node<K, N, E>> {
		let mut nodes = vec![];
		let mut edges = vec![];
		let mut queue = vec![];
		let mut visited = HashSet::default();

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Transposition::Outbound => {
				match self.order {
					Ordering::Pre => {
						self.preorder_forward(&mut edges, &mut visited, &mut queue);
						nodes.push(self.root.clone());
						let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
					},
					Ordering::Post => {
						self.postorder_forward(&mut edges, &mut visited, &mut queue);
						let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
						nodes.push(self.root.clone());
					},
				}
			},
			Transposition::Inbound => {
				match self.order {
					Ordering::Pre => {
						self.preorder_backward(&mut edges, &mut visited, &mut queue);
						nodes.push(self.root.clone());
						let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
					},
					Ordering::Post => {
						self.postorder_backward(&mut edges, &mut visited, &mut queue);
						let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
						nodes.append(&mut coll);
						nodes.push(self.root.clone());
					},
				}
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

		match self.transpose {
			Transposition::Outbound => {
				match self.order {
					Ordering::Pre => {
						self.preorder_forward(&mut edges, &mut visited, &mut queue);
					},
					Ordering::Post => {
						self.postorder_forward(&mut edges, &mut visited, &mut queue);
					},
				}
			},
			Transposition::Inbound => {
				match self.order {
					Ordering::Pre => {
						self.preorder_backward(&mut edges, &mut visited, &mut queue);
					},
					Ordering::Post => {
						self.postorder_backward(&mut edges, &mut visited, &mut queue);
					},
				}
			},
		}
		edges
	}

	fn preorder_forward(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for Edge(u, v, e) in node.iter_out() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						result.push(Edge(u, v.clone(), e));
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
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for Edge(v, u, e) in node.iter_in() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						result.push(Edge(u, v.clone(), e));
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
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for Edge(u, v, e) in node.iter_out() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						self.postorder_forward(
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

	fn postorder_backward(
		&self,
		result: &mut Vec<Edge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut Vec<Node<K, N, E>>,
	) -> bool {
		if let Some(node) = queue.pop() {
			for Edge(v, u, e) in node.iter_in() {
				if visited.contains(v.key()) == false {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						queue.push(v.clone());
						self.postorder_forward(
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