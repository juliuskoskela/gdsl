use std::fmt::Display;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::Deref;
use std::collections::VecDeque;
use min_max_heap::MinMaxHeap;
use crate::*;

pub trait FmtDot {
	fn fmt_dot(&self) -> String;
}

impl FmtDot for Empty {
	fn fmt_dot(&self) -> String {
		String::new()
	}
}

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
	pub fn new() -> Self {
		Self {
			nodes: HashMap::new(),
		}
	}

	pub fn contains(&self, key: &K) -> bool {
		self.nodes.contains_key(key)
	}

	pub fn len(&self) -> usize {
		self.nodes.len()
	}

	pub fn get(&self, key: &K) -> Option<Node<K, N, E>> {
		self.nodes.get(key).map(|node| node.clone())
	}

	pub fn insert(&mut self, node: Node<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	pub fn remove(&mut self, node: Node<K, N, E>) -> Option<Node<K, N, E>> {
		self.nodes.remove(node.key())
	}

	pub fn roots(&self) -> Vec<Node<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn leafs(&self) -> Vec<Node<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn orphans(&self) -> Vec<Node<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().borrow().is_empty() && node.outbound().borrow().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn fmt_dot(&self) -> String
	 where
		N: FmtDot,
		E: FmtDot,
	{
		let mut s = String::new();
		s.push_str("digraph {\n");
		for node in self.nodes.values() {
			s.push_str(&format!("    {} [ label=\"{}: {}\" ]\n", node.key(), node.key(), node.fmt_dot()));
		}
		for node in self.nodes.values() {
			for edge in node {
				s.push_str(&format!("    {} -> {} [ label=\"{}\" ]\n", edge.source().key(), edge.target().key(), edge.fmt_dot()));
			}
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

pub struct WeakNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<NodeInner<K, N, E>>,
}

impl<K, N, E> WeakNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	pub fn upgrade(&self) -> Option<Node<K, N, E>> {
		self.handle.upgrade().map(|handle| Node { handle })
	}
}

struct NodeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	key: K,
	params: N,
	outbound: RefCell<Vec<Edge<K, N, E>>>,
	inbound: RefCell<Vec<WeakEdge<K, N, E>>>,
}

#[derive(Clone)]
pub struct Node<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<NodeInner<K, N, E>>,
}

impl<K, N, E> Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(key: K, params: N) -> Self {
		Self {
			handle: Rc::new(NodeInner {
				params,
				key,
				inbound: RefCell::new(vec![]),
				outbound: RefCell::new(vec![]),
			}),
		}
	}

	fn downgrade(&self) -> WeakNode<K, N, E> { WeakNode { handle: Rc::downgrade(&self.handle) } }

	pub fn key(&self) -> &K { &self.handle.key }

	pub fn params(&self) -> &N { &self.handle.params }

	pub fn outbound(&self) -> &RefCell<Vec<Edge<K, N, E>>> { &self.handle.outbound }

	pub fn find_outbound(&self, other: &Node<K, N, E>) -> Option<Edge<K, N, E>> {
		for edge in self.outbound().borrow().iter() {
			if &edge.target() == other {
				return Some(edge.clone());
			}
		}
		None
	}

	fn delete_outbound(&self, other: &Node<K, N, E>) -> bool {
		let mut outbound = self.outbound().borrow_mut();
		let (mut idx, mut found) = (0, false);
		for (i, edge) in outbound.iter().enumerate() {
			if &edge.target() == other {
				idx = i;
				found = true;
			}
		}
		if found {
			outbound.remove(idx);
		}
		found
	}

	pub fn inbound(&self) -> &RefCell<Vec<WeakEdge<K, N, E>>> { &self.handle.inbound }

	pub fn find_inbound(&self, other: &Node<K, N, E>) -> Option<Edge<K, N, E>> {
		for edge in self.inbound().borrow().iter() {
			if &edge.upgrade().unwrap().source() == other {
				return Some(edge.upgrade().unwrap().clone());
			}
		}
		None
	}

	fn delete_inbound(&self, other: &Node<K, N, E>) -> bool {
		let mut inbound = self.inbound().borrow_mut();
		let (mut idx, mut found) = (0, false);
		for (i, edge) in inbound.iter().enumerate() {
			if &edge.upgrade().unwrap().source() == other {
				idx = i;
				found = true;
			}
		}
		if found {
			inbound.remove(idx);
		}
		found
	}

	pub fn connect(&self, other: &Node<K, N, E>, params: E) {
		let edge = Edge::new(self, other.clone(), params);
		self.outbound().borrow_mut().push(edge.clone());
		other.inbound().borrow_mut().push(edge.downgrade());
	}

	pub fn try_connect(&self, other: &Node<K, N, E>, params: E) -> bool {
		if self.outbound().borrow().iter().any(|e| &e.target() == other) {
			return false;
		}
		self.connect(other, params);
		true
	}

	pub fn disconnect(&self, other: Node<K, N, E>) -> bool{
		if other.delete_inbound(self) {
			self.delete_outbound(&other)
		} else {
			false
		}
	}

	pub fn isolate(&self) {
		for edge in self.inbound().borrow().iter() {
			let target = edge.upgrade().unwrap().target();
			target.delete_outbound(self);
		}

		for edge in self.outbound().borrow().iter() {
			let target = edge.target();
			target.delete_inbound(self);
		}

		self.outbound().borrow_mut().clear();
		self.inbound().borrow_mut().clear();
	}

	pub fn search(&self) -> NodeSearch<K, N, E> {
		NodeSearch { root: self.clone(), edge_tree: vec![] }
	}
}

pub type Map<'a, K, N, E> = &'a dyn Fn(Node<K, N, E>, Node<K, N, E>, E) -> bool;

pub struct NodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: Node<K, N, E>,
	edge_tree: Vec<Edge<K, N, E>>,
}

impl<K, N, E> NodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn root(&self) -> Node<K, N, E> {
		self.root.clone()
	}

	pub fn edge_tree(&self) -> &Vec<Edge<K, N, E>> {
		&self.edge_tree
	}

	pub fn edge_tree_mut(&mut self) -> &mut Vec<Edge<K, N, E>> {
		&mut self.edge_tree
	}

	pub fn dfs(&mut self, target: &Node<K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = Graph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop() {
			for edge in &node {
				if visited.insert(edge.target()) {
					self.edge_tree_mut().push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push(edge.target());
				}
			}
		}
		None
	}

	pub fn dfs_map<'a>(&mut self, target: &Node<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = Graph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop() {
			for edge in &node {
				if visited.insert(edge.target()) {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree_mut().push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push(edge.target());
					} else {
						visited.remove(edge.target());
					}
				}
			}
		}
		None
	}

	pub fn bfs(&mut self, target: &Node<K, N, E>) -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = Graph::new();

		queue.push_back(self.root());
		while let Some(node) = queue.pop_front() {
			for edge in &node {
				if visited.insert(edge.target()) {
					self.edge_tree_mut().push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push_back(edge.target());
				}
			}
		}
		None
	}

	pub fn bfs_map<'a>(&mut self, target: &Node<K, N, E>, map: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = Graph::new();

		queue.push_back(self.root());
		while let Some(node) = queue.pop_front() {
			for edge in &node {
				if visited.insert(edge.target()) {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree_mut().push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push_back(edge.target());
					} else {
						visited.remove(edge.target());
					}
				}
			}
		}
		None
	}

	pub fn pfs_min(&mut self, target: &Node<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = Graph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop_min() {
			for edge in &node {
				if visited.insert(edge.target()) {
					self.edge_tree_mut().push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push(edge.target());
				}
			}
		}
		None
	}

	pub fn pfs_min_map<'a>(&mut self, target: &Node<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = Graph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop_min() {
			for edge in &node {
				if visited.insert(edge.target()) {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree_mut().push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push(edge.target());
					} else {
						visited.remove(edge.target());
					}
				}
			}
		}
		None
	}

	pub fn pfs_max(&mut self, target: &Node<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = Graph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop_max() {
			for edge in &node {
				if visited.insert(edge.target()) {
					self.edge_tree_mut().push(edge.clone());
					if &edge.target() == target {
						return Some(self);
					}
					queue.push(edge.target());
				}
			}
		}
		None
	}

	pub fn pfs_max_map<'a>(&mut self, target: &Node<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = Graph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop_max() {
			for edge in &node {
				if visited.insert(edge.target()) {
					let (s, t, e) = edge.decomp();
					if map(s, t, e) {
						self.edge_tree_mut().push(edge.clone());
						if &edge.target() == target {
							return Some(self);
						}
						queue.push(edge.target());
					} else {
						visited.remove(edge.target());
					}
				}
			}
		}
		None
	}

	pub fn edge_path(&self) -> Vec<Edge<K, N, E>> {
		let mut path = Vec::new();

		let len = self.edge_tree().len() - 1;
		let w = self.edge_tree()[len].clone();
		path.push(w.clone());
		let mut i = 0;
		for edge in self.edge_tree().iter().rev() {
			let source = path[i].source();
			if edge.target() == source {
				path.push(edge.clone());
				i += 1;
			}
		}
		path.reverse();
		path
	}

	pub fn node_path(&self) -> Vec<Node<K, N, E>> {
		if self.edge_path().len() == 0 {
			return Vec::new();
		}
		let mut path = Vec::new();
		path.push(self.edge_path()[0].source());
		for edge in self.edge_path() {
			path.push(edge.target());
		}
		path
	}
}

///////////////////////////////////////////////////////////////////////////////
// Blanket implementations for Graph<K, N, E>
///////////////////////////////////////////////////////////////////////////////

impl<K, N, E> Deref for Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Target = N;

	fn deref(&self) -> &Self::Target {
		&self.params()
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
{ }

impl<K, N, E> PartialOrd for Node<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.params().cmp(&other.params()))
	}
}

impl<K, N, E> Ord for Node<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.params().cmp(&other.params())
	}
}

pub struct NodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: Node<K, N, E>,
	position: usize,
}

impl<K, N, E> Iterator for NodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = Edge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = Edge<K, N, E>;
	type IntoIter = NodeIntoIterator<K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		NodeIntoIterator { node: self, position: 0 }
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
		if self.position >= self.node.outbound().borrow().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().borrow()[self.position - 1].clone())
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

// EDGE

struct EdgeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	params: E,
	source: WeakNode<K, N, E>,
	target: Node<K, N, E>,
}

#[derive(Clone)]
pub struct Edge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Rc<EdgeInner<K, N, E>>,
}

impl<K, N, E> Edge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn new(source: &Node<K, N, E>, target: Node<K, N, E>, params: E) -> Self {
		let handle = Rc::new(EdgeInner {
			params,
			source: source.downgrade(),
			target: target.clone(),
		});
		Self { handle }
	}

	pub fn downgrade(&self) -> WeakEdge<K, N, E> {
		WeakEdge { handle: Rc::downgrade(&self.handle) }
	}

	pub fn source(&self) -> Node<K, N, E> {
		self.handle.source.upgrade().unwrap().clone()
	}

	pub fn target(&self) -> Node<K, N, E> {
		self.handle.target.clone()
	}

	pub fn params(&self) -> &E {
		&self.handle.params
	}

	pub fn decomp(&self) -> (Node<K, N, E>, Node<K, N, E>, E) {
		(self.source(), self.target(), self.params().clone())
	}
}

impl<K, N, E> Deref for Edge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Target = E;

	fn deref(&self) -> &Self::Target {
		&self.params()
	}
}

pub struct WeakEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<EdgeInner<K, N, E>>,
}

impl<K, N, E> WeakEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<Edge<K, N, E>> {
		self.handle.upgrade().map(|handle| Edge { handle })
	}
}

pub fn nodes_exist<K, N, E>(graph: &Graph<K, N, E>, s: K, t: K) -> bool
where
	K: std::fmt::Debug + std::fmt::Display + Hash + Eq + Clone + PartialEq,
	N: std::fmt::Debug + Clone,
	E: Clone,
{
	if !graph.contains(&s) && !graph.contains(&t) {
		panic!("Check your macro invocation: {} and {} are not in the graph", s, t);
	} else if !graph.contains(&s) {
		panic!("Check your macro invocation: {} is not in the graph", s);
	} else if !graph.contains(&t) {
		panic!("Check your macro invocation: {} is not in the graph", t);
	} else {
		true
	}
}

#[macro_export]
macro_rules! node {
	( $key:expr ) => {
        {
			use crate::graph::*;

            Node::new($key, Empty)
        }
    };
    ( $key:expr, $param:expr ) => {
        {
			use crate::graph::*;

            Node::new($key, $param)
        }
    };
}

#[macro_export]
macro_rules! connect {
	( $s:expr => $t:expr ) => {
        {
			use crate::graph::*;

            Node::connect($s, $t, Empty)
        }
    };
    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use crate::graph::*;

            Node::connect($s, $t, $params)
        }
    };
}

#[macro_export]
macro_rules! graph {

	// (Key)
	( ($K:ty) $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use crate::graph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut graph = Graph::<$K, Empty, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE);
				graph.insert(n);
			)*
			for (s, t) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t);
				}
			}
			graph
		}
	};

	// (Key, Node)
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) => $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use crate::graph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut graph = Graph::<$K, $N, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				graph.insert(n);
			)*
			for (s, t) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t);
				}
			}
			graph
		}
	};

	// (Key) => [Edge]
	( ($K:ty) => [$E:ty] $(($NODE:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::graph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut graph = Graph::<$K, Empty, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE);
				graph.insert(n);
			)*
			for (s, t, param) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t, param);
				}
			}
			graph
		}
	};

	// (Key, Node) -> [Edge]
	( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::graph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut graph = Graph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				graph.insert(n);
			)*
			for (s, t, param) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t, param);
				}
			}
			graph
		}
	};
}
