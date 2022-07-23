use std::fmt::Display;
use std::hash::Hash;
use std::sync::{Arc, Weak};
use parking_lot::RwLock;
use std::ops::Deref;
use std::collections::VecDeque;
use min_max_heap::MinMaxHeap;
use std::collections::HashMap;

pub struct AsyncDiGraph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	nodes: HashMap<K, AsyncDiNode<K, N, E>>,
}

impl<'a, K, N, E> AsyncDiGraph<K, N, E>
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

	pub fn get(&self, key: &K) -> Option<AsyncDiNode<K, N, E>> {
		self.nodes.get(key).map(|node| node.clone())
	}

	pub fn insert(&mut self, node: AsyncDiNode<K, N, E>) -> bool {
		if self.nodes.contains_key(node.key()) {
			false
		} else {
			self.nodes.insert(node.key().clone(), node.clone());
			true
		}
	}

	pub fn remove(&mut self, node: AsyncDiNode<K, N, E>) -> Option<AsyncDiNode<K, N, E>> {
		self.nodes.remove(node.key())
	}

	pub fn roots(&self) -> Vec<AsyncDiNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().read().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn leafs(&self) -> Vec<AsyncDiNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.outbound().read().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn orphans(&self) -> Vec<AsyncDiNode<K, N, E>> {
		self.nodes
			.values()
			.filter(|node| node.inbound().read().is_empty() && node.outbound().read().is_empty())
			.map(|node| node.clone())
			.collect()
	}

	pub fn to_vec(&self) -> Vec<AsyncDiNode<K, N, E>> {
		self.nodes.values().map(|node| node.clone()).collect()
	}
}

impl<'a, K, N, E> std::ops::Index<K> for AsyncDiGraph<K, N, E>
where
	K: Clone + Hash + Display + Eq,
	N: Clone,
	E: Clone,
{
	type Output = AsyncDiNode<K, N, E>;

	fn index(&self, key: K) -> &Self::Output {
		&self.nodes[&key]
	}
}

pub struct WeakAsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<AsyncDiNodeInner<K, N, E>>,
}

impl<K, N, E> WeakAsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	pub fn upgrade(&self) -> Option<AsyncDiNode<K, N, E>> {
		self.handle.upgrade().map(|handle| AsyncDiNode { handle })
	}
}

struct AsyncDiNodeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	key: K,
	params: N,
	outbound: RwLock<Vec<AsyncDiEdge<K, N, E>>>,
	inbound: RwLock<Vec<WeakAsyncDiEdge<K, N, E>>>,
}

#[derive(Clone)]
pub struct AsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Arc<AsyncDiNodeInner<K, N, E>>,
}

impl<K, N, E> AsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(key: K, params: N) -> Self {
		Self {
			handle: Arc::new(AsyncDiNodeInner {
				params,
				key,
				inbound: RwLock::new(vec![]),
				outbound: RwLock::new(vec![]),
			}),
		}
	}

	fn downgrade(&self) -> WeakAsyncDiNode<K, N, E> { WeakAsyncDiNode { handle: Arc::downgrade(&self.handle) } }

	pub fn key(&self) -> &K { &self.handle.key }

	pub fn params(&self) -> &N { &self.handle.params }

	pub fn outbound(&self) -> &RwLock<Vec<AsyncDiEdge<K, N, E>>> { &self.handle.outbound }

	pub fn find_outbound(&self, other: &AsyncDiNode<K, N, E>) -> Option<AsyncDiEdge<K, N, E>> {
		for edge in self.outbound().read().iter() {
			if &edge.target() == other {
				return Some(edge.clone());
			}
		}
		None
	}

	fn delete_outbound(&self, other: &AsyncDiNode<K, N, E>) -> bool {
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

	pub fn inbound(&self) -> &RwLock<Vec<WeakAsyncDiEdge<K, N, E>>> { &self.handle.inbound }

	pub fn find_inbound(&self, other: &AsyncDiNode<K, N, E>) -> Option<AsyncDiEdge<K, N, E>> {
		for edge in self.inbound().read().iter() {
			if &edge.upgrade().unwrap().source() == other {
				return Some(edge.upgrade().unwrap().clone());
			}
		}
		None
	}

	fn delete_inbound(&self, other: &AsyncDiNode<K, N, E>) -> bool {
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

	pub fn connect(&self, other: &AsyncDiNode<K, N, E>, params: E) {
		let edge = AsyncDiEdge::new(self, other.clone(), params);
		self.outbound().write().push(edge.clone());
		other.inbound().write().push(edge.downgrade());
	}

	pub fn try_connect(&self, other: &AsyncDiNode<K, N, E>, params: E) -> bool {
		if self.outbound().read().iter().any(|e| &e.target() == other) {
			return false;
		}
		self.connect(other, params);
		true
	}

	pub fn disconnect(&self, other: AsyncDiNode<K, N, E>) -> bool{
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

	pub fn search(&self) -> AsyncDiNodeSearch<K, N, E> {
		AsyncDiNodeSearch { root: self.clone(), edge_tree: vec![] }
	}
}

pub type Map<'a, K, N, E> = &'a dyn Fn(AsyncDiEdge<K, N, E>) -> bool;

pub struct AsyncDiNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: AsyncDiNode<K, N, E>,
	edge_tree: Vec<AsyncDiEdge<K, N, E>>,
}

impl<K, N, E> AsyncDiNodeSearch<K, N, E>
where
	K: Clone + std::hash::Hash + std::fmt::Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn root(&self) -> AsyncDiNode<K, N, E> {
		self.root.clone()
	}

	pub fn edge_tree(&self) -> &Vec<AsyncDiEdge<K, N, E>> {
		&self.edge_tree
	}

	pub fn edge_tree_mut(&mut self) -> &mut Vec<AsyncDiEdge<K, N, E>> {
		&mut self.edge_tree
	}

	pub fn dfs(&mut self, target: &AsyncDiNode<K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn dfs_map<'a>(&mut self, target: &AsyncDiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self> {
		let mut queue = Vec::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn bfs(&mut self, target: &AsyncDiNode<K, N, E>) -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn bfs_map<'a>(&mut self, target: &AsyncDiNode<K, N, E>, map: Map<'a, K, N, E>)  -> Option<&Self> {
		let mut queue = VecDeque::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn pfs_min(&mut self, target: &AsyncDiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn pfs_min_map<'a>(&mut self, target: &AsyncDiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn pfs_max(&mut self, target: &AsyncDiNode<K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn pfs_max_map<'a>(&mut self, target: &AsyncDiNode<K, N, E>, map: Map<'a, K, N, E>) -> Option<&Self>
	where
		N: Ord,
	{
		let mut queue = MinMaxHeap::new();
		let mut visited = AsyncDiGraph::new();

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

	pub fn edge_path(&self) -> Vec<AsyncDiEdge<K, N, E>> {
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

	pub fn node_path(&self) -> Vec<AsyncDiNode<K, N, E>> {
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
// Blanket implementations for AsyncDiGraph<K, N, E>
///////////////////////////////////////////////////////////////////////////////

impl<K, N, E> Deref for AsyncDiNode<K, N, E>
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

impl<K, N, E> PartialEq for AsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn eq(&self, other: &Self) -> bool {
		self.key() == other.key()
	}
}


impl<K, N, E> Eq for AsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{ }

impl<K, N, E> PartialOrd for AsyncDiNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.params().cmp(&other.params()))
	}
}

impl<K, N, E> Ord for AsyncDiNode<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + Ord,
	E: Clone,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.params().cmp(&other.params())
	}
}

pub struct AsyncDiNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: AsyncDiNode<K, N, E>,
	position: usize,
}

impl<K, N, E> Iterator for AsyncDiNodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncDiEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().read().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().read()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for AsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncDiEdge<K, N, E>;
	type IntoIter = AsyncDiNodeIntoIterator<K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		AsyncDiNodeIntoIterator { node: self, position: 0 }
	}
}

pub struct AsyncDiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a AsyncDiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for AsyncDiNodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncDiEdge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.position >= self.node.outbound().read().len() {
			None
		} else {
			self.position += 1;
			Some(self.node.outbound().read()[self.position - 1].clone())
		}
	}
}

impl<'a, K, N, E> IntoIterator for &'a AsyncDiNode<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = AsyncDiEdge<K, N, E>;
	type IntoIter = AsyncDiNodeIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		AsyncDiNodeIterator { node: self, position: 0 }
	}
}

// EDGE

struct AsyncDiEdgeInner<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	params: E,
	source: WeakAsyncDiNode<K, N, E>,
	target: AsyncDiNode<K, N, E>,
}

#[derive(Clone)]
pub struct AsyncDiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Arc<AsyncDiEdgeInner<K, N, E>>,
}

impl<K, N, E> AsyncDiEdge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn new(source: &AsyncDiNode<K, N, E>, target: AsyncDiNode<K, N, E>, params: E) -> Self {
		let handle = Arc::new(AsyncDiEdgeInner {
			params,
			source: source.downgrade(),
			target: target.clone(),
		});
		Self { handle }
	}

	pub fn downgrade(&self) -> WeakAsyncDiEdge<K, N, E> {
		WeakAsyncDiEdge { handle: Arc::downgrade(&self.handle) }
	}

	pub fn source(&self) -> AsyncDiNode<K, N, E> {
		self.handle.source.upgrade().unwrap().clone()
	}

	pub fn target(&self) -> AsyncDiNode<K, N, E> {
		self.handle.target.clone()
	}

	pub fn params(&self) -> &E {
		&self.handle.params
	}
}

impl<K, N, E> Deref for AsyncDiEdge<K, N, E>
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

pub struct WeakAsyncDiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	handle: Weak<AsyncDiEdgeInner<K, N, E>>,
}

impl<K, N, E> WeakAsyncDiEdge<K, N, E>
where
	K: Clone + Hash + Display,
	N: Clone,
	E: Clone,
{
	fn upgrade(&self) -> Option<AsyncDiEdge<K, N, E>> {
		self.handle.upgrade().map(|handle| AsyncDiEdge { handle })
	}
}

pub fn async_nodes_exist<K, N, E>(g: &AsyncDiGraph<K, N, E>, s: K, t: K) -> bool
where
	K: std::fmt::Debug + std::fmt::Display + Hash + Eq + Clone + PartialEq,
	N: std::fmt::Debug + Clone,
	E: Clone,
{
	if !g.contains(&s) && !g.contains(&t) {
		panic!("Check your macro invocation: {} and {} are not in the DiGraph", s, t);
	} else if !g.contains(&s) {
		panic!("Check your macro invocation: {} is not in the DiGraph", s);
	} else if !g.contains(&t) {
		panic!("Check your macro invocation: {} is not in the DiGraph", t);
	} else {
		true
	}
}
