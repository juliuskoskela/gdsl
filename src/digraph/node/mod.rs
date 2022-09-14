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
//! pointer such as a `Cell`, `RefCell`, or `Mutex`.
//!
//! Node's are wrapped in a reference counted smart pointer. This means
//! that a node can be cloned and shared among multiple owners.
//!
//! This node uses `Rc` for reference counting, thus it is not thread-safe.

mod adjacent;
mod algo;

use self::{
	adjacent::*,
	algo::{bfs::*, dfs::*, order::*, pfs::*},
};
use std::{
	cell::RefCell,
	fmt::Display,
	hash::Hash,
	ops::Deref,
	rc::{Rc, Weak},
};

enum Transposition {
	Outbound,
	Inbound,
}

/// An edge between nodes is a tuple struct `Edge(u, v, e)` where `u` is the
/// source node, `v` is the target node, and `e` is the edge's value.
#[derive(Clone, PartialEq)]
pub struct Edge<K, N, E>(pub Node<K, N, E>, pub Node<K, N, E>, pub E)
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone;

impl<K, N, E> Edge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	/// Returns the source node of the edge.
	pub fn source(&self) -> &Node<K, N, E> {
		&self.0
	}

	/// Returns the target node of the edge.
	pub fn target(&self) -> &Node<K, N, E> {
		&self.1
	}

	/// Returns the edge's value.
	pub fn value(&self) -> &E {
		&self.2
	}

	/// Reverse the edge's direction.
	pub fn reverse(&self) -> Edge<K, N, E> {
		Edge(self.1.clone(), self.0.clone(), self.2.clone())
	}
}

/// A `Node<K, N, E>` is a key value pair smart-pointer, which includes inbound
/// and outbound connections to other nodes. Nodes can be created individually
/// and they don't depend on a graph container. Generic parameters include `K`
/// for the node's key, `N` for the node's value, and `E` for the edge's
/// value. Two nodes are equal if they have the same key.
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
/// let Edge(u, v, e) = a.iter_out().next().unwrap();
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
	inner: Rc<(K, N, RefCell<Adjacent<K, N, E>>)>,
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
			inner: Rc::new((key, value, Adjacent::new())),
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
		&self.inner.0
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
		&self.inner.1
	}

	/// Returns the out-degree of the node. The out degree is the number of
	/// outbound edges.
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
	///
	/// assert!(a.out_degree() == 2);
	/// ```
	pub fn out_degree(&self) -> usize {
		self.inner.2.borrow().len_outbound()
	}

	/// Returns the in-degree of the node. The out degree is the number of
	/// inbound edges.
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
	/// b.connect(&a, 0.42);
	/// c.connect(&a, 1.7);
	///
	/// assert!(a.in_degree() == 2);
	pub fn in_degree(&self) -> usize {
		self.inner.2.borrow().len_inbound()
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
	pub fn connect(&self, other: &Self, value: E) {
		self.inner
			.2
			.borrow_mut()
			.push_outbound((other.clone(), value.clone()));
		other
			.inner
			.2
			.borrow_mut()
			.push_inbound((self.clone(), value));
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
	pub fn try_connect(&self, other: &Self, value: E) -> Result<(), E> {
		if self.is_connected(other.key()) {
			Err(value)
		} else {
			self.connect(other, value);
			Ok(())
		}
	}

	/// Disconnect two nodes from each other. The connection is removed in both
	/// directions. Returns Ok(EdgeValue) if the connection was removed,
	/// Err(()) if the connection doesn't exist.
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
			Some(other) => match self.inner.2.borrow_mut().remove_outbound(other.key()) {
				Ok(edge) => {
					other.inner.2.borrow_mut().remove_inbound(self.key())?;
					Ok(edge)
				}
				Err(_) => Err(()),
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
		for Edge(_, v, _) in self.iter_out() {
			v.inner.2.borrow_mut().remove_inbound(self.key()).unwrap();
		}
		for Edge(v, _, _) in self.iter_in() {
			v.inner.2.borrow_mut().remove_outbound(self.key()).unwrap();
		}
		self.inner.2.borrow_mut().clear_outbound();
		self.inner.2.borrow_mut().clear_inbound();
	}

	/// Returns true if the node is a root node. Root nodes are nodes that have
	/// no incoming connections.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	///
	/// n1.connect(&n2, ());
	///
	/// assert!(n1.is_root());
	/// assert!(!n2.is_root());
	/// ```
	pub fn is_root(&self) -> bool {
		self.inner.2.borrow().len_inbound() == 0
	}

	/// Returns true if the node is a leaf node. Leaf nodes are nodes that have
	/// no outgoing connections.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	///
	/// n1.connect(&n2, ());
	///
	/// assert!(!n1.is_leaf());
	/// assert!(n2.is_leaf());
	/// ```
	pub fn is_leaf(&self) -> bool {
		self.inner.2.borrow().len_outbound() == 0
	}

	/// Returns true if the node is an oprhan. Orphan nodes are nodes that have
	/// no connections.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	///
	/// n1.connect(&n2, ());
	///
	/// assert!(!n1.is_orphan());
	///
	/// n1.disconnect(n2.key()).unwrap();
	///
	/// assert!(n1.is_orphan());
	/// ```
	pub fn is_orphan(&self) -> bool {
		self.is_root() && self.is_leaf()
	}

	/// Returns true if the node is connected to another node with a given key.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	///
	/// n1.connect(&n2, ());
	///
	/// assert!(n1.is_connected(n2.key()));
	/// ```
	pub fn is_connected(&self, other: &K) -> bool {
		self.find_outbound(other).is_some()
	}

	/// Get a pointer to an adjacent node with a given key. Returns None if no
	/// node with the given key is found from the node's adjacency list.
	/// Outbound edges are searches, this is the default direction.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n1.connect(&n3, ());
	///
	/// assert!(n1.find_outbound(n2.key()).is_some());
	/// assert!(n1.find_outbound(n3.key()).is_some());
	/// assert!(n1.find_outbound(&4).is_none());
	/// ```
	pub fn find_outbound(&self, other: &K) -> Option<Node<K, N, E>> {
		let edge = self.inner.2.borrow();
		let edge = edge.find_outbound(other);
		edge.map(|edge| edge.0.upgrade().unwrap())
	}

	/// Get a pointer to an adjacent node with a given key. Returns None if no
	/// node with the given key is found from the node's adjacency list.
	/// Inbound edges are searched ie. the transposed graph.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n1.connect(&n3, ());
	///
	/// assert!(n2.find_inbound(n1.key()).is_some());
	/// assert!(n3.find_inbound(n1.key()).is_some());
	/// assert!(n1.find_inbound(&4).is_none());
	/// ```
	pub fn find_inbound(&self, other: &K) -> Option<Node<K, N, E>> {
		let edge = self.inner.2.borrow();
		let edge = edge.find_inbound(other);
		edge.map(|edge| edge.0.upgrade().unwrap())
	}

	/// Returns an iterator-like object that can be used to map, filter and
	/// collect reachable nodes or edges in different orderings such as
	/// postorder or preorder.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n2.connect(&n3, ());
	/// n3.connect(&n1, ());
	///
	/// let order = n1.preorder().search_nodes();
	///
	/// assert!(order[0] == n1);
	/// assert!(order[1] == n2);
	/// assert!(order[2] == n3);
	/// ```
	pub fn preorder(&self) -> Order<K, N, E> {
		Order::preorder(self)
	}

	/// Returns an iterator-like object that can be used to map, filter and
	/// collect reachable nodes or edges in different orderings such as
	/// postorder or preorder.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n2.connect(&n3, ());
	/// n3.connect(&n1, ());
	///
	/// let order = n1.postorder().search_nodes();
	///
	/// assert!(order[2] == n1);
	/// assert!(order[1] == n2);
	/// assert!(order[0] == n3);
	/// ```
	pub fn postorder(&self) -> Order<K, N, E> {
		Order::postroder(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a depth-first search.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n2.connect(&n3, ());
	/// n3.connect(&n1, ());
	///
	/// let path = n1
	/// 	.dfs()
	/// 	.target(&3)
	/// 	.search_path()
	/// 	.unwrap();
	///
	/// let mut iter = path.iter_nodes();
	///
	/// assert!(iter.next().unwrap() == n1);
	/// assert!(iter.next().unwrap() == n2);
	/// assert!(iter.next().unwrap() == n3);
	/// ```
	pub fn dfs(&self) -> Dfs<K, N, E> {
		Dfs::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a breadth-first
	/// search.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n2.connect(&n3, ());
	/// n3.connect(&n1, ());
	///
	/// let path = n1
	/// 	.bfs()
	/// 	.target(&3)
	/// 	.search_path()
	/// 	.unwrap();
	///
	/// let mut iter = path.iter_nodes();
	///
	/// assert!(iter.next().unwrap() == n1);
	/// assert!(iter.next().unwrap() == n2);
	/// assert!(iter.next().unwrap() == n3);
	/// ```
	pub fn bfs(&self) -> Bfs<K, N, E> {
		Bfs::new(self)
	}

	/// Returns an iterator-like object that can be used to map, filter,
	/// search and collect nodes or edges resulting from a
	/// priority-first search.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new('A', 0);
	/// let n2 = Node::new('B', 42);
	/// let n3 = Node::new('C', 7);
	/// let n4 = Node::new('D', 23);
	///
	/// n1.connect(&n2, ());
	/// n1.connect(&n3, ());
	/// n2.connect(&n4, ());
	/// n3.connect(&n4, ());
	///
	/// let path = n1
	/// 	.pfs()
	/// 	.target(&'D')
	/// 	.search_path()
	/// 	.unwrap();
	///
	/// assert!(path[0] == Edge(n1, n3.clone(), ()));
	/// assert!(path[1] == Edge(n3, n4, ()));
	///```
	pub fn pfs(&self) -> Pfs<K, N, E>
	where
		N: Ord,
	{
		Pfs::new(self)
	}

	/// Returns an iterator over the node's outbound edges.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n1.connect(&n3, ());
	///
	/// let mut iter = n1.iter_out();
	/// assert!(iter.next().unwrap() == Edge(n1.clone(), n2.clone(), ()));
	/// assert!(iter.next().unwrap() == Edge(n1, n3, ()));
	/// ```
	pub fn iter_out(&self) -> IterOut<K, N, E> {
		IterOut {
			node: self,
			position: 0,
		}
	}

	/// Returns an iterator over the node's inbound edges.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let n1 = Node::new(1, ());
	/// let n2 = Node::new(2, ());
	/// let n3 = Node::new(3, ());
	///
	/// n1.connect(&n2, ());
	/// n1.connect(&n3, ());
	///
	/// let mut iter = n2.iter_in();
	///
	/// assert!(iter.next().unwrap() == Edge(n1.clone(), n2.clone(), ()));
	/// assert!(iter.next().is_none());
	/// ```
	pub fn iter_in(&self) -> IterIn<K, N, E> {
		IterIn {
			node: self,
			position: 0,
		}
	}

	/// Return's the node's size in bytes.
	pub fn sizeof(&self) -> usize {
		std::mem::size_of::<Node<K, N, E>>()
			+ std::mem::size_of::<K>()
			+ std::mem::size_of::<N>()
			+ self.inner.2.borrow().sizeof()
			+ std::mem::size_of::<Self>()
	}
}

impl<K, N, E> Deref for Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Target = N;
	fn deref(&self) -> &Self::Target {
		self.value()
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
{
}

impl<K, N, E> PartialOrd for Node<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.value().cmp(other.value()))
	}
}

impl<K, N, E> Ord for Node<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.value().cmp(other.value())
	}
}

pub struct IterOut<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a Node<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for IterOut<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = Edge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.node.inner.2.borrow().get_outbound(self.position) {
			Some(current) => match current.0.upgrade() {
				Some(node) => {
					self.position += 1;
					Some(Edge(self.node.clone(), node, current.1.clone()))
				}
				None => {
					panic!(
						"Target node in the adjacency list of `node = {}` has been dropped.",
						self.node.key()
					);
				}
			},
			None => None,
		}
	}
}

pub struct IterIn<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	node: &'a Node<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for IterIn<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	type Item = Edge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.node.inner.2.borrow().get_inbound(self.position) {
			Some(current) => match current.0.upgrade() {
				Some(node) => {
					self.position += 1;
					Some(Edge(node, self.node.clone(), current.1.clone()))
				}
				None => {
					panic!(
						"Target node in the adjacency list of `node = {}` has been dropped.",
						self.node.key()
					);
				}
			},
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
	type Item = Edge<K, N, E>;
	type IntoIter = IterOut<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		IterOut {
			node: self,
			position: 0,
		}
	}
}
