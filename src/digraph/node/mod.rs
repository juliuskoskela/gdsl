//==== Submodules =============================================================

pub mod method;
pub mod order;
pub mod bfs;
pub mod dfs;
pub mod pfs;

//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::digraph::node::order::*;
use crate::digraph::node::dfs::*;
use crate::digraph::node::bfs::*;
use crate::digraph::node::pfs::*;

pub use crate::Empty;
pub use crate::{graph, connect, dinode};

//==== DiNode =================================================================

#[derive(Clone)]
pub struct DiNode<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
	inner: Rc<DiNodeInner<K, N, E>>,
}

struct DiNodeInner<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    key: K,
    value: N,
    edges: RefCell<Adjacent<K, N, E>>,
}

//==== DiNode: Implement ======================================================

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
				edges: RefCell::new(Adjacent::new()),
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
        let edge = DiEdgeInner::new(self, other, value);
        self.edges().borrow_mut().push_outbound(edge.clone());
        other.edges().borrow_mut().push_inbound(edge);
    }

    pub fn disconnect(&self, other: DiNode<K, N, E>) {
        if self.edges().borrow_mut().remove_outbound(&other) {
            other.edges().borrow_mut().remove_inbound(self);
		}
    }

	pub fn isolate(&self) {
		for edge in self.edges().borrow().iter_outbound() {
			edge.target().edges().borrow_mut().remove_inbound(self);
		}
		for edge in self.edges().borrow().iter_inbound() {
			edge.source().edges().borrow_mut().remove_outbound(self);
		}
		self.edges().borrow_mut().clear_outbound();
		self.edges().borrow_mut().clear_inbound();
	}

	pub fn is_root(&self) -> bool {
		self.edges().borrow().inbound().is_empty()
	}

	pub fn is_leaf(&self) -> bool {
		self.edges().borrow().outbound().is_empty()
	}

	pub fn is_orphan(&self) -> bool {
		self.is_root() && self.is_leaf()
	}

	pub fn is_connected(&self, other: &DiNode<K, N, E>) -> bool {
		self.edges()
			.borrow()
			.iter_outbound()
			.find(|&edge| &edge.target() == other)
			.is_some()
	}

	pub fn ordering(&self) -> DirectedOrdering<K, N, E> {
		DirectedOrdering::new(self)
	}

	pub fn dfs(&self) -> DFS<K, N, E> {
		DFS::new(self)
	}

	pub fn bfs(&self) -> BFS<K, N, E> {
		BFS::new(self)
	}

	pub fn pfs(&self) -> PFS<K, N, E>
	where
		N: Ord
	{
		PFS::new(self)
	}

	pub fn iter_outbound(&self) -> DiNodeOutboundIterator<K, N, E> {
		DiNodeOutboundIterator { node: self, position: 0 }
	}

	pub fn iter_inbound(&self) -> DiNodeInboundIterator<K, N, E> {
		DiNodeInboundIterator { node: self, position: 0 }
	}

    //==== Private Methods ====================================================

	fn downgrade(&self) -> WeakDiNode<K, N, E> {
		WeakDiNode {
			inner: Rc::downgrade(&self.inner)
		}
	}

	fn edges(&self) -> &RefCell<Adjacent<K, N, E>> {
		&self.inner.edges
	}
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
		self.inner.upgrade().map(|inner| DiNode { inner })
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

pub struct DiNodeOutboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a DiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for DiNodeOutboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (DiNode<K, N, E>, DiNode<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		let edges = self.node.inner.edges.borrow();
		let edge = edges.outbound().get(self.position);
		match edge {
			Some(edge) => {
				self.position += 1;
				Some((edge.source().clone(), edge.target().clone(), edge.value.clone()))
			}
			None => None,
		}
	}
}

pub struct DiNodeInboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	node: &'a DiNode<K, N, E>,
	position: usize,
}

impl<'a, K, N, E> Iterator for DiNodeInboundIterator<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	type Item = (DiNode<K, N, E>, DiNode<K, N, E>, E);

	fn next(&mut self) -> Option<Self::Item> {
		let edges = self.node.inner.edges.borrow();
		let edge = edges.inbound().get(self.position);
		match edge {
			Some(edge) => {
				self.position += 1;
				Some((edge.source().clone(), edge.target().clone(), edge.value.clone()))
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
	type Item = (DiNode<K, N, E>, DiNode<K, N, E>, E);
	type IntoIter = DiNodeOutboundIterator<'a, K, N, E>;

	fn into_iter(self) -> Self::IntoIter {
		DiNodeOutboundIterator { node: self, position: 0 }
	}
}

//==== DiEdgeInner =================================================================

#[derive(Clone)]
struct DiEdgeInner<K, N = Empty, E = Empty>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    source: WeakDiNode<K, N, E>,
    target: WeakDiNode<K, N, E>,
    value: E,
}

impl<K, N, E> DiEdgeInner<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
    fn new(source: &DiNode<K, N, E>, target: &DiNode<K, N, E>, value: E) -> Self {
		Self {
			value,
			source: source.downgrade(),
			target: target.downgrade(),
		}
    }

	fn source(&self) -> DiNode<K, N, E> {
		self.source.upgrade().unwrap()
	}

	fn target(&self) -> DiNode<K, N, E> {
		self.target.upgrade().unwrap()
	}

	fn value(&self) -> &E {
		&self.value
	}
}

impl<K, N, E> Deref for DiEdgeInner<K, N, E>
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

//==== Adjacency List =========================================================

struct Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	outbound: Vec<DiEdgeInner<K, N, E>>,
	inbound: Vec<DiEdgeInner<K, N, E>>,
}

impl<K, N, E> Adjacent<K, N, E>
where
	K: Clone + Hash + PartialEq + Eq + Display,
	N: Clone,
	E: Clone,
{
	fn new() -> Self {
		Adjacent {
			outbound: Vec::new(),
			inbound: Vec::new(),
		}
	}

	fn outbound(&self) -> &Vec<DiEdgeInner<K, N, E>> {
		&self.outbound
	}

	fn inbound(&self) -> &Vec<DiEdgeInner<K, N, E>> {
		&self.inbound
	}

	fn push_inbound(&mut self, edge: DiEdgeInner<K, N, E>) {
		self.inbound.push(edge);
	}

	fn push_outbound(&mut self, edge: DiEdgeInner<K, N, E>) {
		self.outbound.push(edge);
	}

	fn remove_inbound(&mut self, source: &DiNode<K, N, E>) -> bool {
		let start_len = self.inbound.len();
		self.inbound.retain(|edge| edge.source() != *source);
		start_len != self.inbound.len()
	}

	fn remove_outbound(&mut self, target: &DiNode<K, N, E>) -> bool {
		let start_len = self.outbound.len();
		self.outbound.retain(|e| e.target() != *target);
		start_len != self.outbound.len()
	}

	fn clear_inbound(&mut self) {
		self.inbound.clear();
	}

	fn clear_outbound(&mut self) {
		self.outbound.clear();
	}

	fn iter_outbound(&self) -> std::slice::Iter<DiEdgeInner<K, N, E>> {
		self.outbound.iter()
	}

	fn iter_inbound(&self) -> std::slice::Iter<DiEdgeInner<K, N, E>> {
		self.inbound.iter()
	}
}
