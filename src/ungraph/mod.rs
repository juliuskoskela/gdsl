//! Undirected Graph

mod graph_macros;
mod graph_serde;
mod node;

use std::{
    fmt::{Display, Write},
    hash::Hash,
};

use ahash::HashMap;

pub use self::node::*;

pub struct Graph<K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    nodes: HashMap<K, Node<K, N, E>>,
}

impl<K, N, E> Graph<K, N, E>
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
    pub fn new() -> Self {
        Self {
            nodes: HashMap::default(),
        }
    }

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
    pub fn contains(&self, key: &K) -> bool {
        self.nodes.contains_key(key)
    }

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
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

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
    pub fn get(&self, key: &K) -> Option<Node<K, N, E>> {
        self.nodes.get(key).cloned()
    }

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
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

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
        self.nodes.values().cloned().collect()
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
            .cloned()
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

    pub fn to_dot(&self) -> String {
        let mut s = String::new();
        s.push_str("digraph {\n");
        for (u_key, node) in self.iter() {
            write!(&mut s, "    {}", u_key.clone()).unwrap();
            for Edge(_, v, _) in node {
                write!(&mut s, "\n    {} -> {}", u_key, v.key()).unwrap();
            }
            s.push('\n');
        }
        s.push('}');
        s
    }

    fn fmt_attr(attrs: Vec<(String, String)>) -> String {
        let mut s = String::new();
        for (k, v) in attrs {
            write!(&mut s, "[{}=\"{}\"]", k, v).unwrap();
        }
        s
    }

    pub fn to_dot_with_attr(
        &self,
        gattr: &dyn Fn(&Self) -> Option<Vec<(String, String)>>,
        nattr: &dyn Fn(&Node<K, N, E>) -> Option<Vec<(String, String)>>,
        eattr: &dyn Fn(&Node<K, N, E>, &Node<K, N, E>, &E) -> Option<Vec<(String, String)>>,
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
            s.push('\n');
        }
        for (_, node) in self.iter() {
            for Edge(u, v, e) in node {
                s.push_str(&format!("\t{} -> {}", u.key(), v.key()));
                if let Some(eattrs) = eattr(&u, &v, &e) {
                    s.push_str(&format!(" {}", Self::fmt_attr(eattrs)));
                }
                s.push('\n');
            }
        }
        s.push('}');
        s
    }

    pub fn sizeof(&self) -> usize {
        let mut size = 0;
        for (k, node) in self.iter() {
            size += node.sizeof() + std::mem::size_of_val(k);
        }
        size
    }
}

impl<K, N, E> std::ops::Index<K> for Graph<K, N, E>
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

impl<K, N, E> Default for Graph<K, N, E>
where
    K: Clone + Hash + Display + Eq,
    N: Clone,
    E: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
