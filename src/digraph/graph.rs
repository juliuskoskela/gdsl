//==== graphgdsl::graph =========================================================

//! # Directed DiGraph

//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::HashMap
};

use crate::digraph::node::*;

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
	/// use gdsl::digraph::graph::*;
	///
	/// let mut g = DiGraph::<&str, u64, u64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::new() } }

	/// Check if a node with the given key exists in the DiGraph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
	/// use gdsl::digraph::graph::*;
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
