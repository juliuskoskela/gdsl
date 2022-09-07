//! Directed Graph

//==== Submodules =============================================================

mod node;
mod graph_macros;
mod graph_serde;

//==== Includes ===============================================================

use std::{
	fmt::Display,
    hash::Hash,
};

use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
pub use crate::sync_digraph::node::*;

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
	/// use gdsl::digraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	/// ```
	pub fn new() -> Self { Self { nodes: HashMap::default() } }

	/// Check if a node with the given key exists in the Graph
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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

	/// Collect roots into a vector
	///
	/// # Examples
	///
	/// ```
	/// use gdsl::digraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
	/// g.insert(Node::new("C", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	/// g["A"].connect(&g["C"], 0x1);
	/// g["B"].connect(&g["C"], 0x1);
	///
	/// let roots = g.roots();
	///
	/// assert!(roots.len() == 1);
	/// ```
	pub fn roots(&self) -> Vec<Node<K, N, E>> {
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
	/// use gdsl::digraph::*;
	///
	/// let mut g = Graph::<&str, u64, u64>::new();
	///
	/// g.insert(Node::new("A", 0));
	/// g.insert(Node::new("B", 0));
	/// g.insert(Node::new("C", 0));
	///
	/// g["A"].connect(&g["B"], 0x1);
	/// g["A"].connect(&g["C"], 0x1);
	///
	/// let leaves = g.leaves();
	///
	/// assert!(leaves.len() == 2);
	/// ```
	pub fn leaves(&self) -> Vec<Node<K, N, E>> {
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
	/// use gdsl::digraph::*;
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
	/// use gdsl::digraph::*;
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

	fn scc_ordering(&self) -> Vec<Node<K, N, E>> {
		let mut visited = HashSet::new();
		let mut ordering = Vec::new();

		for (_, next) in self.iter() {
			if !visited.contains(next.key()) {
				let partition = next
					.order()
					.post()
					.filter(&|_, v, _| !visited.contains(v.key()))
					.search_nodes();
				for node in &partition {
					visited.insert(node.key().clone());
					ordering.push(node.clone());
				}
			}
		}
		ordering
	}

	pub fn scc(&self) -> Vec<Vec<Node<K, N, E>>> {
		let mut invariant = HashSet::new();
		let mut components = Vec::new();
		let mut ordering = self.scc_ordering();

		while let Some(node) = ordering.pop() {
			if !invariant.contains(node.key()) {
				let cycle = node
					.dfs()
					.transpose()
					.filter(&|_, v, _| !invariant.contains(v.key()))
					.search_cycle();
				match cycle {
					Some(cycle) => {
						let mut cycle = cycle.to_vec_nodes();
						cycle.pop();
						for node in &cycle {
							invariant.insert(node.key().clone());
						}
						components.push(cycle);
					},
					None => {
						invariant.insert(node.key().clone());
						components.push(vec![node.clone()]);
					},
				}
			}
		}
		components
	}

	pub fn to_dot(&self) -> String
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

	fn fmt_attr(attrs: Vec<(String, String)>) -> String {
		let mut s = String::new();
		for (k, v) in attrs {
			s.push_str(&format!("[{}=\"{}\"]", k, v));
		}
		s
	}

	pub fn to_dot_with_attr(&self,
		gattr: &dyn Fn(&Self) -> Option<Vec<(String, String)>>,
		nattr: &dyn Fn(&Node<K, N, E>) -> Option<Vec<(String, String)>>,
		eattr: &dyn Fn(&Node<K, N, E>, &Node<K, N, E>, &E) -> Option<Vec<(String, String)>>
	) -> String {
		let mut s = String::new();
		s.push_str("digraph {\n");
		if let Some(gattrs) = gattr(self) {
			for (k, v) in gattrs {
				s.push_str(&format!("\t{}=\"{}\"\n", k, v));
			}
		}
		for (u_key, node) in self.iter() {
			s.push_str(&format!("\t{}", u_key.clone()));
			if let Some(nattr) = nattr(node) {
				s.push_str(&format!(" {}", Self::fmt_attr(nattr)));
			}
			s.push_str("\n");
		}
		for (_, node) in self.iter() {
			for (u, v, edge) in node {
				s.push_str(&format!("\t{} -> {}", u.key(), v.key()));
				if let Some(eattrs) = eattr(&u, &v, &edge) {
					s.push_str(&format!(" {}", Self::fmt_attr(eattrs)));
				}
				s.push_str("\n");
			}
		}
		s.push_str("}");
		s
	}

	pub fn sizeof(&self) -> usize {
		let mut size = 0;
		for (_, node) in self.iter() {
			size += node.sizeof();
		}
		size
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
