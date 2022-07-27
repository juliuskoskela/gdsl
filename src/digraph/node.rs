use std::{
    cell::RefCell,
    fmt::Display,
    hash::Hash,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::digraph::graph_search::*;
use crate::Empty;

//==== DiNode ===================================================================

/// # Directed Graph DiNode
///
/// A node in a dirtected graph is a smart pointer containing a key,
/// a value and a set of edges. The adjacent edges are represented as two
/// vectors "inbound" and "outbound".
///
/// # Examples
///
/// TODO!
#[derive(Clone)]
pub struct DiNode<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	inner: Rc<DiNodeInner<K, N, E>>,
}

struct DiNodeInner<K, N, E>
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

	pub fn iter_undir(&self) -> DiNodeUndirIterator<K, N, E> {
		DiNodeUndirIterator { node: self, position: 0 }
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
		self.inner.upgrade().map(|inner| DiNode { inner: inner })
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

pub struct DiNodeUndirIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a DiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for DiNodeUndirIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (DiNode<K, N, E>, E);

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
pub struct DiEdge<K, N, E>
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

pub struct IterUndir<'a, K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	adjacent: &'a Adjacent<K, N, E>,
	index: usize,
}

impl<'a, K, N, E> Iterator for IterUndir<'a, K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	type Item = DiEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		let (out_len, in_len) = (self.adjacent.outbound.len(), self.adjacent.inbound.len());
		if self.index < out_len {
			self.index += 1;
			Some(self.adjacent.outbound[self.index - 1].clone())
		} else if self.index < out_len + in_len {
			self.index += 1;
			Some(self.adjacent.inbound[self.index - 1 - out_len].reverse())
		} else {
			None
		}
	}
}