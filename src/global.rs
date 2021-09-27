///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	collections::HashMap,
	cell::RefCell,
	sync::Arc,
	sync::Weak,
};

use crate::node::*;
use crate::edge::*;
use crate::edge_list::*;

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// CONSTANTS

pub const OPEN: bool = false;
pub const CLOSED: bool = true;
pub const TO: i8 = -1;
pub const FROM: i8 = 1;
pub const BI: i8 = 0;

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// TYPES

pub type NodeRef<K, N, E> = Arc<Node<K, N, E>>;
pub type NodeWeak<K, N, E> = Weak<Node<K, N, E>>;
pub type EdgeRef<K, N, E> = Arc<Edge<K, N, E>>;
pub type EdgeWeak<K, N, E> = Weak<Edge<K, N, E>>;
pub type ListRef<K, N, E> = RefCell<Adjacent<K, N, E>>;
pub type NodeRefPool<K, N, E> = HashMap<K, NodeRef<K, N, E>>;

// Helper void type which implements the necessary traits to be used as a
// placeholder for Node parameters which are not used.
#[derive(Clone, Debug)]
pub struct Void;

impl std::fmt::Display for Void {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "_")
	}
}

///////////////////////////////////////////////////////////////////////////////
