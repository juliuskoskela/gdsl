use crate::node::*;
use crate::edge::*;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::fmt::Display;
use parking_lot::{RwLock, Mutex};

pub struct EdgeInner<E, NT> {
	source: NT,
	target: NT,
	params: Mutex<E>,
}

#[derive(Clone)]
pub struct Edge<E, NT> { handle: Arc<EdgeInner<E, NT>>, }

impl<E, NT> Display for Edge<E, NT>
where
	NT: GraphNode + PartialEq + Clone + Sync + Send + Display,
	E: Clone + Sync + Send+ Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
		where E: Display
	{
		write!(f, "{} -> {}", self.source(), self.target())
	}
}

impl<E, NT> GraphEdge<NT> for Edge<E, NT>
where
	NT: GraphNode + PartialEq + Clone + Sync + Send,
	E: Clone + Sync + Send + Display,
{
	type Params = E;

	fn new(source: &NT, target: &NT, params: Self::Params) -> Self {
		let handle = Arc::new(EdgeInner {
			source: source.clone(),
			target: target.clone(),
			params: Mutex::new(params),
		});
		Self { handle }
	}

	fn source(&self) -> &NT {
		&self.handle.source
	}

	fn target(&self) -> &NT {
		&self.handle.target
	}

	fn params(&self) -> &Mutex<Self::Params> {
		&self.handle.params
	}
}

#[derive(Clone)]
pub struct Node<K, N, E> { handle: Arc<NodeInner<K, N, E>> }

struct NodeInner<K, N, E> {
	id : K,
	params: Mutex<N>,
	outbound: RwLock<Vec<Edge<E, Node<K, N, E>>>>,
	inbound: RwLock<Vec<Edge<E, Node<K, N, E>>>>,
	lock: AtomicBool,
}

impl<K, N, E> GraphNode for Node<K, N, E>
where
	K: Clone + Sync + Send + PartialEq + Display,
	E: Clone + Sync + Send + Display,
	N: Clone + Sync + Send + Display,
{
	type Edge = Edge<E, Self>;
	type Params = N;
	type Key = K;

	fn new(id: K, params: N) -> Self {
		Self {
			handle: Arc::new(NodeInner {
				id,
				params: Mutex::new(params),
				outbound: RwLock::new(Vec::new()),
				inbound: RwLock::new(Vec::new()),
				lock: AtomicBool::new(false),
			}
		)}
	}

	fn id(&self) -> &K {
		&self.handle.id
	}

	fn params(&self) -> &Mutex<Self::Params> {
		&self.handle.params
	}

	fn lock(&self) -> &AtomicBool {
		&self.handle.lock
	}

	fn outbound(&self) -> &RwLock<Vec<Self::Edge>> {
		&self.handle.outbound
	}

	fn inbound(&self) -> &RwLock<Vec<Self::Edge>> {
		&self.handle.inbound
	}
}

impl<K, N, E> Display for Node<K, N, E>
where
	E: Clone + Sync + Send + Display,
	N: Clone + Sync + Send + Display,
	K: Clone + Sync + Send + PartialEq + Display
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut outbound = String::new();
		for edge in self.outbound().read().iter() {
			outbound.push_str(&format!(" {},", edge.target().id()));
		}
		outbound.pop();
		write!(f, "{} => [{} ]", self.handle.id, outbound)
	}
}

impl<K: PartialEq, N, E> PartialEq for Node<K, N, E> {
	fn eq(&self, other: &Self) -> bool {
		self.handle.id == other.handle.id
	}
}

impl<K: PartialEq, N, E> Eq for Node<K, N, E> { }

impl<K: PartialEq, N, E> PartialOrd for Node<K, N, E>
where
	E: Clone + Sync + Send + Display,
	N: Clone + Sync + Send + Display + PartialOrd,
	K: Clone + Sync + Send + PartialEq + Display
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.load() < other.load() {
			Some(std::cmp::Ordering::Less)
		} else if self.load() > other.load() {
			Some(std::cmp::Ordering::Greater)
		} else {
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl<K: PartialEq, N, E> Ord for Node<K, N, E>
where
	E: Clone + Sync + Send + Display,
	N: Clone + Sync + Send + Display + PartialOrd,
	K: Clone + Sync + Send + PartialEq + Display
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.load() < other.load() {
			std::cmp::Ordering::Less
		} else if self.load() > other.load() {
			std::cmp::Ordering::Greater
		} else {
			std::cmp::Ordering::Equal
		}
	}
}
