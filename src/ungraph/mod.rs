//! Undirected Graph

//==== Submodules =============================================================

mod node;
mod graph_macros;

//==== Includes ===============================================================

use std::{
	fmt::Display,
    hash::Hash,
};

use fnv::FnvHashMap as HashMap;

pub use crate::ungraph::node::*;

pub struct Graph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	nodes: HashMap<K, Node<K, N, E>>,
}

impl<'a, K, N, E> Graph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	/// Create a new Graph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::default() } }

	/// Check if a node with the given key exists in the Graph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// ```
	pub fn contains(&self, key: &K) -> bool { self.nodes.contains_key(key) }

	/// Get the length of the Graph (amount of nodes)
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
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
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
	/// g.insert(Node::new("C", 0));
	///
	/// let node = g.get(&"A").unwrap();
	///
	/// assert!(node.key() == &"A");
	/// ```
	pub fn get(&self, key: &K) -> Option<Node<K, N, E>> { self.nodes.get(key).map(|node| node.clone()) }

	/// Check if Graph is empty
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// assert!(g.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

	/// Insert a node into the Graph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	///
	/// assert!(g.contains(&"A"));
	/// assert!(g.insert(Node::new("A", 0)) == false);
	/// ```
	pub fn insert(&mut self, node: Node<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	/// Remove a node from the Graph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
	///
	/// assert!(g.contains(&"A"));
	///
	/// g.remove(&"A");
	///
	/// assert!(g.contains(&"A") == false);
	/// ```
	pub fn remove(&mut self, node: &K) -> Option<Node<K, N, E>> {
		self.nodes.remove(node)
	}

	/// Collect nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
	/// g.insert(Node::new("C", 0));
	///
	/// let nodes = g.to_vec();
	///
	/// assert!(nodes.len() == 3);
	/// ```
	pub fn to_vec(&self) -> Vec<Node<K, N, E>> {
		self.nodes.values().map(|node| node.clone()).collect()
	}

	/// Collect orpahn nodes into a vector
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
	/// g.insert(Node::new("C", 0));
	/// g.insert(Node::new("D", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	///
	/// let orphans = g.orphans();
	///
	/// assert!(orphans.len() == 2);
	/// ```
	pub fn orphans(&self) -> Vec<Node<K, N, E>> {
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
	/// use gdsl::ungraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
	/// g.insert(Node::new("C", 0));
	///
	/// for (key, _)in g.iter() {
	///    println!("{}", key);
	/// }
	/// ```
	pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, Node<K, N, E>> {
		self.nodes.iter()
	}

	pub fn to_dot(&self) -> String
	where
		N: Display,
		E: Display,
	{
		let mut s = String::new();
		s.push_str("digraph {\n");
		for (u_key, node) in self.iter() {
			s.push_str(&format!("    {}", u_key.clone()));
			for (_, v, _) in node {
				s.push_str(&format!("\n    {} -> {}", u_key, v.key()));
			}
			s.push_str("\n");
		}
		s.push_str("}");
		s
	}
}

impl<'a, K, N, E> std::ops::Index<K> for Graph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = Node<K, N, E>;

	fn index(&self, key: K) -> &Self::Output {
		&self.nodes[&key]
	}
}
