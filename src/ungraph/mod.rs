//! # Undirected Graph

//==== Submodules =============================================================

// pub mod graph;
pub mod graph_search;
// pub mod node;

//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::HashMap
};

use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::ungraph::graph_search::*;
use crate::Empty;

//==== UnGraph ==================================================================

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
	/// use gdsl::ungraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::new() } }

	/// Check if a node with the given key exists in the UnGraph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
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
	/// use gdsl::ungraph::*;
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
	/// use gdsl::ungraph::*;
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
	/// use gdsl::ungraph::*;
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
	/// use gdsl::ungraph::*;
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
	/// use gdsl::ungraph::*;
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
	/// use gdsl::ungraph::*;
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
	/// use gdsl::ungraph::*;
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
			.filter(|node| node.is_orphan())
			.map(|node| node.clone())
			.collect()
	}

	/// Iterate over nodes in the graph in random order
	///
	/// # Examples
	///
	/// ```
	///
	/// use gdsl::ungraph::*;
	///
	/// let mut g = UnGraph::<&str, u64, u64>::new();
	///
	/// g.insert(UnNode::new("A", 0));
	/// g.insert(UnNode::new("B", 0));
	/// g.insert(UnNode::new("C", 0));
	///
	/// for (key, _)in g.iter() {
	///    println!("{}", key);
	/// }
	/// ```
	pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, UnNode<K, N, E>> {
		self.nodes.iter()
	}
}

impl<'a, K, N, E> std::ops::Index<K> for UnGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = UnNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output {
		&self.nodes[&key]
	}
}

//==== UnNode ===================================================================

/// # Directed Graph UnNode
///
/// A node in a dirtected graph is a smart pointer containing a key,
/// a value and a set of edges. The adjacent edges are represented as two
/// vectors "inbound" and "outbound".
///
/// # Examples
///
/// TODO!
#[derive(Clone)]
pub struct UnNode<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	inner: Rc<UnNodeInner<K, N, E>>,
}

struct UnNodeInner<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    key: K,
    value: N,
    edges: RefCell<Adjacent<K, N, E>>,
}

//==== UnNode: Implement ========================================================

impl<K, N, E> UnNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	//==== Public Methods =====================================================

	/// Create a new node with the given key and value.
    pub fn new(key: K, value: N) -> Self {
		UnNode {
			inner: Rc::new(UnNodeInner {
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
    pub fn connect(&self, other: &UnNode<K, N, E>, value: E) {
        let edge = UnEdge::new(self, other, value);
        self.edges().borrow_mut().push_outbound(edge.clone());
        other.edges().borrow_mut().push_inbound(edge);
    }

	/// Disconnect two nodes.
    pub fn disconnect(&self, other: UnNode<K, N, E>) {
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

	/// Check if the node is an orphan node ie. it has no incoming or outgoing
	/// edges.
	pub fn is_orphan(&self) -> bool {
		self.edges().borrow().inbound().is_empty()
		&& self.edges().borrow().outbound().is_empty()
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

    //==== Private Methods ====================================================

	fn downgrade(&self) -> WeakUnNode<K, N, E> {
		WeakUnNode {
			inner: Rc::downgrade(&self.inner)
		}
	}

	pub fn edges(&self) -> &RefCell<Adjacent<K, N, E>> {
		&self.inner.edges
	}
}

//==== UnNode: Weak ===========================================================

#[derive(Clone)]
struct WeakUnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	inner: Weak<UnNodeInner<K, N, E>>,
}

impl<K, N, E> WeakUnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<UnNode<K, N, E>> {
		self.inner.upgrade().map(|inner| UnNode { inner })
	}
}

//==== UnNode: Deref ==========================================================

impl<K, N, E> Deref for UnNode<K, N, E>
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

//==== UnNode: PartialEq + Eq =================================================

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
{}

//==== UnNode: PartialOrd + Ord ===============================================

impl<K, N, E> PartialOrd for UnNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Display + Eq,
    N: Clone + Ord,
    E: Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl<K, N, E> Ord for UnNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Display + Eq,
    N: Clone + Ord,
    E: Clone,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

//==== UnNode: Iterator =======================================================

pub struct UnNodeUndirIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a UnNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for UnNodeUndirIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (UnNode<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		let edges = self.node.edges().borrow();
		let (out_len, in_len) = (edges.outbound_len(), edges.inbound_len());

		if self.position < out_len {
			let edge = edges.outbound().get(self.position).unwrap();
			self.position += 1;
			Some((edge.target().clone(), edge.value.clone()))
		} else if self.position < out_len + in_len {
			let edge = edges.inbound().get(self.position - out_len).unwrap();
			self.position += 1;
			Some((edge.source().clone(), edge.value.clone()))
		} else {
			None
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a UnNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (UnNode<K, N, E>, E);
	type IntoIter = UnNodeUndirIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		UnNodeUndirIterator { node: self, position: 0 }
	}
}

//==== UnEdge =================================================================

#[derive(Clone)]
pub struct UnEdge<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    source: WeakUnNode<K, N, E>,
    target: WeakUnNode<K, N, E>,
    value: E,
}

impl<K, N, E> UnEdge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    fn new(source: &UnNode<K, N, E>, target: &UnNode<K, N, E>, value: E) -> Self {
		Self {
			value,
			source: source.downgrade(),
			target: target.downgrade(),
		}
    }

	pub fn source(&self) -> UnNode<K, N, E> {
		self.source.upgrade().unwrap()
	}

	pub fn target(&self) -> UnNode<K, N, E> {
		self.target.upgrade().unwrap()
	}

	pub fn value(&self) -> &E {
		&self.value
	}

	pub fn reverse(&self) -> UnEdge<K, N, E> {
		UnEdge {
			source: self.target.clone(),
			target: self.source.clone(),
			value: self.value.clone(),
		}
	}
}

impl<K, N, E> Deref for UnEdge<K, N, E>
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
	outbound: Vec<UnEdge<K, N, E>>,
	inbound: Vec<UnEdge<K, N, E>>,
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

	pub fn outbound(&self) -> &Vec<UnEdge<K, N, E>> {
		&self.outbound
	}

	pub fn inbound(&self) -> &Vec<UnEdge<K, N, E>> {
		&self.inbound
	}

	pub fn outbound_len(&self) -> usize {
		self.outbound.len()
	}

	pub fn inbound_len(&self) -> usize {
		self.inbound.len()
	}

	pub fn push_inbound(&mut self, edge: UnEdge<K, N, E>) {
		self.inbound.push(edge);
	}

	pub fn push_outbound(&mut self, edge: UnEdge<K, N, E>) {
		self.outbound.push(edge);
	}

	pub fn remove_inbound(&mut self, source: &UnNode<K, N, E>) -> bool {
		let start_len = self.inbound.len();
		self.inbound.retain(|edge| edge.source() != *source);
		start_len != self.inbound.len()
	}

	pub fn remove_outbound(&mut self, target: &UnNode<K, N, E>) -> bool {
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

	pub fn iter_outbound(&self) -> std::slice::Iter<UnEdge<K, N, E>> {
		self.outbound.iter()
	}

	pub fn iter_inbound(&self) -> std::slice::Iter<UnEdge<K, N, E>> {
		self.inbound.iter()
	}

	pub fn iter_undir(&self) -> IterUndir<K, N, E> {
		IterUndir {
			adjacent: self,
			position: 0,
		}
	}
}

pub struct IterUndir<'a, K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	adjacent: &'a Adjacent<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for IterUndir<'a, K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	type Item = UnEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		let (out_len, in_len) = (self.adjacent.outbound.len(), self.adjacent.inbound.len());
		if self.position < out_len {
			self.position += 1;
			Some(self.adjacent.outbound[self.position - 1].clone())
		} else if self.position < out_len + in_len {
			self.position += 1;
			Some(self.adjacent.inbound[self.position - 1 - out_len].reverse())
		} else {
			None
		}
	}
}
