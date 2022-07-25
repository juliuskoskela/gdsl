//==== graph::digraph =========================================================

//! # Directed Graph

//==== Includes ===============================================================

use std::{
    cell::RefCell,
    fmt::Display,
    hash::Hash,
    ops::Deref,
    rc::{Rc, Weak},
	collections::{HashMap, HashSet, VecDeque}
};

use min_max_heap::MinMaxHeap;

//==== DiGraph ==================================================================

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
	/// use ::digraph::*;
	///
	/// let mut g = DiGraph::<String, f64, f64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::new() } }

	/// Check if a node with the given key exists in the DiGraph
	///
	/// # Examples
	///
	/// ```
	/// use ::digraph::*;
	/// use dug::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
	/// use ::digraph::*;
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
}

impl<'a, K, N, E> std::ops::Index<K> for DiGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = DiNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output { &self.nodes[&key]
	}
}

//==== DiNode ===================================================================

#[derive(Clone)]
pub struct DiNode<K, N, E>
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
    edges: RefCell<DiNodeDiEdges<K, N, E>>,
}

struct DiNodeDiEdges<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	outbound: Vec<DiEdge<K, N, E>>,
	inbound: Vec<DiEdge<K, N, E>>,
}

//==== DiNode: Implement ========================================================

impl<K, N, E> DiNode<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	//==== Public Methods =====================================================

    pub fn new(key: K, value: N) -> Self {
		DiNode {
			inner: Rc::new(DiNodeInner {
				key,
				value,
				edges: RefCell::new(DiNodeDiEdges {
					outbound: Vec::new(),
					inbound: Vec::new(),
				}),
			}),
		}
    }

    pub fn key(&self) -> &K {
        &self.inner.key
    }

    pub fn value(&self) -> &N {
        &self.inner.value
    }

    pub fn connect(&self, other: &DiNode<K, N, E>, value: E) {
        let edge = DiEdge::new(self, other.clone(), value);
        self.inner.edges.borrow_mut().outbound.push(edge.clone());
        other.inner.edges.borrow_mut().inbound.push(edge);
    }

    pub fn disconnect(&self, other: DiNode<K, N, E>) {
        if self.delete_outbound(&other) {
            other.delete_inbound(self);
		}
    }

	pub fn isolate(&self) {
		let edges = self.inner.edges.borrow();
		for edge in edges.outbound.iter() {
			edge.target().delete_inbound(self);
		}
		for edge in edges.inbound.iter() {
			edge.source.upgrade().unwrap().delete_outbound(self);
		}
		let mut mut_edges = self.inner.edges.borrow_mut();
		mut_edges.outbound.clear();
		mut_edges.inbound.clear();
	}

	pub fn is_root(&self) -> bool {
		self.inner.edges.borrow().inbound.is_empty()
	}

	pub fn is_leaf(&self) -> bool {
		self.inner.edges.borrow().outbound.is_empty()
	}

	pub fn is_orphan(&self) -> bool {
		self.is_root() && self.is_leaf()
	}

	pub fn search(&self) -> DiNodeSearch<K, N, E> {
		DiNodeSearch { root: self.clone(), edge_tree: vec![] }
	}

    //==== Private Methods ====================================================

    fn delete_outbound(&self, other: &DiNode<K, N, E>) -> bool {
        let mut edges = self.inner.edges.borrow_mut();
        let mut deleted = false;
        let mut idx = 0;
        for (i, edge) in edges.outbound.iter().enumerate() {
            let outbound_target = &edge.target();
            if outbound_target == other {
                deleted = true;
                idx = i;
            }
        }
        if deleted {
            edges.outbound.remove(idx);
        }
        deleted
    }

	fn delete_inbound(&self, other: &DiNode<K, N, E>) -> bool {
        let mut edges = self.inner.edges.borrow_mut();
        let inbound = &mut edges.inbound;
        let mut deleted = false;
        let mut idx = 0;
        for (i, edge) in inbound.iter().enumerate() {
            let inbound_source = &edge.source;
            if inbound_source.upgrade().unwrap() == *other {
                deleted = true;
                idx = i;
            }
        }
        if deleted {
            inbound.remove(idx);
        }
        deleted
    }

	fn downgrade(&self) -> WeakDiNode<K, N, E> { WeakDiNode { inner: Rc::downgrade(&self.inner) } }
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

pub struct DiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a DiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for DiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (E, DiNode<K, N, E>);

	fn next(&mut self) -> Option<Self::Item> {
		let edges = self.node.inner.edges.borrow();
		let edge = edges.outbound.get(self.position);
		match edge {
			Some(edge) => {
				self.position += 1;
				Some((edge.value.clone(), edge.target().clone()))
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
	type Item = (E, DiNode<K, N, E>);
	type IntoIter = DiNodeIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		DiNodeIterator { node: self, position: 0 }
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
    fn new(source: &DiNode<K, N, E>, target: DiNode<K, N, E>, value: E) -> Self {
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


type Map<'a, K, N, E> = &'a dyn Fn(&DiNode<K, N, E>, &DiNode<K, N, E>, &E) -> bool;

/// Search for a node in the DiGraph.
pub struct DiNodeSearch<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: DiNode<K, N, E>,
	edge_tree: Vec<DiEdge<K, N, E>>,
}

impl<K, N, E> DiNodeSearch<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn dfs(&mut self, target: &DiNode<K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.inner.edges.borrow();
			let edge = edge_list.outbound.iter().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					if edge.target() == *target {
						return Some(self);
					}
					queue.push(edge.target().clone());
				}
				None => {}
			}
		}
		None
	}

	pub fn dfs_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop() {
			let edge_list = node.inner.edges.borrow();
			let edge = edge_list.outbound.iter().find(|e| visited.contains(e.target().key()) == false);
			match edge {
				Some(edge) => {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						if edge.target() == *target {
							return Some(self);
						}
						queue.push(edge.target().clone());
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn bfs(&mut self, target: &DiNode<K, N, E>) -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.inner.edges.borrow();
			for edge in edge_list.outbound.iter() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					if edge.target() == *target {
						return Some(self);
					}
					queue.push_back(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn bfs_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();

		queue.push_back(self.root.clone());
		while let Some(node) = queue.pop_front() {
			let edge_list = node.inner.edges.borrow();
			for edge in edge_list.outbound.iter() {
				if !visited.contains(edge.target().key()) {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						if edge.target() == *target {
							return Some(self);
						}
						queue.push_back(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn pfs_min(&mut self, target: &DiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let edge_list = node.inner.edges.borrow();
			for edge in edge_list.outbound.iter() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					self.edge_tree.push(edge.clone());
					if edge.target() == *target {
						return Some(self);
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn pfs_min_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_min() {
			let edge_list = node.inner.edges.borrow();
			for edge in edge_list.outbound.iter() {
				if !visited.contains(edge.target().key()) {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						if edge.target() == *target {
							return Some(self);
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn pfs_max(&mut self, target: &DiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let edge_list = node.inner.edges.borrow();
			for edge in edge_list.outbound.iter() {
				if !visited.contains(edge.target().key()) {
					visited.insert(edge.target().key().clone());
					if edge.target() == *target {
						return Some(self);
					}
					queue.push(edge.target().clone());
				}
			}
		}
		None
	}

	pub fn pfs_max_map<'a>(&mut self, target: &DiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();

		queue.push(self.root.clone());
		while let Some(node) = queue.pop_max() {
			let edge_list = node.inner.edges.borrow();
			for edge in edge_list.outbound.iter() {
				if !visited.contains(edge.target().key()) {
					if map(&edge.source(), &edge.target(), edge.value()) {
						visited.insert(edge.target().key().clone());
						self.edge_tree.push(edge.clone());
						if edge.target() == *target {
							return Some(self);
						}
						queue.push(edge.target().clone());
					}
				}
			}
		}
		None
	}

	pub fn edge_path(&self) -> Vec<DiEdge<K, N, E>> {
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

	pub fn node_path(&self) -> Vec<DiNode<K, N, E>> {
		let mut path = Vec::new();

		if !self.edge_path().is_empty() {
			let edge = self.edge_path()[0].clone();
			path.push(edge.source().clone());
			for edge in self.edge_path() {
				path.push(edge.target().clone());
			}
		}

		path
	}
}
