use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fmt::Display;
use std::collections::BTreeMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use parking_lot::{RwLock, Mutex};

pub type GraphMap<K, N, E> = BTreeMap<K, GraphNode<K, N, E>>;

const OPEN: bool = false;
const CLOSED: bool = true;

pub enum Traverse {
    Include,
    Exclude,
    Terminate,
}

pub enum Continue<T> {
    Yes(T),
    No(T),
}

// pub enum While<T> {
// 	Continue(T),
// 	Stop(T),
// }

pub struct NodeIter {
	pub map: Box<dyn Fn () -> bool>
}

pub trait Edge<NodeType: Node>: Clone + Send + Sync
{
	// Associated types
	type Params: Clone + Sync + Send;

	// Required implementations
	fn new(source: &NodeType, target: &NodeType, data: Option<Self::Params>) -> Self;
	fn source(&self) -> &NodeType;
	fn target(&self) -> &NodeType;
	fn params(&self) -> &Mutex<Option<Self::Params>>;
	fn lock(&self) -> &AtomicBool;

	// Trivial implementations
	fn get_lock(&self) -> bool { self.lock().load(Ordering::Relaxed) }
	fn close(&self) { self.lock().store(CLOSED, Ordering::Relaxed) }
	fn try_close(&self) -> Result<bool, bool> { self.lock().compare_exchange(OPEN, CLOSED, Ordering::Acquire, Ordering::Relaxed) }
	fn open(&self) { self.lock().store(OPEN, Ordering::Relaxed) }
	fn try_open(&self) -> Result<bool, bool> { self.lock().compare_exchange(CLOSED, OPEN, Ordering::Acquire, Ordering::Relaxed) }
	fn load(&self) -> Option<Self::Params> { self.params().lock().clone() }
	fn store(&self, params: Option<Self::Params>) { *self.params().lock() = params; }
	fn to_tuple(&self) -> (&NodeType, &NodeType, Option<Self::Params>) { (self.source(), self.target(), self.params().lock().clone()) }
}

pub trait Node: PartialEq + Clone + Sync + Send
{
	// Associated types
	type Edge: Edge<Self>;
	type Params: Clone + Sync + Send;
	type Key: Clone + Sync + Send + PartialEq + Display;

	// Required implementations
	fn new(id: Self::Key, params: Option<Self::Params>) -> Self;
	fn params(&self) -> &Mutex<Option<Self::Params>>;
	fn next_edge(&self) -> Option<Self::Edge>;
	fn add_edge(&self, edge: Self::Edge);
	fn remove_edge(&self, edge: &Self::Edge);
	fn find_edge(&self, target: &Self) -> Option<&Self::Edge>;
	fn outbound(&self) -> &RwLock<Vec<Self::Edge>>;
	fn inbound(&self) -> &RwLock<Vec<Self::Edge>>;
	fn lock(&self) -> &AtomicBool;
	fn id(&self) -> &Self::Key;

	// Trivial implementations
	fn get_lock(&self) -> bool { self.lock().load(Ordering::Relaxed) }
	fn close(&self) { self.lock().store(CLOSED, Ordering::Relaxed) }
	fn try_close(&self) -> Result<bool, bool> { self.lock().compare_exchange(OPEN, CLOSED, Ordering::Acquire, Ordering::Relaxed) }
	fn open(&self) { self.lock().store(OPEN, Ordering::Relaxed) }
	fn try_open(&self) -> Result<bool, bool> { self.lock().compare_exchange(CLOSED, OPEN, Ordering::Acquire, Ordering::Relaxed) }
	fn load(&self) -> Option<Self::Params> { self.params().lock().clone() }
	fn store(&self, params: Option<<Self as Node>::Params>) { *self.params().lock() = params; }

	fn connect(&self, target: &Self, params: Option<<<Self as Node>::Edge as Edge<Self>>::Params>) -> Option<Self::Edge> {
		if self.find_outbound(target).is_none() {
			let edge = Self::Edge::new(self, target, params);
			let mut self_outbound = self.outbound().write();
			let mut target_inbound = target.inbound().write();
			self_outbound.push(edge.clone());
			target_inbound.push(edge.clone());
			Some(edge)
		}
		else { None	}
	}

	fn disconnect(&self, target: &Self) -> bool {
		let mut self_outbound = self.outbound().write();
		let mut target_inbound = target.inbound().write();
		let mut found = false;
		for (i, edge) in self_outbound.iter().enumerate() {
			if edge.target() == target {
				edge.close();
				found = true;
				self_outbound.remove(i);
				break;
			}
		}
		if found {
			for (i, edge) in target_inbound.iter().enumerate() {
				if edge.source() == self {
					edge.close();
					target_inbound.remove(i);
					break;
				}
			}
		}
		found
	}

	fn find_outbound(&self, target: &Self) -> Option<Self::Edge> {
		match self
			.outbound()
			.read()
			.iter()
			.find(|edge| edge.target() == target)
		{
			Some(edge) => Some(edge.clone()),
			None => None
		}
	}

	fn find_inbound(&self, source: &Self) -> Option<Self::Edge> {
		match self
			.inbound()
			.read()
			.iter()
			.find(|edge| edge.source() == source)
		{
			Some(edge) => Some(edge.clone()),
			None => None
		}
	}

	fn add_outbound(&self, edge: Self::Edge) {
		self.outbound()
			.write()
			.push(edge.clone());
	}

	fn add_inbound(&self, edge: Self::Edge) {
		self.inbound()
			.write()
			.push(edge.clone());
	}

	fn filter_outbound(&self, predicate: &dyn Fn(&Self::Edge) -> bool) {
		self.outbound()
			.write()
			.retain(predicate);
	}

	fn filter_inbound(&self, predicate: &dyn Fn(&Self::Edge) -> bool) {
		self.inbound()
			.write()
			.retain(predicate);
	}

	fn remove_outbound(&self, edge: Self::Edge) {
		self.filter_outbound(&|e: &Self::Edge| e.source() != edge.source())
	}

	fn remove_inbound(&self, edge: Self::Edge) {
		self.filter_inbound(&|e: &Self::Edge| e.target() != edge.target())
	}

	fn breadth_traversal_map_adjacent<F>(&self, f: F) -> Continue<Vec<Self::Edge>>
	where
		F: Fn(&Self, &Self, &Mutex<Option<<Self::Edge as Edge<Self>>::Params>>) -> Traverse + Sync + Send + Copy,
	{
		let mut result = Vec::<Self::Edge>::new();
		for edge in self.outbound().read().iter() {
			match edge.target().try_close() {
				Ok(_) => {
					match f(edge.source(), edge.target(), edge.params()) {
						Traverse::Include => { result.push(edge.clone()); },
						Traverse::Exclude => { },
						Traverse::Terminate => { result.push(edge.clone()); return Continue::No(result); }
					}
				},
				Err(_) => {}
			}
		}
		Continue::Yes(result)
	}

	fn breadth_traversal<F>(&self, explorer: F) -> Vec<Self::Edge>
	where
		F: Fn(&Self, &Self, &Mutex<Option<<Self::Edge as Edge<Self>>::Params>>) -> Traverse + Sync + Send + Copy,
	{
		let mut frontiers = Vec::new();
		let mut bounds: (usize, usize) = (0, 0);
		let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
		self.close();
		match self.breadth_traversal_map_adjacent(&explorer) {
			Continue::No(segment) => {
				for edge in segment.iter() { edge.source().open(); edge.target().open(); edge.open();
				}
			}
			Continue::Yes(segment) => {
				frontiers = segment;
			}
		}
		loop {
			bounds.1 = frontiers.len();
			if bounds.0 == bounds.1 {
				break;
			}
			let current_frontier = &frontiers[bounds.0..bounds.1];
			bounds.0 = bounds.1;
			let frontier_segments: Vec<_> = current_frontier
				.into_par_iter()
				.map(|edge| {
					match terminate.load(Ordering::Relaxed) {
						true => { None }
						false => {
							let node = edge.target();
							match node.breadth_traversal_map_adjacent(&explorer) {
								Continue::No(segment) => {
									terminate.store(true, Ordering::Relaxed);
									Some(segment)
								}
								Continue::Yes(segment) => Some(segment),
							}
						}
					}
				})
				.while_some()
				.collect();
			for mut segment in frontier_segments {
				frontiers.append(&mut segment);
			}
			if terminate.load(Ordering::Relaxed) == true {
				break;
			}
		}
		for edge in frontiers.iter() { edge.source().open(); edge.target().open(); edge.open();
		}
		return frontiers;
	}
}

pub struct GraphEdgeInner<E, NT>
{
	source: NT,
	target: NT,
	params: Mutex<Option<E>>,
	lock: AtomicBool,
}

#[derive(Clone)]
pub struct GraphEdge<E, NT> { handle: Arc<GraphEdgeInner<E, NT>>, }

impl<E, NT> Display for GraphEdge<E, NT>
where
	NT: Node + PartialEq + Clone + Sync + Send + Display,
	E: Clone + Sync + Send,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} -> {}", self.source(), self.target())
	}
}

impl<E, NT> Edge<NT> for GraphEdge<E, NT>
where
	NT: Node + PartialEq + Clone + Sync + Send,
	E: Clone + Sync + Send,
{
	type Params = E;

	fn new(source: &NT, target: &NT, params: Option<Self::Params>) -> Self {
		let handle = Arc::new(GraphEdgeInner {
			source: source.clone(),
			target: target.clone(),
			params: Mutex::new(params),
			lock: AtomicBool::new(OPEN),
		});
		Self { handle }
	}

	fn source(&self) -> &NT {
		&self.handle.source
	}

	fn target(&self) -> &NT {
		&self.handle.target
	}

	fn params(&self) -> &Mutex<Option<Self::Params>> {
		&self.handle.params
	}

	fn lock(&self) -> &AtomicBool {
		&self.handle.lock
	}
}

struct Adjacent<GraphEdge>
{
	outbound: RwLock<Vec<GraphEdge>>,
	inbound: RwLock<Vec<GraphEdge>>,
}

struct GraphNodeInner<K, N, E>
{
	id : K,
	params: Mutex<Option<N>>,
	adjacent: Adjacent<GraphEdge<E, GraphNode<K, N, E>>>,
	lock: AtomicBool,
}

#[derive(Clone)]
pub struct GraphNode<K, N, E> { handle: Arc<GraphNodeInner<K, N, E>>, }

impl<K, N, E> Display for GraphNode<K, N, E>
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

impl<K: PartialEq, N, E> PartialEq for GraphNode<K, N, E>
{
	fn eq(&self, other: &Self) -> bool {
		self.handle.id == other.handle.id
	}
}

impl<K, N, E> Node for GraphNode<K, N, E>
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + PartialEq + Display
{
	type Edge = GraphEdge<E, Self>;
	type Params = N;
	type Key = K;

	fn new(id: K, params: Option<N>) -> Self {
		let handle = Arc::new(GraphNodeInner {
			id,
			params: Mutex::new(params),
			adjacent: Adjacent {
				outbound: RwLock::new(Vec::new()),
				inbound: RwLock::new(Vec::new()),
			},
			lock: AtomicBool::new(OPEN),
		});
		Self { handle }
	}

	fn next_edge(&self) -> Option<Self::Edge> {
		let adjacent = self.handle.adjacent.outbound.read();
		for edge in adjacent.iter() {
			if edge.lock().load(Ordering::Relaxed) == OPEN {
				return Some(edge.clone());
			}
		}
		None
	}

	fn id(&self) -> &K {
		&self.handle.id
	}

	fn params(&self) -> &Mutex<Option<Self::Params>> {
		&self.handle.params
	}

	fn lock(&self) -> &AtomicBool {
		&self.handle.lock
	}

	fn outbound(&self) -> &RwLock<Vec<Self::Edge>> {
		&self.handle.adjacent.outbound
	}

	fn inbound(&self) -> &RwLock<Vec<Self::Edge>> {
		&self.handle.adjacent.inbound
	}
}


	// fn map_outbound<F>(&self, f: F) -> Vec<Self::Edge>
	// 	where F: Fn(&Self::Edge) -> Option<Option<Self::Edge>>
	// {
	// 	let mut result = Vec::new();
	// 	for edge in self.outbound().read().iter() {
	// 		match edge.target().try_close() {
	// 			Ok(_) => {
	// 				match f(&edge) {
	// 					Some(Some(edge)) => result.push(edge),
	// 					Some(None) => { break; },
	// 					None => ()
	// 				}
	// 			},
	// 			Err(_) => {}
	// 		}
	// 	}
	// 	result
	// }

	// fn map_inbound<F>(&self, f: F) -> Vec<Self::Edge>
	// 	where F: Fn(&Self::Edge) -> Option<Option<Self::Edge>>
	// {
	// 	let mut result = Vec::new();
	// 	for edge in self.inbound().read().iter() {
	// 		match edge.source().try_close() {
	// 			Ok(_) => {
	// 				match f(&edge) {
	// 					Some(Some(edge)) => result.push(edge),
	// 					Some(None) => { break; },
	// 					None => ()
	// 				}
	// 			},
	// 			Err(_) => {}
	// 		}
	// 	}
	// 	result
	// }

	// fn map_adjacent_dir<F>(&self, f: F) -> Continue<Vec<Self::Edge>>
	// 	where F: Fn(&Self::Edge) -> Traverse + Sync + Send + Copy
	// {
	// 	let mut result = Vec::<Self::Edge>::new();
	// 	for edge in self.outbound().read().iter() {
	// 		match edge.target().try_close() {
	// 			Ok(_) => {
	// 				match f(&edge) {
	// 					Traverse::Include => { result.push(edge.clone()); },
	// 					Traverse::Exclude => { },
	// 					Traverse::Terminate => { result.push(edge.clone()); return Continue::No(result); }
	// 				}
	// 			},
	// 			Err(_) => {}
	// 		}
	// 	}
	// 	Continue::Yes(result)
	// }

	// fn par_bf<F>(
	// 	&self,
	// 	explorer: F,
	// ) -> Option<Vec<Self::Edge>>
	// where
	// 	F: Fn(&Self::Edge) -> Traverse + Sync + Send + Copy,
	// {
	// 	let mut frontiers: Vec<Self::Edge>;
	// 	let mut bounds: (usize, usize) = (0, 0);
	// 	let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
	// 	self.close();
	// 	match self.map_adjacent_dir(&explorer) {
	// 		Continue::No(segment) => {
	// 			for edge in segment.iter() { edge.source().open(); edge.target().open(); edge.open(); }
	// 			return Some(segment);
	// 		}
	// 		Continue::Yes(segment) => {
	// 			frontiers = segment;
	// 		}
	// 	}
	// 	loop {
	// 		bounds.1 = frontiers.len();
	// 		if bounds.0 == bounds.1 {
	// 			break;
	// 		}
	// 		let current_frontier = &frontiers[bounds.0..bounds.1];
	// 		bounds.0 = bounds.1;
	// 		let frontier_segments: Vec<_> = current_frontier
	// 			.into_par_iter()
	// 			.map(|edge| {
	// 				match terminate.load(Ordering::Relaxed) {
	// 					true => { None }
	// 					false => {
	// 						let node = edge.target();
	// 						match node.map_adjacent_dir(&explorer) {
	// 							Continue::No(segment) => {
	// 								terminate.store(true, Ordering::Relaxed);
	// 								Some(segment)
	// 							}
	// 							Continue::Yes(segment) => Some(segment),
	// 						}
	// 					}
	// 				}
	// 			})
	// 			.while_some()
	// 			.collect();
	// 		for mut segment in frontier_segments {
	// 			frontiers.append(&mut segment);
	// 		}
	// 		if terminate.load(Ordering::Relaxed) == true {
	// 			break;
	// 		}
	// 	}
	// 	for edge in frontiers.iter() { edge.source().open(); edge.target().open(); edge.open(); }
	// 	if terminate.load(Ordering::Relaxed) == true {
	// 		Some(frontiers)
	// 	} else {
	// 		None
	// 	}
	// }