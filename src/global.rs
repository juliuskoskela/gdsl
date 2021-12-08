///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	collections::HashMap,
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
