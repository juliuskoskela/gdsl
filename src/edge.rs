use std::fmt::Display;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::ops::Deref;
use crate::node::*;

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
