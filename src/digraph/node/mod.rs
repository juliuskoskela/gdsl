//==== Submodules =============================================================

pub mod method;
pub mod order;
pub mod bfs;
pub mod dfs;
pub mod pfs;

//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use self::{
	pfs::*,
	dfs::*,
	bfs::*,
	order::*,
};

pub use crate::{
	graph,
	connect,
	dinode,
	Empty
};

//==== Public =================================================================

/// Edge between directed nodes.
pub type DiEdge<K, N, E> = (DiNode<K, N, E>, DiNode<K, N, E>, E);

/// A directed node. Node has both outbound and inbound connections. Default
/// direction when iterating over the node's neighbours is outbound.
#[derive(Clone)]
pub struct DiNode<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	inner: Rc<DiNodeInner<K, N, E>>,
}

impl<K, N, E> DiNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	/// Creates a new node with a given key and value. The key is used to
	/// identify the node in the graph. Two nodes with the same key are
	/// considered equal. Value is optional, node use's `Empty` as default
	/// value type.
    pub fn new(key: K, value: N) -> Self {
		DiNode {
			inner: Rc::new(DiNodeInner {
				key,
				value,
				edges: Adjacent::new(),
			}),
		}
    }

	/// Returns a reference to the node's key.
    pub fn key(&self) -> &K {
        &self.inner.key
    }

	/// Returns a reference to the node's value.
    pub fn value(&self) -> &N {
        &self.inner.value
    }

	/// Connects this node to another node. The connection is created in both
	/// directions. The connection is created with the given edge value and
	/// defaults to `Empty`. This function allows for creating multiple
	/// connections between the same nodes.
    pub fn connect(&self, other: &DiNode<K, N, E>, value: E) {
        let edge = DiEdgeInner::new(self, other, value);
        self.inner.edges.push_outbound(edge.clone());
        other.inner.edges.push_inbound(edge);
    }

	/// Connects this node to another node. The connection is created in both
	/// directions. The connection is created with the given edge value and
	/// defaults to `Empty`. This function doesn't allow for creating multiple
	/// connections between the same nodes. Returns Ok(()) if the connection
	/// was created, Err(EdgeValue) if the connection already exists.
	pub fn try_connect(&self, other: &DiNode<K, N, E>, value: E) -> Result<(), E> {
		if self.is_connected(other.key()) {
			Err(value)
		} else {
			self.connect(other, value);
			Ok(())
		}
	}

	/// Disconnect two nodes from each other. The connection is removed in both
	/// directions. Returns Ok(EdgeValue) if the connection was removed, Err(())
	/// if the connection doesn't exist.
    pub fn disconnect(&self, other: &K) -> Result<E, ()> {
		if let Some(other) = self.find_outbound(other) {
			if let Ok(edge) = self.inner.edges
				.remove_outbound(other.key()) {
				if other.inner.edges
					.remove_inbound(self.key())
					.is_err() {
					panic!("This should not happen!");
				}
				Ok(edge)
			} else {
				Err(())
			}
		} else {
			Err(())
		}
	}

	/// Removes all inbound and outbound connections to and from the node.
	pub fn isolate(&self) {
		for (_, v, _) in self.iter_out() {
			if v.inner.edges
				.remove_inbound(self.key())
				.is_err() {
				panic!("This should not happen!");
			}
		}
		for (u, _, _) in self.iter_in() {
			if u.inner.edges
				.remove_outbound(self.key())
				.is_err() {
				panic!("Matching outbound connection not found!");
			}
		}
		self.inner.edges.clear_outbound();
		self.inner.edges.clear_inbound();
	}

	/// Returns true if the node is a root node. Root nodes are nodes that have
	/// no incoming connections.
	pub fn is_root(&self) -> bool {
		self.inner.edges.len_inbound() == 0
	}

	/// Returns true if the node is a leaf node. Leaf nodes are nodes that have
	/// no outgoing connections.
	pub fn is_leaf(&self) -> bool {
		self.inner.edges.len_outbound() == 0
	}

	/// Returns true if the node is an oprhan. Orphan nodes are nodes that have
	/// no connections.
	pub fn is_orphan(&self) -> bool {
		self.is_root() && self.is_leaf()
	}

	/// Returns true if the node is connected to another node with a given key.
	pub fn is_connected(&self, other: &K) -> bool {
		self.find_outbound(other).is_some()
	}

	/// Get a pointer to an adjacent node with a given key. Returns None if no
	/// node with the given key is found from the node's adjacency list.
	pub fn find_outbound(&self, other: &K) -> Option<DiNode<K, N, E>> {
		let edge = self.inner.edges.find_outbound(other);
		if let Some(edge) = edge {
			Some(edge.target().clone())
		} else {
			None
		}
	}

	pub fn find_inbound(&self, other: &K) -> Option<DiNode<K, N, E>> {
		let edge = self.inner.edges.find_inbound(other);
		if let Some(edge) = edge {
			Some(edge.source().clone())
		} else {
			None
		}
	}

	/// Returns an iterator-like object that can be used to map, filter and
	/// collect reachable nodes or edges in different orderings such as
	/// postorder or preorder.
	pub fn order(&self) -> DiOrder<K, N, E> {
		DiOrder::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a depth-first search.
	pub fn dfs(&self) -> DiDFS<K, N, E> {
		DiDFS::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a breadth-first search.
	pub fn bfs(&self) -> DiBFS<K, N, E> {
		DiBFS::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a
	/// priotity-first search.
	pub fn pfs(&self) -> DiPFS<K, N, E>
	where
		N: Ord
	{
		DiPFS::new(self)
	}

	/// Returns an iterator over the node's outbound edges.
	pub fn iter_out(&self) -> DiNodeOutboundIterator<K, N, E> {
		DiNodeOutboundIterator { node: self, position: 0 }
	}

	/// Returns an iterator over the node's inbound edges.
	pub fn iter_in(&self) -> DiNodeInboundIterator<K, N, E> {
		DiNodeInboundIterator { node: self, position: 0 }
	}
}

//==== Trait Implementations ==================================================

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
	type Item = (DiNode<K, N, E>, DiNode<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		let edge = self.node.inner.edges.get_outbound(self.position);
		match edge {
			Some(edge) => {
				self.position += 1;
				Some((edge.source().clone(), edge.target().clone(), edge.value.clone()))
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
	type Item = (DiNode<K, N, E>, DiNode<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		let edge = self.node.inner.edges.get_inbound(self.position);
		match edge {
			Some(edge) => {
				self.position += 1;
				Some((edge.source().clone(), edge.target().clone(), edge.value.clone()))
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
	type Item = (DiNode<K, N, E>, DiNode<K, N, E>, E);
	type IntoIter = DiNodeOutboundIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		DiNodeOutboundIterator { node: self, position: 0 }
	}
}

//==== Private ================================================================

//==== DiNode: Inner ===========================================================

struct DiNodeInner<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    key: K,
    value: N,
    edges: Adjacent<K, N, E>,
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

	fn downgrade(node: &DiNode<K, N, E>) -> Self {
		WeakDiNode {
			inner: Rc::downgrade(&node.inner)
		}
	}
}

//==== DiEdgeInner =================================================================

#[derive(Clone)]
struct DiEdgeInner<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    source: WeakDiNode<K, N, E>,
    target: WeakDiNode<K, N, E>,
    value: E,
}

impl<K, N, E> DiEdgeInner<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    fn new(source: &DiNode<K, N, E>, target: &DiNode<K, N, E>, value: E) -> Self {
		Self {
			value,
			source: WeakDiNode::downgrade(source),
			target: WeakDiNode::downgrade(target),
		}
    }

	fn source(&self) -> DiNode<K, N, E> {
		self.source.upgrade().unwrap()
	}

	fn target(&self) -> DiNode<K, N, E> {
		self.target.upgrade().unwrap()
	}

	fn value(&self) -> &E {
		&self.value
	}
}

impl<K, N, E> Deref for DiEdgeInner<K, N, E>
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

#[derive(Clone)]
struct Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	edges: RefCell<AdjacentInner<K, N, E>>,
}

#[derive(Clone)]
struct AdjacentInner<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	outbound: Vec<DiEdgeInner<K, N, E>>,
	inbound: Vec<DiEdgeInner<K, N, E>>,
}

impl<K, N, E> Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	fn new() -> Self {
		Self {
			edges: RefCell::new(AdjacentInner {
				outbound: Vec::new(),
				inbound: Vec::new(),
			}),
		}
	}

	fn get_outbound(&self, idx: usize) -> Option<DiEdgeInner<K, N, E>> {
		let edges = self.edges.borrow();
		edges.outbound.get(idx).cloned()
	}

	fn get_inbound(&self, idx: usize) -> Option<DiEdgeInner<K, N, E>> {
		let edges = self.edges.borrow();
		edges.inbound.get(idx).cloned()
	}

	fn find_outbound(&self, node: &K) -> Option<DiEdgeInner<K, N, E>> {
		let edges = self.edges.borrow();
		edges.outbound.iter().find(|edge| edge.target().key() == node).cloned()
	}

	fn find_inbound(&self, node: &K) -> Option<DiEdgeInner<K, N, E>> {
		let edges = self.edges.borrow();
		edges.inbound.iter().find(|edge| edge.source().key() == node).cloned()
	}

	fn len_outbound(&self) -> usize {
		let edges = self.edges.borrow();
		edges.outbound.len()
	}

	fn len_inbound(&self) -> usize {
		let edges = self.edges.borrow();
		edges.inbound.len()
	}

	fn push_inbound(&self, edge: DiEdgeInner<K, N, E>) {
		self.edges.borrow_mut().inbound.push(edge);
	}

	fn push_outbound(&self, edge: DiEdgeInner<K, N, E>) {
		self.edges.borrow_mut().outbound.push(edge);
	}

	fn remove_inbound(&self, source: &K) -> Result<E, ()> {
		let mut index = 0;
		for edge in self.edges.borrow().inbound.iter() {
			if edge.source().key() == source {
				self.edges.borrow_mut().inbound.remove(index);
				return Ok(edge.value().clone());
			}
			index += 1;
		}
		Err(())
	}

	fn remove_outbound(&self, target: &K) -> Result<E, ()> {
		let mut index = 0;
		for edge in self.edges.borrow().outbound.iter() {
			if edge.target().key() == target {
				self.edges.borrow_mut().outbound.remove(index);
				return Ok(edge.value().clone());
			}
			index += 1;
		}
		Err(())
	}

	fn clear_inbound(&self) {
		self.edges.borrow_mut().inbound.clear();
	}

	fn clear_outbound(&self) {
		self.edges.borrow_mut().outbound.clear();
	}
}

//==== EOF ====================================================================