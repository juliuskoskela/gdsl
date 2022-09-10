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
//! use gdsl::ungraph::*;
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

mod algo;
mod adjacent;

use std::{
    fmt::Display,
    hash::Hash,
	cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use self::{
	algo::{
		pfs::*,
		dfs::*,
		bfs::*,
		order::*,
	},
	adjacent::*,
};

/// An edge between nodes is a tuple struct `Edge(u, v, e)` where `u` is the
/// source node, `v` is the target node, and `e` is the edge's value.
#[derive(Clone)]
pub struct Edge<K, N, E>(
    pub Node<K, N, E>,
    pub Node<K, N, E>,
    pub E,
) where
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

impl<K, N, E> PartialEq for Edge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone + Ord
{
	fn eq(&self, other: &Edge<K, N, E>) -> bool {
		self.2 == other.2
	}
}

impl<K, N, E> Eq for Edge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone + Ord
{}

impl<K, N, E> PartialOrd for Edge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone + Ord
{
	fn partial_cmp(&self, other: &Edge<K, N, E>) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<K, N, E> Ord for Edge<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone + Ord
{
	fn cmp(&self, other: &Edge<K, N, E>) -> std::cmp::Ordering {
		self.2.cmp(&other.2)
	}
}

/// A `Node<K, N, E>` is a key value pair smart-pointer, which includes inbound and
/// outbound connections to other nodes. Nodes can be created individually and they
/// don't depend on a graph container. Generic parameters include `K` for the node's
/// key, `N` for the node's value, and `E` for the edge's value. Two nodes are equal
/// if they have the same key.
///
/// # Example
///
/// ```
/// use gdsl::ungraph::*;
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
/// let Edge(u, v, e) = a.iter().next().unwrap();
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
	///	use gdsl::ungraph::*;
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
	///	use gdsl::ungraph::*;
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
	///	use gdsl::ungraph::*;
	///
	///	let n1 = Node::<i32, char, ()>::new(1, 'A');
	///
	///	assert!(*n1.value() == 'A');
	/// ```
    pub fn value(&self) -> &N {
        &self.inner.1
    }

	/// Returns the degree of the node. The degree is the number of
    /// adjacent edges.
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
    pub fn degree(&self) -> usize {
        self.inner.2.borrow().len_outbound()
		+ self.inner.2.borrow().len_inbound()
    }

	/// Connects this node to another node. The connection is created in both
	/// directions. The connection is created with the given edge value and
	/// defaults to `()`. This function allows for creating multiple
	/// connections between the same nodes.
	///
	/// # Example
	///
	/// ```
	/// use gdsl::ungraph::*;
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
	///	use gdsl::ungraph::*;
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
	///	use gdsl::ungraph::*;
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
		self.inner.2.borrow_mut().remove_undirected(other)
	}

	/// Removes all inbound and outbound connections to and from the node.
	///
	/// # Example
	///
	/// ```
	///	use gdsl::ungraph::*;
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
		for Edge(_, v, _) in self.iter() {
			if v.inner.2.borrow_mut().remove_inbound(self.key()).is_err() {
				v.inner.2.borrow_mut().remove_outbound(self.key()).unwrap();
			}
		}
		self.inner.2.borrow_mut().clear_outbound();
		self.inner.2.borrow_mut().clear_inbound();
	}

	/// Returns true if the node is an oprhan. Orphan nodes are nodes that have
	/// no connections.
	pub fn is_orphan(&self) -> bool {
		self.inner.2.borrow().len_outbound() == 0 && self.inner.2.borrow().len_inbound() == 0
	}

	/// Returns true if the node is connected to another node with a given key.
	pub fn is_connected(&self, other: &K) -> bool {
		self.find_adjacent(other).is_some()
	}

	/// Get a pointer to an adjacent node with a given key. Returns None if no
	/// node with the given key is found from the node's adjacency list.
	pub fn find_adjacent(&self, other: &K) -> Option<Node<K, N, E>> {
		self.inner.2.borrow().find_adjacent(other).map(|(n, _)| n.upgrade().unwrap())
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

	/// Returns an iterator over the node's adjacent edges.
	pub fn iter(&self) -> NodeIterator<K, N, E> {
		NodeIterator { node: self, position: 0 }
	}

	pub fn sizeof(&self) -> usize {
        std::mem::size_of::<Node<K, N, E>>()
            + std::mem::size_of::<K>()
            + std::mem::size_of::<N>()
            + self.inner.2.borrow().sizeof()
            + std::mem::size_of::<Self>()
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
{}

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

pub struct NodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a Node<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for NodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = Edge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		let adjacent = &self.node.inner.2.borrow();
		match adjacent.get_adjacent(self.position) {
			Some((n, e)) => {
				self.position += 1;
				Some(Edge(self.node.clone(), n.upgrade().unwrap(), e.clone()))
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
	type Item = Edge<K, N, E>;
	type IntoIter = NodeIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		NodeIterator { node: self, position: 0 }
	}
}