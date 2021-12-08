///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	collections::HashMap,
	fmt::{Debug, Display},
	hash::Hash,
	sync::{RwLock, Arc, Weak}
};

use crate::node::*;
use crate::edge::*;
use crate::adjacent::*;
// use fxhash::FxHashMap;

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// CONSTANTS

pub const OPEN: bool = false;
pub const CLOSED: bool = true;

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// TYPES

pub type RefNode<K, N, E> = Arc<Node<K, N, E>>;
pub type WeakNode<K, N, E> = Weak<Node<K, N, E>>;
pub type RefEdge<K, N, E> = Arc<Edge<K, N, E>>;
pub type WeakEdge<K, N, E> = Weak<Edge<K, N, E>>;
pub type RefAdjacent<K, N, E> = RwLock<Adjacent<K, N, E>>;
pub type RefNodePool<K, N, E> = HashMap<K, RefNode<K, N, E>>;
pub type EdgeList<K, N, E> = Vec<WeakEdge<K, N, E>>;

pub enum Return<T> {
	Yes(T),
	No(T)
}

// Helper void type which implements the necessary traits to be used as a
// placeholder for Node parameters which are not used.
#[derive(Clone, Debug)]
pub struct Null;

impl std::fmt::Display for Null {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "_")
	}
}

pub enum Traverse {
	Skip,
	Include,
	Finish,
}

///////////////////////////////////////////////////////////////////////////////

pub fn backtrack_edges<K, N, E>(edges: &EdgeList<K, N, E>) -> Option<EdgeList<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	if edges.len() == 0 {
		return None;
	}
	let mut res = EdgeList::new();
	let w = edges.get(edges.len() - 1).unwrap();
	res.push(w.clone());
	let mut i = 0;
	for edge in edges.iter().rev() {
		let source = res[i].upgrade().unwrap().source();
		if edge.upgrade().unwrap().target() == source {
			res.push(edge.clone());
			i += 1;
		}
	}
	Some(res)
}
