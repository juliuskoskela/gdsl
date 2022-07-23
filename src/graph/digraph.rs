//! Directed graph API

use crate::*;

use std:: {
	hash::Hash,
	ops::Deref,
	rc::{Rc, Weak},
	cell::RefCell,
	collections::HashMap,
	fmt::Display,
};

/// # DiGraph
///
/// DiGraph container
pub struct DiGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	nodes: HashMap<K, DiNode<K, N, E>>,
}

impl<'a, K, N, E> DiGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	/// Create a new DiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<String, f64, f64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::new() } }

	/// Check if a node with the given key exists in the DiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	/// use ggi::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// ```
	pub fn contains(&self, key: &K) -> bool { self.nodes.contains_key(key) }

	/// Get the length of the DiGraph (amount of nodes)
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	///
	/// let len = g.len();
	///
	/// assert!(len == 2);
	/// ```
	pub fn len(&self) -> usize { self.nodes.len() }

	/// Get a node by key
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	/// g.insert(DiNode::new("C", 0));
	///
	/// let node = g.get(&"A").unwrap();
	///
	/// assert!(node.key() == &"A");
	/// ```
	pub fn get(&self, key: &K) -> Option<DiNode<K, N, E>> { self.nodes.get(key).map(|node| node.clone()) }

	/// Check if DiGraph is empty
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// assert!(g.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

	/// Insert a node into the DiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// assert!(g.insert(DiNode::new("A", 0)) == false);
	/// ```
	pub fn insert(&mut self, node: DiNode<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	/// Remove a node from the DiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	///
	/// assert!(g.contains(&"A"));
	///
	/// g.remove(&"A");
	///
	/// assert!(g.contains(&"A") == false);
	/// ```
	pub fn remove(&mut self, node: &K) -> Option<DiNode<K, N, E>> {
		self.nodes.remove(node)
	}

	/// Collect nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	/// g.insert(DiNode::new("C", 0));
	///
	/// let nodes = g.to_vec();
	///
	/// assert!(nodes.len() == 3);
	/// ```
	pub fn to_vec(&self) -> Vec<DiNode<K, N, E>> {
		self.nodes.values().map(|node| node.clone()).collect()
	}

	/// Collect roots into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	/// g.insert(DiNode::new("C", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	/// g["A"].connect(&g["C"], 0x1);
	/// g["B"].connect(&g["C"], 0x1);
	///
	/// let roots = g.roots();
	///
	/// assert!(roots.len() == 1);
	/// ```
	pub fn roots(&self) -> Vec<DiNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	/// Collect leaves into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	/// g.insert(DiNode::new("C", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	/// g["A"].connect(&g["C"], 0x1);
	///
	/// let leaves = g.leaves();
	///
	/// assert!(leaves.len() == 2);
	/// ```
	pub fn leaves(&self) -> Vec<DiNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	/// Collect orpahn nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	/// g.insert(DiNode::new("C", 0));
	/// g.insert(DiNode::new("D", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	///
	/// let orphans = g.orphans();
	///
	/// assert!(orphans.len() == 2);
	/// ```
	pub fn orphans(&self) -> Vec<DiNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty() && node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}
}

impl<'a, K, N, E> std::ops::Index<K> for DiGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = DiNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output { &self.nodes[&key]
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// Weak reference to a node.
struct WeakDiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<DiNodeInner<K, N, E>>,
}

impl<K, N, E> WeakDiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<DiNode<K, N, E>> {
		self.handle.upgrade().map(|handle| DiNode { handle })
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// DiNode smart pointer.
#[derive(Clone)]
pub struct DiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<DiNodeInner<K, N, E>>,
}

struct DiNodeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	key: K,
	params: N,
	outbound: RefCell<Vec<DiEdge<K, N, E>>>,
	inbound: RefCell<Vec<WeakDiEdge<K, N, E>>>,
}

impl<K, N, E> DiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(key: K, params: N) -> Self {
		Self {
			handle: Rc::new(DiNodeInner {
				params,
				key,
				inbound: RefCell::new(vec![]),
				outbound: RefCell::new(vec![]),
			}),
		}
	}

	fn downgrade(&self) -> WeakDiNode<K, N, E> { WeakDiNode { handle: Rc::downgrade(&self.handle) } }

	pub fn key(&self) -> &K { &self.handle.key }

	pub fn params(&self) -> &N { &self.handle.params }

	pub fn outbound(&self) -> &RefCell<Vec<DiEdge<K, N, E>>> { &self.handle.outbound }

	pub fn find_outbound(&self, other: &DiNode<K, N, E>) -> Option<DiEdge<K, N, E>> {
		for edge in self.outbound().borrow().iter() {
			if &edge.target() == other {
				return Some(edge.clone());
			}
		}
		None
	}

	fn delete_outbound(&self, other: &DiNode<K, N, E>) -> bool {
		let mut outbound = self.outbound().borrow_mut();
		let (mut idx, mut found) = (0, false);
		for (i, edge) in outbound.iter().enumerate() {
			if &edge.target() == other {
				idx = i;
				found = true;
			}
		}
		if found {
			outbound.remove(idx);
		}
		found
	}

	pub fn inbound(&self) -> &RefCell<Vec<WeakDiEdge<K, N, E>>> { &self.handle.inbound }

	pub fn find_inbound(&self, other: &DiNode<K, N, E>) -> Option<DiEdge<K, N, E>> {
		for edge in self.inbound().borrow().iter() {
			if &edge.upgrade().unwrap().source() == other {
				return Some(edge.upgrade().unwrap().clone());
			}
		}
		None
	}

	fn delete_inbound(&self, other: &DiNode<K, N, E>) -> bool {
		let mut inbound = self.inbound().borrow_mut();
		let (mut idx, mut found) = (0, false);
		for (i, edge) in inbound.iter().enumerate() {
			if &edge.upgrade().unwrap().source() == other {
				idx = i;
				found = true;
			}
		}
		if found {
			inbound.remove(idx);
		}
		found
	}

	pub fn connect(&self, other: &DiNode<K, N, E>, params: E) {
		let edge = DiEdge::new(self, other.clone(), params);
		self.outbound().borrow_mut().push(edge.clone());
		other.inbound().borrow_mut().push(edge.downgrade());
	}

	pub fn try_connect(&self, other: &DiNode<K, N, E>, params: E) -> bool {
		if self.outbound().borrow().iter().any(|e| &e.target() == other) {
			return false;
		}
		self.connect(other, params);
		true
	}

	pub fn disconnect(&self, other: DiNode<K, N, E>) -> bool{
		if other.delete_inbound(self) {
			self.delete_outbound(&other)
		} else {
			false
		}
	}

	pub fn isolate(&self) {
		for edge in self.inbound().borrow().iter() {
			let target = edge.upgrade().unwrap().target();
			target.delete_outbound(self);
		}

		for edge in self.outbound().borrow().iter() {
			let target = edge.target();
			target.delete_inbound(self);
		}

		self.outbound().borrow_mut().clear();
		self.inbound().borrow_mut().clear();
	}

	pub fn search(&self) -> DiNodeSearch<K, N, E> {
		DiNodeSearch { root: self.clone(), edge_tree: vec![] }
	}
}

impl<K, N, E> Deref for DiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Target = N;

	fn deref(&self) -> &Self::Target {
		&self.params()
	}
}

impl<K, N, E> PartialEq for DiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn eq(&self, other: &Self) -> bool {
		self.key() == other.key()
	}
}


impl<K, N, E> Eq for DiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{ }

impl<K, N, E> PartialOrd for DiNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.params().cmp(&other.params()))
	}
}

impl<K, N, E> Ord for DiNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.params().cmp(&other.params())
	}
}

pub struct DiNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: DiNode<K, N, E>,
	position: usize,
}

impl<K, N, E> Iterator for DiNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = DiEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for DiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = DiEdge<K, N, E>;
	type IntoIter = DiNodeIntoIterator<K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		DiNodeIntoIterator { node: self, position: 0 }
	}
}

pub struct DiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a DiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for DiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = DiEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a DiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = DiEdge<K, N, E>;
	type IntoIter = DiNodeIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		DiNodeIterator { node: self, position: 0 }
	}
}

///////////////////////////////////////////////////////////////////////////////
/// DiNodeSearch
///////////////////////////////////////////////////////////////////////////////

type Map<'a, K, N, E> = &'a dyn Fn(DiNode<K, N, E>, DiNode<K, N, E>, E) -> bool;

/// Search for a node in the DiGraph.
pub struct DiNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: DiNode<K, N, E>,
	edge_tree: Vec<DiEdge<K, N, E>>,
}

impl<K, N, E> DiNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn dfs(&mut self, target: &DiNode<K, N, E>) -> Option<&Self> {
		let mut queue = DiNodeStack::new();
		let mut visited = DiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let iterator = node.outbound().borrow();
			let edge = iterator.iter().find(|e| visited.insert(e.target()) == true);
			match edge {
				Some(edge) => {
					self.edge_tree.push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push(edge.target().clone());
				}
				None => {}
			}
		}
		None
	}

	pub fn dfs_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = DiNodeStack::new();
		let mut visited = DiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let iterator = node.outbound().borrow();
			let edge = iterator.iter().find(|e| visited.insert(e.target()) == true);
			match edge {
				Some(edge) => {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree.push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push(edge.target());
					} else {
						visited.remove(edge.target().key());
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn bfs(&mut self, target: &DiNode<K, N, E>) -> Option<&Self> {
		let mut queue = DiNodeQueue::new();
		let mut visited = DiGraph::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			println!("popped: {}", node.key());
			for edge in &node {
				if visited.insert(edge.target()) {
					self.edge_tree.push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push_back(edge.target());
				}
			}
		}
		None
	}

	pub fn bfs_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = DiNodeQueue::new();
		let mut visited = DiGraph::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			for edge in &node {
				if visited.insert(edge.target()) {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree.push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push_back(edge.target());
					} else {
						visited.remove(edge.target().key());
					}
				}
			}
		}
		None
	}

	pub fn pfs_min(&mut self, target: &DiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = DiNodePriorityQueue::new();
		let mut visited = DiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			for edge in &node {
				if visited.insert(edge.target()) {
					self.edge_tree.push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push(edge.target());
				}
			}
		}
		None
	}

	pub fn pfs_min_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = DiNodePriorityQueue::new();
		let mut visited = DiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			for edge in &node {
				if visited.insert(edge.target()) {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree.push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push(edge.target());
					} else {
						visited.remove(edge.target().key());
					}
				}
			}
		}
		None
	}

	pub fn pfs_max(&mut self, target: &DiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = DiNodePriorityQueue::new();
		let mut visited = DiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			for edge in &node {
				if visited.insert(edge.target()) {
					self.edge_tree.push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push(edge.target());
				}
			}
		}
		None
	}

	pub fn pfs_max_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = DiNodePriorityQueue::new();
		let mut visited = DiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			for edge in &node {
				if visited.insert(edge.target()) {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree.push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push(edge.target());
					} else {
						visited.remove(edge.target().key());
					}
				}
			}
		}
		None
	}

	pub fn edge_path(&self) -> Vec<DiEdge<K, N, E>> {
		let mut path = Vec::new();

		let len = self.edge_tree.len() - 1;
		let w = self.edge_tree[len].clone();
		path.push(w.clone());
		let mut i = 0;
		for edge in self.edge_tree.iter().rev() {
			// print!("backtracking: {} => {}", edge.target().key(), edge.source().key());
			let source = path[i].source();
			if edge.target() == source {
				// print!(" *");
				path.push(edge.clone());
				i += 1;
			}
			// println!("");
		}
		path.reverse();
		path
	}

	pub fn node_path(&self) -> Vec<DiNode<K, N, E>> {
		if self.edge_path().len() == 0 {
			return Vec::new();
		}
		let mut path = Vec::new();
		path.push(self.edge_path()[0].source());
		for edge in self.edge_path() {
			path.push(edge.target());
		}
		path
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// DiEdge between two nodes.
#[derive(Clone)]
pub struct DiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<DiEdgeInner<K, N, E>>,
}

struct DiEdgeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	params: E,
	source: WeakDiNode<K, N, E>,
	target: DiNode<K, N, E>,
}

impl<K, N, E> DiEdge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn new(source: &DiNode<K, N, E>, target: DiNode<K, N, E>, params: E) -> Self {
		let handle = Rc::new(DiEdgeInner {
			params,
			source: source.downgrade(),
			target: target.clone(),
		});
		Self { handle }
	}

	pub fn downgrade(&self) -> WeakDiEdge<K, N, E> {
		WeakDiEdge { handle: Rc::downgrade(&self.handle) }
	}

	pub fn source(&self) -> DiNode<K, N, E> {
		self.handle.source.upgrade().unwrap().clone()
	}

	pub fn target(&self) -> DiNode<K, N, E> {
		self.handle.target.clone()
	}

	pub fn params(&self) -> &E {
		&self.handle.params
	}

	pub fn decomp(&self) -> (DiNode<K, N, E>, DiNode<K, N, E>, E) {
		(self.source(), self.target(), self.params().clone())
	}
}

impl<K, N, E> Deref for DiEdge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Target = E;

	fn deref(&self) -> &Self::Target {
		&self.params()
	}
}

pub struct WeakDiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<DiEdgeInner<K, N, E>>,
}

impl<K, N, E> WeakDiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<DiEdge<K, N, E>> {
		self.handle.upgrade().map(|handle| DiEdge { handle })
	}
}
