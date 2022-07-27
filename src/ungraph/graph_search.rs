use std::{
	fmt::Display,
    hash::Hash,
	collections::{HashSet, VecDeque}
};

use min_max_heap::MinMaxHeap;

use crate::ungraph::*;

//==== Search =================================================================

type FilterMap<'a, K, N, E> = &'a dyn Fn(&UnNode<K, N, E>, &UnNode<K, N, E>, &E) -> bool;
type Filter<'a, K, N, E> = &'a dyn Fn(&UnNode<K, N, E>, &UnNode<K, N, E>, &E) -> bool;
type Map<'a, K, N, E> = &'a dyn Fn(&UnNode<K, N, E>, &UnNode<K, N, E>, &E);

//==== Depth First Search =====================================================

pub struct Dfs<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
}

impl<K, N, E> Dfs<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		Self { root: root.clone() }
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let adjacent = node.edges().borrow();
			let edge = adjacent.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
				None => {}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Filter<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.edges().borrow();
			let edge = edge_list.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.edges().borrow();
			let edge = edge_list.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					f(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
				None => {}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: FilterMap<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.edges().borrow();
			let edge = edge_list.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
				None => {}
			}
		}
		None
	}
}

pub struct DfsPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
	edge_tree: Vec<UnEdge<K, N, E>>,
}

impl<K, N, E> DfsPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		Self {
			root: root.clone(),
			edge_tree: Vec::new(),
		}
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let adjacent = node.edges().borrow();
			let edge = adjacent.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
				None => {}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Filter<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.edges().borrow();
			let edge = edge_list.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.edges().borrow();
			let edge = edge_list.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					f(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
				None => {}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: FilterMap<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.edges().borrow();
			let edge = edge_list.iter_undir().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn edge_tree(&self) -> &Vec<UnEdge<K, N, E>> {
		&self.edge_tree
	}

	pub fn edge_path(&self) -> Vec<UnEdge<K, N, E>> {
		let mut path = Vec::new();

		let len = self.edge_tree.len() - 1;
		let w = self.edge_tree[len].clone();
		path.push(w.clone());
		let mut i = 0;
		for edge in self.edge_tree.iter().rev() {
			let source = path[i].source();
			if edge.target() == source {
				path.push(edge.clone());
				i += 1;
			}
		}
		path.reverse();
		path
	}

	pub fn node_path(&self) -> Vec<UnNode<K, N, E>> {
		let mut path = Vec::new();

		if !self.edge_path().is_empty() {
			let edge = self.edge_path()[0].clone();
			path.push(edge.source().clone());
			for edge in self.edge_path() {
				path.push(edge.target().clone());
			}
		}

		path
	}
}

//==== Breadth First Search ===================================================

pub struct Bfs<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
}

impl<K, N, E> Bfs<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		Bfs { root: root.clone() }
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push_back(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					f(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push_back(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Filter<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push_back(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: FilterMap<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push_back(edge.target().clone());
					}
				}
			}
		}
		None
	}
}

pub struct BfsPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
	edge_tree: Vec<UnEdge<K, N, E>>,
}

impl<K, N, E> BfsPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		BfsPath {
			root: root.clone(),
			edge_tree: Vec::new(),
		}
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push_back(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					f(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push_back(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Filter<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push_back(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: FilterMap<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.edges().borrow();
			for edge in edge_list.iter_undir() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push_back(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn edge_tree(&self) -> &Vec<UnEdge<K, N, E>> {
		&self.edge_tree
	}

	pub fn edge_path(&self) -> Vec<UnEdge<K, N, E>> {
		let mut path = Vec::new();

		let len = self.edge_tree.len() - 1;
		let w = self.edge_tree[len].clone();
		path.push(w.clone());
		let mut i = 0;
		for edge in self.edge_tree.iter().rev() {
			let source = path[i].source();
			if edge.target() == source {
				path.push(edge.clone());
				i += 1;
			}
		}
		path.reverse();
		path
	}

	pub fn node_path(&self) -> Vec<UnNode<K, N, E>> {
		let mut path = Vec::new();

		if !self.edge_path().is_empty() {
			let edge = self.edge_path()[0].clone();
			path.push(edge.source().clone());
			for edge in self.edge_path() {
				path.push(edge.target().clone());
			}
		}

		path
	}
}

//==== PriorityFirst Search ===================================================

pub struct PfsMin<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
}

impl<K, N, E> PfsMin<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		PfsMin { root: root.clone() }
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					map(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, map: Filter<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, map: FilterMap<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}
}

pub struct PfsMinPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
	edge_tree: Vec<UnEdge<K, N, E>>,
}

impl<K, N, E> PfsMinPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		PfsMinPath {
			root: root.clone(),
			edge_tree: Vec::new(),
		}
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					f(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Filter<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: FilterMap<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn edge_tree(&self) -> &Vec<UnEdge<K, N, E>> {
		&self.edge_tree
	}

	pub fn edge_path(&self) -> Vec<UnEdge<K, N, E>> {
		let mut path = Vec::new();

		let len = self.edge_tree.len() - 1;
		let w = self.edge_tree[len].clone();
		path.push(w.clone());
		let mut i = 0;
		for edge in self.edge_tree.iter().rev() {
			let source = path[i].source();
			if edge.target() == source {
				path.push(edge.clone());
				i += 1;
			}
		}
		path.reverse();
		path
	}

	pub fn node_path(&self) -> Vec<UnNode<K, N, E>> {
		let mut path = Vec::new();

		if !self.edge_path().is_empty() {
			let edge = self.edge_path()[0].clone();
			path.push(edge.source().clone());
			for edge in self.edge_path() {
				path.push(edge.target().clone());
			}
		}

		path
	}
}

pub struct PfsMax<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
}

impl<K, N, E> PfsMax<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		PfsMax { root: root.clone() }
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					map(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, map: Filter<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, map: FilterMap<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}
}

pub struct PfsMaxPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
	edge_tree: Vec<UnEdge<K, N, E>>,
}

impl<K, N, E> PfsMaxPath<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(root: &UnNode<K, N, E>) -> Self {
		PfsMaxPath {
			root: root.clone(),
			edge_tree: Vec::new(),
		}
	}

	pub fn search(&mut self, target: Option<&UnNode<K, N, E>>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					f(&edge.source(), &edge.target(), edge.value());
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					match target {
						Some(target) => {
							if edge.target() == *target {
								return Some(self);
							}
						},
						None => {},
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn search_filter<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: Filter<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn search_filter_map<'a>(&mut self, target: Option<&UnNode<K, N, E>>, f: FilterMap<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let adjacent = node.edges().borrow();
			for edge in adjacent.iter_outbound() {
				if !visited.contains(edge.target().key()) {
					if f(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						match target {
							Some(target) => {
								if edge.target() == *target {
									return Some(self);
								}
							},
							None => {},
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn edge_tree(&self) -> &Vec<UnEdge<K, N, E>> {
		&self.edge_tree
	}

	pub fn edge_path(&self) -> Vec<UnEdge<K, N, E>> {
		let mut path = Vec::new();

		let len = self.edge_tree.len() - 1;
		let w = self.edge_tree[len].clone();
		path.push(w.clone());
		let mut i = 0;
		for edge in self.edge_tree.iter().rev() {
			let source = path[i].source();
			if edge.target() == source {
				path.push(edge.clone());
				i += 1;
			}
		}
		path.reverse();
		path
	}

	pub fn node_path(&self) -> Vec<UnNode<K, N, E>> {
		let mut path = Vec::new();

		if !self.edge_path().is_empty() {
			let edge = self.edge_path()[0].clone();
			path.push(edge.source().clone());
			for edge in self.edge_path() {
				path.push(edge.target().clone());
			}
		}

		path
	}
}
