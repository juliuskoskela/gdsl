use std::fmt::Display;
use std::hash::Hash;
use std::sync::{Arc, Weak};
use parking_lot::RwLock;
use std::ops::Deref;
use std::collections::VecDeque;
use min_max_heap::MinMaxHeap;
use std::collections::HashMap;

pub struct AsyncGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	nodes: HashMap<K, AsyncNode<K, N, E>>,
}

impl<'a, K, N, E> AsyncGraph<K, N, E>
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

	pub fn get(&self, key: &K) -> Option<AsyncNode<K, N, E>> {
		self.nodes.get(key).map(|node| node.clone())
	}

	pub fn insert(&mut self, node: AsyncNode<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	pub fn remove(&mut self, node: AsyncNode<K, N, E>) -> Option<AsyncNode<K, N, E>> {
		self.nodes.remove(node.key())
	}

	pub fn roots(&self) -> Vec<AsyncNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().read().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn leafs(&self) -> Vec<AsyncNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.outbound().read().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn orphans(&self) -> Vec<AsyncNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().read().is_empty() && node.outbound().read().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn to_vec(&self) -> Vec<AsyncNode<K, N, E>> {
		self.nodes.values().map(|node| node.clone()).collect()
	}
}

impl<'a, K, N, E> std::ops::Index<K> for AsyncGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = AsyncNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output {
		&self.nodes[&key]
	}
}

pub struct WeakAsyncNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<AsyncNodeInner<K, N, E>>,
}

impl<K, N, E> WeakAsyncNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	pub fn upgrade(&self) -> Option<AsyncNode<K, N, E>> {
		self.handle.upgrade().map(|handle| AsyncNode { handle })
	}
}

struct AsyncNodeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	key: K,
	params: N,
	outbound: RwLock<Vec<AsyncEdge<K, N, E>>>,
	inbound: RwLock<Vec<WeakAsyncEdge<K, N, E>>>,
}

#[derive(Clone)]
pub struct AsyncNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Arc<AsyncNodeInner<K, N, E>>,
}

impl<K, N, E> AsyncNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(key: K, params: N) -> Self {
		Self {
			handle: Arc::new(AsyncNodeInner {
				params,
				key,
				inbound: RwLock::new(vec![]),
				outbound: RwLock::new(vec![]),
			}),
		}
	}

	fn downgrade(&self) -> WeakAsyncNode<K, N, E> { WeakAsyncNode { handle: Arc::downgrade(&self.handle) } }

	pub fn key(&self) -> &K { &self.handle.key }

	pub fn params(&self) -> &N { &self.handle.params }

	pub fn outbound(&self) -> &RwLock<Vec<AsyncEdge<K, N, E>>> { &self.handle.outbound }

	pub fn find_outbound(&self, other: &AsyncNode<K, N, E>) -> Option<AsyncEdge<K, N, E>> {
		for edge in self.outbound().read().iter() {
			if &edge.target() == other {
				return Some(edge.clone());
			}
		}
		None
	}

	fn delete_outbound(&self, other: &AsyncNode<K, N, E>) -> bool {
		let (mut idx, mut found) = (0, false);
		for (i, edge) in self.outbound().read().iter().enumerate() {
			if &edge.target() == other {
				idx = i;
				found = true;
				break;
			}
		}
		if found {
			self.outbound().write().remove(idx);
		}
		found
	}

	pub fn inbound(&self) -> &RwLock<Vec<WeakAsyncEdge<K, N, E>>> { &self.handle.inbound }

	pub fn find_inbound(&self, other: &AsyncNode<K, N, E>) -> Option<AsyncEdge<K, N, E>> {
		for edge in self.inbound().read().iter() {
			if &edge.upgrade().unwrap().source() == other {
				return Some(edge.upgrade().unwrap().clone());
			}
		}
		None
	}

	fn delete_inbound(&self, other: &AsyncNode<K, N, E>) -> bool {
		let (mut idx, mut found) = (0, false);
		for (i, edge) in self.inbound().read().iter().enumerate() {
			if &edge.upgrade().unwrap().source() == other {
				idx = i;
				found = true;
			}
		}
		if found {
			self.inbound().write().remove(idx);
		}
		found
	}

	pub fn connect(&self, other: &AsyncNode<K, N, E>, params: E) {
		let edge = AsyncEdge::new(self, other.clone(), params);
		self.outbound().write().push(edge.clone());
		other.inbound().write().push(edge.downgrade());
	}

	pub fn try_connect(&self, other: &AsyncNode<K, N, E>, params: E) -> bool {
		if self.outbound().read().iter().any(|e| &e.target() == other) {
			return false;
		}
		self.connect(other, params);
		true
	}

	pub fn disconnect(&self, other: AsyncNode<K, N, E>) -> bool{
		if other.delete_inbound(self) {
			self.delete_outbound(&other)
		} else {
			false
		}
	}

	pub fn isolate(&self) {
		for edge in self.inbound().read().iter() {
			let target = edge.upgrade().unwrap().target();
			target.delete_outbound(self);
		}

		for edge in self.outbound().read().iter() {
			let target = edge.target();
			target.delete_inbound(self);
		}

		self.outbound().write().clear();
		self.inbound().write().clear();
	}

	pub fn search(&self) -> AsyncNodeSearch<K, N, E> {
		AsyncNodeSearch { root: self.clone(), edge_tree: vec![] }
	}
}

pub type Map<'a, K, N, E> = &'a dyn Fn(AsyncEdge<K, N, E>) -> bool;

pub struct AsyncNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: AsyncNode<K, N, E>,
	edge_tree: Vec<AsyncEdge<K, N, E>>,
}

impl<K, N, E> AsyncNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn root(&self) -> AsyncNode<K, N, E> {
		self.root.clone()
	}

	pub fn edge_tree(&self) -> &Vec<AsyncEdge<K, N, E>> {
		&self.edge_tree
	}

	pub fn edge_tree_mut(&mut self) -> &mut Vec<AsyncEdge<K, N, E>> {
		&mut self.edge_tree
	}

	pub fn dfs(&mut self, target: &AsyncNode<K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = AsyncGraph::new();

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

	pub fn dfs_map<'a>(&mut self, target: &AsyncNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = AsyncGraph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop() {
			for edge in &node {
				if visited.insert(edge.target()) {
					if map(edge.clone()) {
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

	pub fn bfs(&mut self, target: &AsyncNode<K, N, E>) -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = AsyncGraph::new();

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

	pub fn bfs_map<'a>(&mut self, target: &AsyncNode<K, N, E>, map: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = AsyncGraph::new();

		queue.push_back(self.root());
		while let Some(node) = queue.pop_front() {
			for edge in &node {
				if visited.insert(edge.target()) {
					if map(edge.clone()) {
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

	pub fn pfs_min(&mut self, target: &AsyncNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncGraph::new();

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

	pub fn pfs_min_map<'a>(&mut self, target: &AsyncNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncGraph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop_min() {
			for edge in &node {
				if visited.insert(edge.target()) {
					if map(edge.clone()) {
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

	pub fn pfs_max(&mut self, target: &AsyncNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncGraph::new();

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

	pub fn pfs_max_map<'a>(&mut self, target: &AsyncNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncGraph::new();

		queue.push(self.root());
		while let Some(node) = queue.pop_max() {
			for edge in &node {
				if visited.insert(edge.target()) {
					if map(edge.clone()) {
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

	pub fn edge_path(&self) -> Vec<AsyncEdge<K, N, E>> {
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

	pub fn node_path(&self) -> Vec<AsyncNode<K, N, E>> {
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
// Blanket implementations for AsyncGraph<K, N, E>
///////////////////////////////////////////////////////////////////////////////

impl<K, N, E> Deref for AsyncNode<K, N, E>
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

impl<K, N, E> PartialEq for AsyncNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn eq(&self, other: &Self) -> bool {
		self.key() == other.key()
	}
}


impl<K, N, E> Eq for AsyncNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{ }

impl<K, N, E> PartialOrd for AsyncNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.params().cmp(&other.params()))
	}
}

impl<K, N, E> Ord for AsyncNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.params().cmp(&other.params())
	}
}

pub struct AsyncNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: AsyncNode<K, N, E>,
	position: usize,
}

impl<K, N, E> Iterator for AsyncNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().read().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().read()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for AsyncNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncEdge<K, N, E>;
	type IntoIter = AsyncNodeIntoIterator<K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		AsyncNodeIntoIterator { node: self, position: 0 }
	}
}

pub struct AsyncNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a AsyncNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for AsyncNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().read().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().read()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a AsyncNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncEdge<K, N, E>;
	type IntoIter = AsyncNodeIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		AsyncNodeIterator { node: self, position: 0 }
	}
}

// EDGE

struct AsyncEdgeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	params: E,
	source: WeakAsyncNode<K, N, E>,
	target: AsyncNode<K, N, E>,
}

#[derive(Clone)]
pub struct AsyncEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Arc<AsyncEdgeInner<K, N, E>>,
}

impl<K, N, E> AsyncEdge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn new(source: &AsyncNode<K, N, E>, target: AsyncNode<K, N, E>, params: E) -> Self {
		let handle = Arc::new(AsyncEdgeInner {
			params,
			source: source.downgrade(),
			target: target.clone(),
		});
		Self { handle }
	}

	pub fn downgrade(&self) -> WeakAsyncEdge<K, N, E> {
		WeakAsyncEdge { handle: Arc::downgrade(&self.handle) }
	}

	pub fn source(&self) -> AsyncNode<K, N, E> {
		self.handle.source.upgrade().unwrap().clone()
	}

	pub fn target(&self) -> AsyncNode<K, N, E> {
		self.handle.target.clone()
	}

	pub fn params(&self) -> &E {
		&self.handle.params
	}
}

impl<K, N, E> Deref for AsyncEdge<K, N, E>
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

pub struct WeakAsyncEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<AsyncEdgeInner<K, N, E>>,
}

impl<K, N, E> WeakAsyncEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<AsyncEdge<K, N, E>> {
		self.handle.upgrade().map(|handle| AsyncEdge { handle })
	}
}

pub fn async_nodes_exist<K, N, E>(graph: &AsyncGraph<K, N, E>, s: K, t: K) -> bool
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
macro_rules! async_node {
	( $key:expr ) => {
        {
			use crate::async_graph::*;

            AsyncNode::new($key, Empty)
        }
    };
    ( $key:expr, $param:expr ) => {
        {
			use crate::async_graph::*;

            AsyncNode::new($key, $param)
        }
    };
}

#[macro_export]
macro_rules! async_connect {
	( $s:expr => $t:expr ) => {
        {
			use crate::async_graph::*;

            AsyncNode::connect($s, $t, Empty)
        }
    };
    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use crate::async_graph::*;

            AsyncNode::connect($s, $t, $params)
        }
    };
}

#[macro_export]
macro_rules! async_graph {

	// (Key)
	( ($K:ty) $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use crate::async_graph::*;

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
				if async_nodes_exist(&graph, s, t) {
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
			use crate::async_graph::*;

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
				if async_nodes_exist(&graph, s, t) {
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
			use crate::async_graph::*;

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
				if async_nodes_exist(&graph, s, t) {
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
			use crate::async_graph::*;

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
				if async_nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t, param);
				}
			}
			graph
		}
	};
}
