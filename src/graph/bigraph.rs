//! Birected graph API

use crate::*;

use std:: {
	hash::Hash,
	ops::Deref,
	rc::{Rc, Weak},
	cell::RefCell,
	collections::HashMap,
	fmt::Display,
};

/// # BiGraph
///
/// BiGraph container
pub struct BiGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	nodes: HashMap<K, BiNode<K, N, E>>,
}

impl<'a, K, N, E> BiGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	/// Create a new BiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<String, f64, f64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::new() } }

	/// Check if a node with the given key exists in the BiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::BiGraph;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(BiNode::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// ```
	pub fn contains(&self, key: &K) -> bool { self.nodes.contains_key(key) }

	/// Get the length of the BiGraph (amount of nodes)
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(BiNode::new("A", 0));
	/// g.insert(BiNode::new("B", 0));
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
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(BiNode::new("A", 0));
	/// g.insert(BiNode::new("B", 0));
	/// g.insert(BiNode::new("C", 0));
	///
	/// let node = g.get(&"A").unwrap();
	///
	/// assert!(node.key() == &"A");
	/// ```
	pub fn get(&self, key: &K) -> Option<BiNode<K, N, E>> { self.nodes.get(key).map(|node| node.clone()) }

	/// Check if BiGraph is empty
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// assert!(g.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

	/// Insert a node into the BiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(BiNode::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// assert!(g.insert(BiNode::new("A", 0)) == false);
	/// ```
	pub fn insert(&mut self, node: BiNode<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	/// Remove a node from the BiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(BiNode::new("A", 0));
	/// g.insert(BiNode::new("B", 0));
	///
	/// assert!(g.contains(&"A"));
	///
	/// g.remove(&"A");
	///
	/// assert!(g.contains(&"A") == false);
	/// ```
	pub fn remove(&mut self, node: &K) -> Option<BiNode<K, N, E>> {
		self.nodes.remove(node)
	}

	/// Collect nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(BiNode::new("A", 0));
	/// g.insert(BiNode::new("B", 0));
	/// g.insert(BiNode::new("C", 0));
	///
	/// let nodes = g.to_vec();
	///
	/// assert!(nodes.len() == 3);
	/// ```
	pub fn to_vec(&self) -> Vec<BiNode<K, N, E>> {
		self.nodes.values().map(|node| node.clone()).collect()
	}

	/// Collect orpahn nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ggi::graph::bigraph::*;
	///
	/// let mut g = BiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(BiNode::new("A", 0));
	/// g.insert(BiNode::new("B", 0));
	/// g.insert(BiNode::new("C", 0));
	/// g.insert(BiNode::new("D", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	///
	/// let orphans = g.orphans();
	///
	/// assert!(orphans.len() == 2);
	/// ```
	pub fn orphans(&self) -> Vec<BiNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty() && node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}
}

impl<'a, K, N, E> std::ops::Index<K> for BiGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = BiNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output { &self.nodes[&key]
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// Weak reference to a node.
struct WeakBiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<BiNodeInner<K, N, E>>,
}

impl<K, N, E> WeakBiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<BiNode<K, N, E>> {
		self.handle.upgrade().map(|handle| BiNode { handle })
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// BiNode smart pointer.
#[derive(Clone)]
pub struct BiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<BiNodeInner<K, N, E>>,
}

struct BiNodeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	key: K,
	params: N,
	outbound: RefCell<Vec<BiEdge<K, N, E>>>,
	inbound: RefCell<Vec<WeakBiEdge<K, N, E>>>,
}

impl<K, N, E> BiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(key: K, params: N) -> Self {
		Self {
			handle: Rc::new(BiNodeInner {
				params,
				key,
				inbound: RefCell::new(vec![]),
				outbound: RefCell::new(vec![]),
			}),
		}
	}

	fn downgrade(&self) -> WeakBiNode<K, N, E> { WeakBiNode { handle: Rc::downgrade(&self.handle) } }

	pub fn key(&self) -> &K { &self.handle.key }

	pub fn params(&self) -> &N { &self.handle.params }

	pub fn outbound(&self) -> &RefCell<Vec<BiEdge<K, N, E>>> { &self.handle.outbound }

	pub fn find_outbound(&self, other: &BiNode<K, N, E>) -> Option<BiEdge<K, N, E>> {
		for edge in self.outbound().borrow().iter() {
			if &edge.target() == other {
				return Some(edge.clone());
			}
		}
		None
	}

	fn delete_outbound(&self, other: &BiNode<K, N, E>) -> bool {
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

	pub fn inbound(&self) -> &RefCell<Vec<WeakBiEdge<K, N, E>>> { &self.handle.inbound }

	pub fn find_inbound(&self, other: &BiNode<K, N, E>) -> Option<BiEdge<K, N, E>> {
		for edge in self.inbound().borrow().iter() {
			if &edge.upgrade().unwrap().source() == other {
				return Some(edge.upgrade().unwrap().clone());
			}
		}
		None
	}

	fn delete_inbound(&self, other: &BiNode<K, N, E>) -> bool {
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

	pub fn connect(&self, other: &BiNode<K, N, E>, params: E) {
		let edge = BiEdge::new(self, other.clone(), params);
		self.outbound().borrow_mut().push(edge.clone());
		other.inbound().borrow_mut().push(edge.downgrade());
	}

	pub fn try_connect(&self, other: &BiNode<K, N, E>, params: E) -> bool {
		if self.outbound().borrow().iter().any(|e| &e.target() == other) {
			return false;
		}
		self.connect(other, params);
		true
	}

	pub fn disconnect(&self, other: BiNode<K, N, E>) -> bool{
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

	pub fn search(&self) -> BiNodeSearch<K, N, E> {
		BiNodeSearch { root: self.clone(), edge_tree: vec![] }
	}
}

impl<K, N, E> Deref for BiNode<K, N, E>
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

impl<K, N, E> PartialEq for BiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn eq(&self, other: &Self) -> bool {
		self.key() == other.key()
	}
}


impl<K, N, E> Eq for BiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{ }

impl<K, N, E> PartialOrd for BiNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.params().cmp(&other.params()))
	}
}

impl<K, N, E> Ord for BiNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.params().cmp(&other.params())
	}
}

pub struct BiNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: BiNode<K, N, E>,
	position: usize,
}

impl<K, N, E> Iterator for BiNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = BiEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for BiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = BiEdge<K, N, E>;
	type IntoIter = BiNodeIntoIterator<K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		BiNodeIntoIterator { node: self, position: 0 }
	}
}

pub struct BiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a BiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for BiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = BiEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a BiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = BiEdge<K, N, E>;
	type IntoIter = BiNodeIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		BiNodeIterator { node: self, position: 0 }
	}
}

///////////////////////////////////////////////////////////////////////////////
/// BiNodeSearch
///////////////////////////////////////////////////////////////////////////////

type Map<'a, K, N, E> = &'a dyn Fn(BiNode<K, N, E>, BiNode<K, N, E>, E) -> bool;

/// Search for a node in the BiGraph.
pub struct BiNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: BiNode<K, N, E>,
	edge_tree: Vec<BiEdge<K, N, E>>,
}

impl<K, N, E> BiNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn dfs(&mut self, target: &BiNode<K, N, E>) -> Option<&Self> {
		let mut queue = BiNodeStack::new();
		let mut visited = BiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
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

	pub fn dfs_map<'a>(&mut self, target: &BiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = BiNodeStack::new();
		let mut visited = BiGraph::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
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

	pub fn bfs(&mut self, target: &BiNode<K, N, E>) -> Option<&Self> {
		let mut queue = BiNodeQueue::new();
		let mut visited = BiGraph::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
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

	pub fn bfs_map<'a>(&mut self, target: &BiNode<K, N, E>, map: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = BiNodeQueue::new();
		let mut visited = BiGraph::new();

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

	pub fn pfs_min(&mut self, target: &BiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = BiNodePriorityQueue::new();
		let mut visited = BiGraph::new();

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

	pub fn pfs_min_map<'a>(&mut self, target: &BiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = BiNodePriorityQueue::new();
		let mut visited = BiGraph::new();

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

	pub fn pfs_max(&mut self, target: &BiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = BiNodePriorityQueue::new();
		let mut visited = BiGraph::new();

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

	pub fn pfs_max_map<'a>(&mut self, target: &BiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = BiNodePriorityQueue::new();
		let mut visited = BiGraph::new();

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

	pub fn edge_path(&self) -> Vec<BiEdge<K, N, E>> {
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

	pub fn node_path(&self) -> Vec<BiNode<K, N, E>> {
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
/// BiEdge between two nodes.
#[derive(Clone)]
pub struct BiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<BiEdgeInner<K, N, E>>,
}

struct BiEdgeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	params: E,
	source: WeakBiNode<K, N, E>,
	target: BiNode<K, N, E>,
}

impl<K, N, E> BiEdge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn new(source: &BiNode<K, N, E>, target: BiNode<K, N, E>, params: E) -> Self {
		let handle = Rc::new(BiEdgeInner {
			params,
			source: source.downgrade(),
			target: target.clone(),
		});
		Self { handle }
	}

	pub fn downgrade(&self) -> WeakBiEdge<K, N, E> {
		WeakBiEdge { handle: Rc::downgrade(&self.handle) }
	}

	pub fn source(&self) -> BiNode<K, N, E> {
		self.handle.source.upgrade().unwrap().clone()
	}

	pub fn target(&self) -> BiNode<K, N, E> {
		self.handle.target.clone()
	}

	pub fn params(&self) -> &E {
		&self.handle.params
	}

	pub fn decomp(&self) -> (BiNode<K, N, E>, BiNode<K, N, E>, E) {
		(self.source(), self.target(), self.params().clone())
	}
}

impl<K, N, E> Deref for BiEdge<K, N, E>
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

pub struct WeakBiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<BiEdgeInner<K, N, E>>,
}

impl<K, N, E> WeakBiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<BiEdge<K, N, E>> {
		self.handle.upgrade().map(|handle| BiEdge { handle })
	}
}
