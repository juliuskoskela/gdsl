//! # Node<K, N, E>
//!
//! `Node` is a key value pair smart-pointer, which includes inbound and
//! outbound connections to other nodes. Nodes can be created
//! individually and they don't depend on any graph container. They are
//! essentially smart-pointers that contain connections to other similar
//! smart pointers. For two nodes to be able to connect, they must have the
//! same type signature. Uniqueness is determined by the node's key.
//!
//! A node's type signature is <KeyType, NodeValueType, EdgeValueType>.
//!
//! - The `KeyType` is required and is used to identify the node.
//! - The `NodeValueType` is optional (supply `()` in type signature)
//!   and is used to store data associated with the node.
//! - The `EdgeValueType` is optional (supply `()` in type signature)
//!   and is used to store data associated with the edge.
//!
//! ```
//! use gdsl::digraph::*;
//!
//! type N<'a> = Node<usize, &'a str, f64>;
//!
//! let n1 = N::new(1, "Naughty Node");
//! ```
//!
//! For an inner value type to be mutable, it must be wrapped in a mutable
//! pointer such as a `Cell`, `RwLock`, or `Mutex`.
//!
//! Node's are wrapped in a reference counted smart pointer. This means
//! that a node can be cloned and shared among multiple owners.
//!
//! This node uses `Arc` for reference counting, thus it is not thread-safe.

mod method;
mod order;
mod bfs;
mod dfs;
mod pfs;
mod path;

use std::{
    fmt::Display,
    hash::Hash,
    ops::Deref,
    sync::{Arc, Weak, RwLock},
};

use self::{
	pfs::*,
	dfs::*,
	bfs::*,
	order::*,
};

// pub use self::path::Path;

//==== PUBLIC =================================================================

/// An edge between nodes is a tuple `(u, v, e)` where `u` is the
/// source node, `v` is the target node, and `e` is the edge's value.
pub type Edge<K, N, E> = (Node<K, N, E>, Node<K, N, E>, E);

/// A `Node<K, N, E>` is a key value pair smart-pointer, which includes inbound and
/// outbound connections to other nodes. Nodes can be created individually and they
/// don't depend on a graph container. Generic parameters include `K` for the node's
/// key, `N` for the node's value, and `E` for the edge's value. Two nodes are equal
/// if they have the same key.
///
/// # Example
///
/// ```
/// use gdsl::digraph::*;
///
/// let a = Node::new(0x1, "A");
/// let b = Node::new(0x2, "B");
/// let c = Node::new(0x4, "C");
///
/// a.connect(&b, 0.42);
/// a.connect(&c, 1.7);
/// b.connect(&c, 0.09);
/// c.connect(&b, 12.9);
///
/// let (u, v, e) = a.iter_out().next().unwrap();
///
/// assert!(u == a);
/// assert!(v == b);
/// assert!(e == 0.42);
/// ```
#[derive(Clone)]
pub struct Node<K = usize, N = (), E = ()>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	inner: Arc<NodeInner<K, N, E>>,
}

impl<K, N, E> Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	/// Creates a new node with a given key and value. The key is used to
	/// identify the node in the graph. Two nodes with the same key are
	/// considered equal. Value is optional, node use's `()` as default
	/// value type.
	///
	/// # Example
	///
	/// ```
	///	use gdsl::digraph::*;
	///
	///	let n1 = Node::<i32, char, ()>::new(1, 'A');
	///
	///	assert!(*n1.key() == 1);
	///	assert!(*n1.value() == 'A');
	/// ```
    pub fn new(key: K, value: N) -> Self {
		Node {
			inner: Arc::new(NodeInner {
				key,
				value,
				edges: Adjacent::new(),
			}),
		}
    }

	/// Returns a reference to the node's key.
	///
	/// # Example
	///
	/// ```
	///	use gdsl::digraph::*;
	///
	///	let n1 = Node::<i32, (), ()>::new(1, ());
	///
	///	assert!(*n1.key() == 1);
	/// ```
    pub fn key(&self) -> &K {
        &self.inner.key
    }

	/// Returns a reference to the node's value.
	///
	/// # Example
	///
	/// ```
	///	use gdsl::digraph::*;
	///
	///	let n1 = Node::<i32, char, ()>::new(1, 'A');
	///
	///	assert!(*n1.value() == 'A');
	/// ```
    pub fn value(&self) -> &N {
        &self.inner.value
    }

	/// Connects this node to another node. The connection is created in both
	/// directions. The connection is created with the given edge value and
	/// defaults to `()`. This function allows for creating multiple
	/// connections between the same nodes.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	///	let n1 = Node::new(1, ());
	///	let n2 = Node::new(2, ());
	///
	///	n1.connect(&n2, 4.20);
	///
	///	assert!(n1.is_connected(n2.key()));
	/// ```
	pub fn connect(&self, other: &Node<K, N, E>, value: E) {
	    let edge = EdgeInner::new(other, value.clone());
		let rev_edge = EdgeInner::new(&self, value);
	    self.inner.edges.push_outbound(edge);
	    other.inner.edges.push_inbound(rev_edge);
	}

	/// Connects this node to another node. The connection is created in both
	/// directions. The connection is created with the given edge value and
	/// defaults to `()`. This function doesn't allow for creating multiple
	/// connections between the same nodes. Returns Ok(()) if the connection
	/// was created, Err(EdgeValue) if the connection already exists.
	///
	/// # Example
	///
	/// ```
	///	use gdsl::digraph::*;
	///
	///	let n1 = Node::new(1, ());
	///	let n2 = Node::new(2, ());
	///
	///	match n1.try_connect(&n2, ()) {
	///		Ok(_) => assert!(n1.is_connected(n2.key())),
	///		Err(_) => panic!("n1 should be connected to n2"),
	///	}
	///
	///	match n1.try_connect(&n2, ()) {
	///		Ok(_) => panic!("n1 should be connected to n2"),
	///		Err(_) => assert!(n1.is_connected(n2.key())),
	///	}
	/// ```
	pub fn try_connect(&self, other: &Node<K, N, E>, value: E) -> Result<(), E> {
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
	///
	/// # Example
	///
	/// ```
	///	use gdsl::digraph::*;
	///
	///	let n1 = Node::new(1, ());
	///	let n2 = Node::new(2, ());
	///
	///	n1.connect(&n2, ());
	///
	///	assert!(n1.is_connected(n2.key()));
	///
	///	if n1.disconnect(n2.key()).is_err() {
	///		panic!("n1 should be connected to n2");
	///	}
	///
	///	assert!(!n1.is_connected(n2.key()));
	/// ```
	pub fn disconnect(&self, other: &K) -> Result<E, ()> {
		match self.find_outbound(other) {
			Some(other) => {
				match self.inner.edges.remove_outbound(other.key()) {
					Ok(edge) => {
						other.inner.edges.remove_inbound(self.key())?;
						Ok(edge)
					},
					Err(_) => Err(()),
				}
			},
			None => Err(()),
		}
	}

	/// Removes all inbound and outbound connections to and from the node.
	///
	/// # Example
	///
	/// ```
	///	use gdsl::digraph::*;
	///
	///	let n1 = Node::new(1, ());
	///	let n2 = Node::new(2, ());
	///	let n3 = Node::new(3, ());
	///	let n4 = Node::new(4, ());
	///
	///	n1.connect(&n2, ());
	///	n1.connect(&n3, ());
	///	n1.connect(&n4, ());
	///	n2.connect(&n1, ());
	///	n3.connect(&n1, ());
	///	n4.connect(&n1, ());
	///
	///	assert!(n1.is_connected(n2.key()));
	///	assert!(n1.is_connected(n3.key()));
	///	assert!(n1.is_connected(n4.key()));
	///
	///	n1.isolate();
	///
	///	assert!(n1.is_orphan());
	/// ```
	pub fn isolate(&self) {
		for (_, v, _) in self.iter_out() {
			v.inner.edges.remove_inbound(self.key()).unwrap();
		}
		for (v, _, _) in self.iter_in() {
			v.inner.edges.remove_outbound(self.key()).unwrap();
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
	pub fn find_outbound(&self, other: &K) -> Option<Node<K, N, E>> {
		let edge = self.inner.edges.find_outbound(other);
		if let Some(edge) = edge {
			Some(edge.target().clone())
		} else {
			None
		}
	}

	pub fn find_inbound(&self, other: &K) -> Option<Node<K, N, E>> {
		let edge = self.inner.edges.find_inbound(other);
		if let Some(edge) = edge {
			Some(edge.target().clone())
		} else {
			None
		}
	}

	/// Returns an iterator-like object that can be used to map, filter and
	/// collect reachable nodes or edges in different orderings such as
	/// postorder or preorder.
	pub fn order(&self) -> Order<K, N, E> {
		Order::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a depth-first search.
	pub fn dfs(&self) -> DFS<K, N, E> {
		DFS::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a breadth-first search.
	pub fn bfs(&self) -> BFS<K, N, E> {
		BFS::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a
	/// priotity-first search.
	pub fn pfs(&self) -> PFS<K, N, E>
	where
		N: Ord
	{
		PFS::new(self)
	}

	/// Returns an iterator over the node's outbound edges.
	pub fn iter_out(&self) -> NodeOutboundIterator<K, N, E> {
		NodeOutboundIterator { node: self, position: 0 }
	}

	/// Returns an iterator over the node's inbound edges.
	pub fn iter_in(&self) -> NodeInboundIterator<K, N, E> {
		NodeInboundIterator { node: self, position: 0 }
	}

	pub fn sizeof(&self) -> usize {
		let len_in = self.inner.edges.len_inbound();
		let len_out = self.inner.edges.len_outbound();
		let size_edges = (len_in + len_out) * std::mem::size_of::<EdgeInner<K, N, E>>();
		let size_node_inner = std::mem::size_of::<NodeInner<K, N, E>>();
		size_edges + size_node_inner + 8
	}
}

//==== TRAIT IMPLEMENTATIONS ==================================================

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

unsafe impl<K, N, E> Send for Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{}

unsafe impl<K, N, E> Sync for Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{}

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
	type Item = (Node<K, N, E>, Node<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		match self.node.inner.edges.get_outbound(self.position) {
			Some(current) => {
				self.position += 1;
				Some((
					self.node.clone(),
					current.target().clone(),
					current.value.clone()
				))
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
	type Item = (Node<K, N, E>, Node<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		match self.node.inner.edges.get_inbound(self.position) {
			Some(current) => {
				self.position += 1;
				Some((
					current.target().clone(),
					self.node.clone(),
					current.value.clone()
				))
			}
			None => None,
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (Node<K, N, E>, Node<K, N, E>, E);
	type IntoIter = NodeOutboundIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		NodeOutboundIterator { node: self, position: 0 }
	}
}

//==== PRIVATE ================================================================

struct NodeInner<K = usize, N = (), E = ()>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    key: K,
    value: N,
    edges: Adjacent<K, N, E>,
}

#[derive(Clone)]
struct WeakNode<K = usize, N = (), E = ()>
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
		self.inner.upgrade().map(|inner| Node { inner })
	}

	fn downgrade(node: &Node<K, N, E>) -> Self {
		WeakNode {
			inner: Arc::downgrade(&node.inner)
		}
	}
}

#[derive(Clone)]
struct EdgeInner<K, N = (), E = ()>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    target: WeakNode<K, N, E>,
    value: E,
}

impl<K, N, E> EdgeInner<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    fn new(target: &Node<K, N, E>, value: E) -> Self {
		Self {
			value,
			target: WeakNode::downgrade(target),
		}
    }

	fn target(&self) -> Node<K, N, E> {
		self.target.upgrade().unwrap()
	}

	fn value(&self) -> &E {
		&self.value
	}
}

impl<K, N, E> Deref for EdgeInner<K, N, E>
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

#[derive(Clone)]
struct Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	edges: Arc<RwLock<AdjacentInner<K, N, E>>>,
}

#[derive(Clone)]
struct AdjacentInner<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	outbound: Vec<EdgeInner<K, N, E>>,
	inbound: Vec<EdgeInner<K, N, E>>,
}

impl<K, N, E> Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	fn new() -> Self {
		Self {
			edges: Arc::new(RwLock::new(AdjacentInner {
				outbound: Vec::new(),
				inbound: Vec::new(),
			})),
		}
	}

	fn get_outbound(&self, idx: usize) -> Option<EdgeInner<K, N, E>> {
		let edges = self.edges.read().unwrap();
		edges.outbound.get(idx).cloned()
	}

	fn get_inbound(&self, idx: usize) -> Option<EdgeInner<K, N, E>> {
		let edges = self.edges.read().unwrap();
		edges.inbound.get(idx).cloned()
	}

	fn find_outbound(&self, node: &K) -> Option<EdgeInner<K, N, E>> {
		let edges = self.edges.read().unwrap();
		edges.outbound.iter().find(|edge| edge.target().key() == node).cloned()
	}

	fn find_inbound(&self, node: &K) -> Option<EdgeInner<K, N, E>> {
		let edges = self.edges.read().unwrap();
		edges.inbound.iter().find(|edge| edge.target().key() == node).cloned()
	}

	fn len_outbound(&self) -> usize {
		let edges = self.edges.read().unwrap();
		edges.outbound.len()
	}

	fn len_inbound(&self) -> usize {
		let edges = self.edges.read().unwrap();
		edges.inbound.len()
	}

	fn push_inbound(&self, edge: EdgeInner<K, N, E>) {
		self.edges.write().unwrap().inbound.push(edge);
	}

	fn push_outbound(&self, edge: EdgeInner<K, N, E>) {
		self.edges.write().unwrap().outbound.push(edge);
	}

	fn remove_inbound(&self, source: &K) -> Result<E, ()> {
		let mut edges = self.edges.write().unwrap();
		let idx = edges.inbound.iter().position(|edge| edge.target().key() == source);
		match idx {
			Some(idx) => {
				let edge = edges.inbound.remove(idx);
				Ok(edge.value().clone())
			},
			None => Err(()),
		}
	}

	fn remove_outbound(&self, target: &K) -> Result<E, ()> {
		let mut edges = self.edges.write().unwrap();
		let idx = edges.outbound.iter().position(|edge| edge.target().key() == target);
		match idx {
			Some(idx) => {
				let edge = edges.outbound.remove(idx);
				Ok(edge.value().clone())
			},
			None => Err(()),
		}
	}

	fn clear_inbound(&self) {
		self.edges.write().unwrap().inbound.clear();
	}

	fn clear_outbound(&self) {
		self.edges.write().unwrap().outbound.clear();
	}
}

//==== EOF ====================================================================