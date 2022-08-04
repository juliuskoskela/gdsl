use std::{
    fmt::Display,
    hash::Hash, ops::Index,
};

use crate::digraph::node::*;

pub fn backtrack_edge_tree<K, N, E>(edge_tree: Vec<Edge<K, N, E>>) -> Vec<Edge<K, N, E>>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	let mut path = Vec::new();

	if edge_tree.len() == 1 {
		path.push(edge_tree[0].clone());
		return path;
	}
	let w = edge_tree.last().unwrap();
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

pub struct Path<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub edges: Vec<Edge<K, N, E>>,
}

impl<K, N, E> Path<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn from_edge_tree(edge_tree: Vec<Edge<K, N, E>>) -> Path<K, N, E> {
		Path { edges: backtrack_edge_tree(edge_tree) }
	}

	pub fn iter_nodes(&self) -> PathNodeIterator<K, N, E> {
		PathNodeIterator {
			path: self.clone(),
			position: 0,
		}
	}

	pub fn iter_edges(&self) -> PathEdgeIterator<K, N, E> {
		PathEdgeIterator {
			path: self.clone(),
			position: 0,
		}
	}

	pub fn to_vec_nodes(&self) -> Vec<Node<K, N, E>> {
		self.iter_nodes().map(|v| v).collect()
	}

	pub fn to_vec_edges(&self) -> Vec<Edge<K, N, E>> {
		self.edges.clone()
	}
}

impl<K, N, E> Index<usize> for Path<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Output = Edge<K, N, E>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.edges[index]
	}
}

pub struct PathEdgeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	path: &'a Path<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for PathEdgeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (Node<K, N, E>, Node<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		match self.path.edges.get(self.position) {
			Some(edge) => {
				self.position += 1;
				Some((edge.0.clone(), edge.1.clone(), edge.2.clone()))
			}
			None => None,
		}
	}
}

pub struct PathNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	path: &'a Path<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for PathNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = Node<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position == 0 {
			match self.path.edges.get(self.position) {
				Some(edge) => {
					self.position += 1;
					return Some(edge.0.clone());
				}
				None => return None,
			}
		}
		match self.path.edges.get(self.position - 1) {
			Some(edge) => {
				self.position += 1;
				Some(edge.1.clone())
			}
			None => None,
		}
	}
}
