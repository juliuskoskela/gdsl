use std::{
    cell::RefCell,
    fmt::Display,
    hash::Hash,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::graph_search::*;

//==== Node ===================================================================

/// # Directed Graph Node
///
/// A node in a dirtected graph is a smart pointer containing a key,
/// a value and a set of edges. The adjacent edges are represented as two
/// vectors "inbound" and "outbound".
///
/// # Examples
///
/// TODO!
#[derive(Clone)]
pub struct Node<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	inner: Rc<NodeInner<K, N, E>>,
}

struct NodeInner<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    key: K,
    value: N,
    edges: RefCell<Adjacent<K, N, E>>,
}

//==== Node: Implement ========================================================

impl<K, N, E> Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	//==== Public Methods =====================================================

	/// Create a new node with the given key and value.
    pub fn new(key: K, value: N) -> Self {
		Node {
			inner: Rc::new(NodeInner {
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
    pub fn connect(&self, other: &Node<K, N, E>, value: E) {
        let edge = Edge::new(self, other, value);
        self.edges().borrow_mut().push_outbound(edge.clone());
        other.edges().borrow_mut().push_inbound(edge);
    }

	/// Disconnect two nodes.
    pub fn disconnect(&self, other: Node<K, N, E>) {
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

	pub fn iter_outbound(&self) -> NodeOutboundIterator<K, N, E> {
		NodeOutboundIterator { node: self, position: 0 }
	}

	pub fn iter_inbound(&self) -> NodeInboundIterator<K, N, E> {
		NodeInboundIterator { node: self, position: 0 }
	}

	pub fn iter_undir(&self) -> NodeUndirIterator<K, N, E> {
		NodeUndirIterator { node: self, position: 0 }
	}

    //==== Private Methods ====================================================

	fn downgrade(&self) -> WeakNode<K, N, E> {
		WeakNode {
			inner: Rc::downgrade(&self.inner)
		}
	}

	pub fn edges(&self) -> &RefCell<Adjacent<K, N, E>> {
		&self.inner.edges
	}
}

//==== Node: Weak ===========================================================

#[derive(Clone)]
struct WeakNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	inner: Weak<NodeInner<K, N, E>>,
}

impl<K, N, E> WeakNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<Node<K, N, E>> {
		self.inner.upgrade().map(|inner| Node { inner: inner })
	}
}

//==== Node: Deref ==========================================================

impl<K, N, E> Deref for Node<K, N, E>
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

//==== Node: PartialEq + Eq =================================================

impl<K, N, E> PartialEq for Node<K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<K, N, E> Eq for Node<K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{}

//==== Node: PartialOrd + Ord ===============================================

impl<K, N, E> PartialOrd for Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Display + Eq,
    N: Clone + Ord,
    E: Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl<K, N, E> Ord for Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Display + Eq,
    N: Clone + Ord,
    E: Clone,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

//==== Node: Iterator =======================================================

pub struct NodeOutboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a Node<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for NodeOutboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (Node<K, N, E>, E);

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

pub struct NodeInboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a Node<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for NodeInboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (Node<K, N, E>, E);

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

pub struct NodeUndirIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a Node<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for NodeUndirIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (Node<K, N, E>, E);

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

impl<'a, K, N, E> IntoIterator for &'a Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (Node<K, N, E>, E);
	type IntoIter = NodeOutboundIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		NodeOutboundIterator { node: self, position: 0 }
	}
}

//==== Edge =================================================================

#[derive(Clone)]
pub struct Edge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    source: WeakNode<K, N, E>,
    target: WeakNode<K, N, E>,
    value: E,
}

impl<K, N, E> Edge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    fn new(source: &Node<K, N, E>, target: &Node<K, N, E>, value: E) -> Self {
		Self {
			value,
			source: source.downgrade(),
			target: target.downgrade(),
		}
    }

	pub fn source(&self) -> Node<K, N, E> {
		self.source.upgrade().unwrap()
	}

	pub fn target(&self) -> Node<K, N, E> {
		self.target.upgrade().unwrap()
	}

	pub fn value(&self) -> &E {
		&self.value
	}

	pub fn reverse(&self) -> Edge<K, N, E> {
		Edge {
			source: self.target.clone(),
			target: self.source.clone(),
			value: self.value.clone(),
		}
	}
}

impl<K, N, E> Deref for Edge<K, N, E>
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
	outbound: Vec<Edge<K, N, E>>,
	inbound: Vec<Edge<K, N, E>>,
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

	pub fn outbound(&self) -> &Vec<Edge<K, N, E>> {
		&self.outbound
	}

	pub fn inbound(&self) -> &Vec<Edge<K, N, E>> {
		&self.inbound
	}

	pub fn outbound_len(&self) -> usize {
		self.outbound.len()
	}

	pub fn inbound_len(&self) -> usize {
		self.inbound.len()
	}

	pub fn push_inbound(&mut self, edge: Edge<K, N, E>) {
		self.inbound.push(edge);
	}

	pub fn push_outbound(&mut self, edge: Edge<K, N, E>) {
		self.outbound.push(edge);
	}

	pub fn remove_inbound(&mut self, source: &Node<K, N, E>) -> bool {
		let start_len = self.inbound.len();
		self.inbound.retain(|edge| edge.source() != *source);
		start_len != self.inbound.len()
	}

	pub fn remove_outbound(&mut self, target: &Node<K, N, E>) -> bool {
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

	pub fn iter_outbound(&self) -> std::slice::Iter<Edge<K, N, E>> {
		self.outbound.iter()
	}

	pub fn iter_inbound(&self) -> std::slice::Iter<Edge<K, N, E>> {
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
	type Item = Edge<K, N, E>;

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