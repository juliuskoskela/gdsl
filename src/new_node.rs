use std::fmt::Display;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::Deref;

const OPEN: bool = false;
const CLOSED: bool = true;

// NODE

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
	lock: AtomicBool,
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
				lock: AtomicBool::new(OPEN),
			}),
		}
	}

	pub fn downgrade(&self) -> WeakNode<K, N, E> {
		WeakNode {
			handle: Rc::downgrade(&self.handle),
		}
	}

	pub fn key(&self) -> &K {
		&self.handle.key
	}

	pub fn params(&self) -> &N {
		&self.handle.params
	}

	pub fn inbound(&self) -> &RefCell<Vec<WeakEdge<K, N, E>>> {
		&self.handle.inbound
	}

	pub fn outbound(&self) -> &RefCell<Vec<Edge<K, N, E>>> {
		&self.handle.outbound
	}

	pub fn open(&self) {
		self.handle.lock.store(OPEN, Ordering::Relaxed);
	}

	pub fn try_open(&self) -> Option<Self> {
		let res = self.handle.lock.compare_exchange(OPEN, CLOSED, Ordering::Relaxed, Ordering::Relaxed);
		match res {
			Ok(_) => Some(self.clone()),
			Err(_) => None,
		}
	}

	pub fn close(&self) {
		self.handle.lock.store(CLOSED, Ordering::Relaxed);
	}

	pub fn try_close(&self) -> Option<Self> {
		let res = self.handle.lock.compare_exchange(CLOSED, OPEN, Ordering::Relaxed, Ordering::Relaxed);
		match res {
			Ok(_) => Some(self.clone()),
			Err(_) => None,
		}
	}

	pub fn connect(&self, other: &Node<K, N, E>, params: E) {
		let edge = Edge::new(self, other.clone(), params);
		self.outbound().borrow_mut().push(edge.clone());
		other.inbound().borrow_mut().push(edge.downgrade());
	}

	pub fn try_connect(&self, other: &Node<K, N, E>, params: E) -> bool {
		if self.outbound().borrow().iter().any(|e| e.target().key() == other.key()) {
			return false;
		}
		self.connect(other, params);
		true
	}

	pub fn disconnect(&self, other: Node<K, N, E>) -> bool{
		let mut outbound = self.outbound().borrow_mut();
		let mut inbound = other.inbound().borrow_mut();
		let mut found = false;
		let mut i = 0;
		while i < outbound.len() {
			if outbound[i].target().key() == other.key() {
				outbound.remove(i);
				found = true;
				i -= 1;
			}
			i += 1;
		}
		i = 0;
		while i < inbound.len() {
			if inbound[i].upgrade().unwrap().source().key() == self.key() {
				inbound.remove(i);
				i -= 1;
			}
			i += 1;
		}
		found
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
		&self.params()
	}
}

impl<K, N, E> PartialEq for Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone + PartialEq,
	E: Clone,
{
	fn eq(&self, other: &Self) -> bool {
		self.params() == other.params()
	}
}


impl<K, N, E> Eq for Node<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone + PartialEq,
	E: Clone,
{

}

impl<K, N, E> PartialOrd for Node<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + PartialEq + Ord,
	E: Clone,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.params().cmp(&other.params()))
	}
}

impl<K, N, E> Ord for Node<K, N, E>
where
	K: Clone + Hash + PartialEq + Display + Eq,
	N: Clone + PartialEq + Ord,
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
}

impl<K, N, E> Iterator for NodeIntoIterator<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = Edge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		let outbound = self.node.outbound().borrow();
		for edge in outbound.iter() {
			match edge.target().try_close() {
				Some(_) => return Some(edge.clone()),
				None => (),
			}
		}
		None
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
		NodeIntoIterator { node: self }
	}
}

pub struct NodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a Node<K, N, E>,
}

impl<'a, K, N, E> Iterator for NodeIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = Edge<K, N, E>;

	fn next(&mut self) -> Option<Self::Item> {
		let outbound = self.node.outbound().borrow();
		for edge in outbound.iter() {
			match edge.target().try_close() {
				Some(_) => return Some(edge.clone()),
				None => (),
			}
		}
		None
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
		NodeIterator { node: self }
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


impl<K, N, E> Edge<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn new(source: &Node<K, N, E>, target: Node<K, N, E>, params: E) -> Self {
		let handle = Rc::new(EdgeInner {
			params,
			source: source.downgrade(),
			target: target.clone(),
		});
		Self { handle }
	}

	pub fn downgrade(&self) -> WeakEdge<K, N, E> {
		WeakEdge {
			handle: Rc::downgrade(&self.handle),
		}
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
	pub fn upgrade(&self) -> Option<Edge<K, N, E>> {
		self.handle.upgrade().map(|handle| Edge { handle })
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

impl<K, N, E> Graph<K, N, E>
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

	pub fn remove(&mut self, key: &K) -> Option<Node<K, N, E>> {
		self.nodes.remove(key).map(|node| node.clone())
	}

	pub fn update(&mut self, node: Node<K, N, E>) -> Option<Node<K, N, E>> {
		self.nodes.insert(node.key().clone(), node).map(|node| node.clone())
	}

	pub fn traverse(&self) -> GraphTraverse<K, N, E> {
		GraphTraverse {
			traversal: Vec::new(),
			position: 0,
		}
	}

	pub fn priority_traverse(&self, root: K) -> GraphPriorityTraverse<K, N, E>
	where
		N: Ord
	{
		let root = self.get(&root).unwrap();
		let mut min_heap = MinHeap::new();
		min_heap.push(root.clone());
		GraphPriorityTraverse {
			traversal: Vec::new(),
			min_heap: MinHeap::new(),
		}
	}
}

impl<K, N, E> std::ops::Index<K> for Graph<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Output = Node<K, N, E>;

	fn index(&self, key: K) -> &Self::Output {
		&self.nodes[&key]
	}
}

pub trait Traversal<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	fn traversed(&self) -> Vec<Edge<K, N, E>>;
	fn next(&mut self) -> Option<Node<K, N, E>>;
	fn curr(&self) -> Option<Node<K, N, E>>;
}

pub struct GraphTraverse<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	traversal: Vec<Edge<K, N, E>>,
	position: usize,
}

impl<K, N, E> GraphTraverse<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn adjacent(&mut self, node: Node<K, N, E>, f: &dyn Fn (&Edge<K, N, E>) -> bool) -> Option<Edge<K, N, E>> {
		for edge in &node {
			match edge.target().try_close() {
				Some(_) => {
					match f(&edge) {
						true => {
							self.traversal.push(edge.clone());
							return Some(edge.clone())
						},
						false => (),
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn next(&mut self) -> Option<Node<K, N, E>> {
		if self.position == self.traversal.len() {
			for edge in self.traversal.iter() {
				edge.source().open();
				edge.target().open();
			}
			None
		} else {
			self.position += 1;
			Some(self.traversal[self.position - 1].target().clone())
		}
	}
}

use crate::heap::*;

pub struct GraphPriorityTraverse<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone + Ord,
	E: Clone,
{
	traversal: Vec<Edge<K, N, E>>,
	min_heap: MinHeap<Node<K, N, E>>,
}

impl<K, N, E> GraphPriorityTraverse<K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone + Ord,
	E: Clone,
{
	pub fn next_adjacent(&mut self, node: &Node<K, N, E>, f: &dyn Fn (Edge<K, N, E>) -> bool) -> Option<Edge<K, N, E>> {
		for edge in node.outbound().borrow().iter() {
			match edge.target().try_close() {
				Some(_) => {
					match f(edge.clone()) {
						true => {
							self.traversal.push(edge.clone());
							return Some(edge.clone())
						},
						false => (),
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn next(&mut self) -> Option<Node<K, N, E>> {
		self.min_heap.pop()
	}
}