use std:: {
	collections::HashMap,
	fmt::{Debug, Display},
	hash::Hash,
	sync::{RwLock, Arc, Weak}
};

use crate::node::*;
use crate::edge::*;
use crate::adjacent::*;

pub const OPEN: bool = false;
pub const CLOSED: bool = true;

pub type RefNode<K, N, E> = Arc<Node<K, N, E>>;
pub type WeakNode<K, N, E> = Weak<Node<K, N, E>>;
pub type RefEdge<K, N, E> = Arc<Edge<K, N, E>>;
pub type WeakEdge<K, N, E> = Weak<Edge<K, N, E>>;
pub type RefAdjacent<K, N, E> = RwLock<Adjacent<K, N, E>>;
pub type RefNodePool<K, N, E> = HashMap<K, RefNode<K, N, E>>;
pub type EdgeList<K, N, E> = Vec<WeakEdge<K, N, E>>;

pub enum Continue<T> {
	Yes(T),
	No(T)
}

// Helper void type which implements the necessary traits to be used as a
// placeholder for Node parameters which are not used.
#[derive(Clone, Debug)]
pub struct Empty;

impl std::fmt::Display for Empty {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "_")
	}
}

///////////////////////////////////////////////////////////////////////////////

pub fn backtrack_edges<K, N, E>(edges: &EdgeList<K, N, E>) -> EdgeList<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	let mut res = EdgeList::new();
	if edges.len() == 0 {
		return res;
	}
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
	res
}

// A helper function to open all closed nodes and edges after the algorithm has
// finished.
pub fn open_locks<K, N, E>(result: &EdgeList<K, N, E>)
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    for weak in result.iter() {
        let edge = weak.upgrade().unwrap();
        edge.open();
        edge.target().open();
        edge.source().open();
    }
}
