//! Birected graph API

use std:: {
	hash::Hash,
	ops::Deref,
	rc::{Rc, Weak},
	cell::RefCell,
	collections::{HashMap, VecDeque},
	fmt::Display,
};

use min_max_heap::MinMaxHeap;

pub type UnNodeStack<K, N, E> = Vec<UnNode<K, N, E>>;
pub type UnNodeQueue<K, N, E> = VecDeque<UnNode<K, N, E>>;
pub type UnNodePriorityQueue<K, N, E> = MinMaxHeap<UnNode<K, N, E>>;

/// # UnGraph
///
/// UnGraph container
pub struct UnGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	nodes: HashMap<K, UnNode<K, N, E>>,
}

impl<'a, K, N, E> UnGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	/// Create a new UnGraph
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<String, f64, f64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::new() } }

	/// Check if a node with the given key exists in the UnGraph
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::UnGraph;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// ```
	pub fn contains(&self, key: &K) -> bool { self.nodes.contains_key(key) }

	/// Get the length of the UnGraph (amount of nodes)
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	/// g.insert(UnNode::new("B", 0));
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
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	/// g.insert(UnNode::new("B", 0));
	/// g.insert(UnNode::new("C", 0));
	///
	/// let node = g.get(&"A").unwrap();
	///
	/// assert!(node.key() == &"A");
	/// ```
	pub fn get(&self, key: &K) -> Option<UnNode<K, N, E>> { self.nodes.get(key).map(|node| node.clone()) }

	/// Check if UnGraph is empty
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// assert!(g.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

	/// Insert a node into the UnGraph
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// assert!(g.insert(UnNode::new("A", 0)) == false);
	/// ```
	pub fn insert(&mut self, node: UnNode<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	/// Remove a node from the UnGraph
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	/// g.insert(UnNode::new("B", 0));
	///
	/// assert!(g.contains(&"A"));
	///
	/// g.remove(&"A");
	///
	/// assert!(g.contains(&"A") == false);
	/// ```
	pub fn remove(&mut self, node: &K) -> Option<UnNode<K, N, E>> {
		self.nodes.remove(node)
	}

	/// Collect nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	/// g.insert(UnNode::new("B", 0));
	/// g.insert(UnNode::new("C", 0));
	///
	/// let nodes = g.to_vec();
	///
	/// assert!(nodes.len() == 3);
	/// ```
	pub fn to_vec(&self) -> Vec<UnNode<K, N, E>> {
		self.nodes.values().map(|node| node.clone()).collect()
	}

	/// Collect orpahn nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use ::bigraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	/// g.insert(UnNode::new("B", 0));
	/// g.insert(UnNode::new("C", 0));
	/// g.insert(UnNode::new("D", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	///
	/// let orphans = g.orphans();
	///
	/// assert!(orphans.len() == 2);
	/// ```
	pub fn orphans(&self) -> Vec<UnNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty() && node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}
}

impl<'a, K, N, E> std::ops::Index<K> for UnGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = UnNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output { &self.nodes[&key]
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// Weak reference to a node.
struct WeakUnNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<UnNodeInner<K, N, E>>,
}

impl<K, N, E> WeakUnNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<UnNode<K, N, E>> {
		self.handle.upgrade().map(|handle| UnNode { handle })
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// UnNode smart pointer.
#[derive(Clone)]
pub struct UnNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<UnNodeInner<K, N, E>>,
}

struct UnNodeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	key: K,
	params: N,
	outbound: RefCell<Vec<UnEdge<K, N, E>>>,
	inbound: RefCell<Vec<WeakUnEdge<K, N, E>>>,
}

impl<K, N, E> UnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(key: K, params: N) -> Self {
		Self {
			handle: Rc::new(UnNodeInner {
				params,
				key,
				inbound: RefCell::new(vec![]),
				outbound: RefCell::new(vec![]),
			}),
		}
	}

	fn downgrade(&self) -> WeakUnNode<K, N, E> { WeakUnNode { handle: Rc::downgrade(&self.handle) } }

	pub fn key(&self) -> &K { &self.handle.key }

	pub fn params(&self) -> &N { &self.handle.params }

	pub fn outbound(&self) -> &RefCell<Vec<UnEdge<K, N, E>>> { &self.handle.outbound }

	pub fn find_outbound(&self, other: &UnNode<K, N, E>) -> Option<UnEdge<K, N, E>> {
		for edge in self.outbound().borrow().iter() {
			if &edge.target() == other {
				return Some(edge.clone());
			}
		}
		None
	}

	fn delete_outbound(&self, other: &UnNode<K, N, E>) -> bool {
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

	pub fn inbound(&self) -> &RefCell<Vec<WeakUnEdge<K, N, E>>> { &self.handle.inbound }

	pub fn find_inbound(&self, other: &UnNode<K, N, E>) -> Option<UnEdge<K, N, E>> {
		for edge in self.inbound().borrow().iter() {
			if &edge.upgrade().unwrap().source() == other {
				return Some(edge.upgrade().unwrap().clone());
			}
		}
		None
	}

	fn delete_inbound(&self, other: &UnNode<K, N, E>) -> bool {
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

	pub fn connect(&self, other: &UnNode<K, N, E>, params: E) {
		let edge = UnEdge::new(self, other.clone(), params);
		self.outbound().borrow_mut().push(edge.clone());
		other.inbound().borrow_mut().push(edge.downgrade());
	}

	pub fn try_connect(&self, other: &UnNode<K, N, E>, params: E) -> bool {
		if self.outbound().borrow().iter().any(|e| &e.target() == other) {
			return false;
		}
		self.connect(other, params);
		true
	}

	pub fn disconnect(&self, other: UnNode<K, N, E>) -> bool{
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

	pub fn search(&self) -> UnNodeSearch<K, N, E> {
		UnNodeSearch { root: self.clone(), edge_tree: vec![] }
	}
}

impl<K, N, E> Deref for UnNode<K, N, E>
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

impl<K, N, E> PartialEq for UnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn eq(&self, other: &Self) -> bool {
		self.key() == other.key()
	}
}


impl<K, N, E> Eq for UnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{ }

impl<K, N, E> PartialOrd for UnNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.params().cmp(&other.params()))
	}
}

impl<K, N, E> Ord for UnNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.params().cmp(&other.params())
	}
}

pub struct UnNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: UnNode<K, N, E>,
	position: usize,
}

impl<K, N, E> Iterator for UnNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = UnEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for UnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = UnEdge<K, N, E>;
	type IntoIter = UnNodeIntoIterator<K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		UnNodeIntoIterator { node: self, position: 0 }
	}
}

pub struct UnNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a UnNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for UnNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = UnEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a UnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = UnEdge<K, N, E>;
	type IntoIter = UnNodeIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		UnNodeIterator { node: self, position: 0 }
	}
}

///////////////////////////////////////////////////////////////////////////////
/// UnNodeSearch
///////////////////////////////////////////////////////////////////////////////

type Map<'a, K, N, E> = &'a dyn Fn(UnNode<K, N, E>, UnNode<K, N, E>, E) -> bool;

/// Search for a node in the UnGraph.
pub struct UnNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: UnNode<K, N, E>,
	edge_tree: Vec<UnEdge<K, N, E>>,
}

impl<K, N, E> UnNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn dfs(&mut self, target: &UnNode<K, N, E>) -> Option<&Self> {
		let mut queue = UnNodeStack::new();
		let mut visited = UnGraph::new();

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
			let inbound = node.inbound().borrow();
			for edge in inbound.iter() {
				let edge = edge.upgrade().unwrap();
				if visited.insert(edge.source()) {
					self.edge_tree.push(edge.clone());
					if &edge.source() == target {
						return Some(self);
					}
					queue.push(edge.source());
				}
			}
		}
		None
	}

	pub fn dfs_map<'a>(&mut self, target: &UnNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = UnNodeStack::new();
		let mut visited = UnGraph::new();

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
			let inbound = node.inbound().borrow();
			for edge in inbound.iter() {
				let edge = edge.upgrade().unwrap();
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

	pub fn bfs(&mut self, target: &UnNode<K, N, E>) -> Option<&Self> {
		let mut queue = UnNodeQueue::new();
		let mut visited = UnGraph::new();

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

	pub fn bfs_map<'a>(&mut self, target: &UnNode<K, N, E>, map: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = UnNodeQueue::new();
		let mut visited = UnGraph::new();

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

	pub fn pfs_min(&mut self, target: &UnNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = UnNodePriorityQueue::new();
		let mut visited = UnGraph::new();

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

	pub fn pfs_min_map<'a>(&mut self, target: &UnNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = UnNodePriorityQueue::new();
		let mut visited = UnGraph::new();

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

	pub fn pfs_max(&mut self, target: &UnNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = UnNodePriorityQueue::new();
		let mut visited = UnGraph::new();

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

	pub fn pfs_max_map<'a>(&mut self, target: &UnNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = UnNodePriorityQueue::new();
		let mut visited = UnGraph::new();

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
/// UnEdge between two nodes.
#[derive(Clone)]
pub struct UnEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<UnEdgeInner<K, N, E>>,
}

struct UnEdgeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	params: E,
	source: WeakUnNode<K, N, E>,
	target: UnNode<K, N, E>,
}

impl<K, N, E> UnEdge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn new(source: &UnNode<K, N, E>, target: UnNode<K, N, E>, params: E) -> Self {
		let handle = Rc::new(UnEdgeInner {
			params,
			source: source.downgrade(),
			target: target.clone(),
		});
		Self { handle }
	}

	pub fn downgrade(&self) -> WeakUnEdge<K, N, E> {
		WeakUnEdge { handle: Rc::downgrade(&self.handle) }
	}

	pub fn source(&self) -> UnNode<K, N, E> {
		self.handle.source.upgrade().unwrap().clone()
	}

	pub fn target(&self) -> UnNode<K, N, E> {
		self.handle.target.clone()
	}

	pub fn params(&self) -> &E {
		&self.handle.params
	}

	pub fn decomp(&self) -> (UnNode<K, N, E>, UnNode<K, N, E>, E) {
		(self.source(), self.target(), self.params().clone())
	}
}

impl<K, N, E> Deref for UnEdge<K, N, E>
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

pub struct WeakUnEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<UnEdgeInner<K, N, E>>,
}

impl<K, N, E> WeakUnEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<UnEdge<K, N, E>> {
		self.handle.upgrade().map(|handle| UnEdge { handle })
	}
}
