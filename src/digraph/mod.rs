//! Directed Graph

//==== Submodules =============================================================

pub mod graph_search;

//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::HashMap,
	cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::digraph::graph_search::*;
use crate::Empty;

//==== DiGraph ================================================================

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
	/// use gdsl::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::new() } }

	/// Check if a node with the given key exists in the DiGraph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
			.filter(|node| node.is_root())
			.map(|node| node.clone())
			.collect()
	}

	/// Collect leaves into a vector
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::digraph::*;
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
			.filter(|node| node.is_leaf())
			.map(|node| node.clone())
			.collect()
	}

	/// Collect orpahn nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::digraph::*;
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
			.filter(|node| node.is_orphan())
			.map(|node| node.clone())
			.collect()
	}

	/// Iterate over nodes in the graph in random order
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	///
	/// g.insert(DiNode::new("A", 0));
	/// g.insert(DiNode::new("B", 0));
	/// g.insert(DiNode::new("C", 0));
	///
	/// for (key, _)in g.iter() {
	///    println!("{}", key);
	/// }
	/// ```
	pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, DiNode<K, N, E>> {
		self.nodes.iter()
	}
}

impl<'a, K, N, E> std::ops::Index<K> for DiGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = DiNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output {
		&self.nodes[&key]
	}
}

//==== DiNode ===================================================================

/// # Directed Graph DiNode
///
/// A node in a dirtected graph is a smart pointer containing a key,
/// a value and a set of edges.
#[derive(Clone)]
pub struct DiNode<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	inner: Rc<DiNodeInner<K, N, E>>,
}

pub struct DiNodeInner<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    key: K,
    value: N,
    edges: RefCell<Adjacent<K, N, E>>,
}

//==== DiNode: Implement ========================================================

impl<K, N, E> DiNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	//==== Public Methods =====================================================

	/// Create a new node with the given key and value.
    pub fn new(key: K, value: N) -> Self {
		DiNode {
			inner: Rc::new(DiNodeInner {
				key,
				value,
				edges: RefCell::new(Adjacent::new()),
			}),
		}
    }

	/// Get the key of the node.
    pub fn key(&self) -> &K {
        &self.inner.key
    }

	/// Get the value of the node.
    pub fn value(&self) -> &N {
        &self.inner.value
    }

	/// Connect two nodes.
    pub fn connect(&self, other: &DiNode<K, N, E>, value: E) {
        let edge = DiEdge::new(self, other, value);
        self.edges().borrow_mut().push_outbound(edge.clone());
        other.edges().borrow_mut().push_inbound(edge);
    }

	/// Disconnect two nodes.
    pub fn disconnect(&self, other: DiNode<K, N, E>) {
        if self.edges().borrow_mut().remove_outbound(&other) {
            other.edges().borrow_mut().remove_inbound(self);
		}
    }

	/// Disconnect the node from all of its neighbouring nodes.
	pub fn isolate(&self) {
		for edge in self.edges().borrow().iter_outbound() {
			edge.target().edges().borrow_mut().remove_inbound(self);
		}
		for edge in self.edges().borrow().iter_inbound() {
			edge.source().edges().borrow_mut().remove_outbound(self);
		}
		self.edges().borrow_mut().clear_outbound();
		self.edges().borrow_mut().clear_inbound();
	}

	/// Check if the node is a root node ie. it has no incoming edges.
	pub fn is_root(&self) -> bool {
		self.edges().borrow().inbound().is_empty()
	}

	/// Check if the node is a leaf node ie. it has no outgoing edges.
	pub fn is_leaf(&self) -> bool {
		self.edges().borrow().outbound().is_empty()
	}

	/// Check if the node is an orphan node ie. it has no incoming or outgoing
	/// edges.
	pub fn is_orphan(&self) -> bool {
		self.is_root() && self.is_leaf()
	}

	pub fn is_connected(&self, other: &DiNode<K, N, E>) -> bool {
		self.edges()
			.borrow()
			.iter_outbound()
			.find(|&edge| &edge.target() == other)
			.is_some()
	}

	pub fn dfs(&self) -> Dfs<K, N, E> {
		Dfs::new(self)
	}

	pub fn dfs_path(&self) -> DfsPath<K, N, E> {
		DfsPath::new(self)
	}

	pub fn bfs(&self) -> Bfs<K, N, E> {
		Bfs::new(self)
	}

	pub fn bfs_path(&self) -> BfsPath<K, N, E> {
		BfsPath::new(self)
	}

	pub fn pfs_min(&self) -> PfsMin<K, N, E> {
		PfsMin::new(self)
	}

	pub fn pfs_min_path(&self) -> PfsMinPath<K, N, E> {
		PfsMinPath::new(self)
	}

	pub fn pfs_max(&self) -> PfsMax<K, N, E> {
		PfsMax::new(self)
	}

	pub fn pfs_max_path(&self) -> PfsMaxPath<K, N, E> {
		PfsMaxPath::new(self)
	}

	pub fn iter_outbound(&self) -> DiNodeOutboundIterator<K, N, E> {
		DiNodeOutboundIterator { node: self, position: 0 }
	}

	pub fn iter_inbound(&self) -> DiNodeInboundIterator<K, N, E> {
		DiNodeInboundIterator { node: self, position: 0 }
	}

    //==== Private Methods ====================================================

	fn downgrade(&self) -> WeakDiNode<K, N, E> {
		WeakDiNode {
			inner: Rc::downgrade(&self.inner)
		}
	}

	pub fn edges(&self) -> &RefCell<Adjacent<K, N, E>> {
		&self.inner.edges
	}
}

//==== DiNode: Weak ===========================================================

#[derive(Clone)]
struct WeakDiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	inner: Weak<DiNodeInner<K, N, E>>,
}

impl<K, N, E> WeakDiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<DiNode<K, N, E>> {
		self.inner.upgrade().map(|inner| DiNode { inner })
	}
}

//==== DiNode: Deref ==========================================================

impl<K, N, E> Deref for DiNode<K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    type Target = N;
    fn deref(&self) -> &Self::Target {
        &self.value()
    }
}

//==== DiNode: PartialEq + Eq =================================================

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
{}

//==== DiNode: PartialOrd + Ord ===============================================

impl<K, N, E> PartialOrd for DiNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Display + Eq,
    N: Clone + Ord,
    E: Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl<K, N, E> Ord for DiNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Display + Eq,
    N: Clone + Ord,
    E: Clone,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

//==== DiNode: Iterator =======================================================

pub struct DiNodeOutboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a DiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for DiNodeOutboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (DiNode<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		let edges = self.node.inner.edges.borrow();
		let edge = edges.outbound().get(self.position);
		match edge {
			Some(edge) => {
				self.position += 1;
				Some((edge.target().clone(), edge.value.clone()))
			}
			None => None,
		}
	}
}

pub struct DiNodeInboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a DiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for DiNodeInboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (DiNode<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		let edges = self.node.inner.edges.borrow();
		let edge = edges.inbound().get(self.position);
		match edge {
			Some(edge) => {
				self.position += 1;
				Some((edge.source().clone(), edge.value.clone()))
			}
			None => None,
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a DiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (DiNode<K, N, E>, E);
	type IntoIter = DiNodeOutboundIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		DiNodeOutboundIterator { node: self, position: 0 }
	}
}

//==== DiEdge =================================================================

#[derive(Clone)]
pub struct DiEdge<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    source: WeakDiNode<K, N, E>,
    target: WeakDiNode<K, N, E>,
    value: E,
}

impl<K, N, E> DiEdge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    fn new(source: &DiNode<K, N, E>, target: &DiNode<K, N, E>, value: E) -> Self {
		Self {
			value,
			source: source.downgrade(),
			target: target.downgrade(),
		}
    }

	pub fn source(&self) -> DiNode<K, N, E> {
		self.source.upgrade().unwrap()
	}

	pub fn target(&self) -> DiNode<K, N, E> {
		self.target.upgrade().unwrap()
	}

	pub fn value(&self) -> &E {
		&self.value
	}

	pub fn reverse(&self) -> DiEdge<K, N, E> {
		DiEdge {
			source: self.target.clone(),
			target: self.source.clone(),
			value: self.value.clone(),
		}
	}
}

impl<K, N, E> Deref for DiEdge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	type Target = E;

	fn deref(&self) -> &Self::Target {
		self.value()
	}
}

//==== Adjacency List =========================================================

pub struct Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	outbound: Vec<DiEdge<K, N, E>>,
	inbound: Vec<DiEdge<K, N, E>>,
}

impl<K, N, E> Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	pub fn new() -> Self {
		Adjacent {
			outbound: Vec::new(),
			inbound: Vec::new(),
		}
	}

	pub fn outbound(&self) -> &Vec<DiEdge<K, N, E>> {
		&self.outbound
	}

	pub fn inbound(&self) -> &Vec<DiEdge<K, N, E>> {
		&self.inbound
	}

	pub fn outbound_len(&self) -> usize {
		self.outbound.len()
	}

	pub fn inbound_len(&self) -> usize {
		self.inbound.len()
	}

	pub fn push_inbound(&mut self, edge: DiEdge<K, N, E>) {
		self.inbound.push(edge);
	}

	pub fn push_outbound(&mut self, edge: DiEdge<K, N, E>) {
		self.outbound.push(edge);
	}

	pub fn remove_inbound(&mut self, source: &DiNode<K, N, E>) -> bool {
		let start_len = self.inbound.len();
		self.inbound.retain(|edge| edge.source() != *source);
		start_len != self.inbound.len()
	}

	pub fn remove_outbound(&mut self, target: &DiNode<K, N, E>) -> bool {
		let start_len = self.outbound.len();
		self.outbound.retain(|e| e.target() != *target);
		start_len != self.outbound.len()
	}

	pub fn clear_inbound(&mut self) {
		self.inbound.clear();
	}

	pub fn clear_outbound(&mut self) {
		self.outbound.clear();
	}

	pub fn iter_outbound(&self) -> std::slice::Iter<DiEdge<K, N, E>> {
		self.outbound.iter()
	}

	pub fn iter_inbound(&self) -> std::slice::Iter<DiEdge<K, N, E>> {
		self.inbound.iter()
	}
}
