use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use crate::core::Empty;
use std::hash::Hash;
use std::fmt::Display;
use std::collections::BTreeMap;
use log::debug;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
// use parking_lot::{RwLock, Mutex, RwLockReadGuard, RwLockWriteGuard};

pub type GraphMap<K, N, E> = BTreeMap<K, GraphNode<K, N, E>>;

const OPEN: bool = false;
const CLOSED: bool = true;

pub enum Which<T> {
	Left(T),
	Right(T),
}

pub enum Either<A, B> {
	This(A),
	That(B),
}

pub enum Collect<N: Node> {
	Yes(Travel<N>),
	No(Travel<N>)
}

pub enum Travel<N: Node> {
	NextDepth,
	NextBreadth,
	Prev,
	Goto(N),
	Stop
}

// pub enum Traverse<GraphNode>
// 	where GraphNode: Node
// {
// 	Include,
// 	Exclude,
// 	Switch(GraphNode),
// 	Terminate,
// }

// pub type Explorer<N: Node> = dyn Fn(&N) -> Collect<N>;

pub trait Edge<NT>: Clone {
	type Params: Clone + Sync + Send;

	fn new(source: &NT, target: &NT, data: Option<Self::Params>) -> Self;
	fn source(&self) -> &NT;
	fn target(&self) -> &NT;
	fn params(&self) -> &Mutex<Option<Self::Params>>;
	fn lock(&self) -> &AtomicBool;

	fn get_lock(&self) -> bool {
		let lock = self.lock();
		lock.load(Ordering::Relaxed)
	}

	fn close(&self) {
		let lock = self.lock();
		lock.store(CLOSED, Ordering::Relaxed)
	}

	fn try_close(&self) -> Result<bool, bool> {
		let lock = self.lock();
		lock.compare_exchange(OPEN, CLOSED, Ordering::AcqRel, Ordering::Relaxed)
	}

	fn open(&self) {
		let lock = self.lock();
		lock.store(OPEN, Ordering::Relaxed)
	}

	fn try_open(&self) -> Result<bool, bool> {
		let lock = self.lock();
		lock.compare_exchange(CLOSED, OPEN, Ordering::Acquire, Ordering::Relaxed)
	}

	fn load(&self) -> Option<Self::Params> {
		let params = self.params();
		let params = params.lock();
		let params = params.unwrap();
		params.clone()
	}

	fn store(&self, params: Option<Self::Params>) {
		let self_params = self.params();
		let self_params = self_params.lock();
		let mut self_params = self_params.unwrap();
		*self_params = params;
	}
}

pub trait Node: PartialEq + Clone + Sync + Send
{
	type Edge: Edge<Self> + Clone + Sync + Send;
	type Params: Clone + Sync + Send;
	type Key: Clone + Sync + Send + Hash + PartialEq + Display;

	fn new(id: Self::Key, params: Option<Self::Params>) -> Self;
	fn params(&self) -> &Mutex<Option<Self::Params>>;
	fn outbound(&self) -> &RwLock<Vec<Self::Edge>>;
	fn inbound(&self) -> &RwLock<Vec<Self::Edge>>;
	fn lock(&self) -> &AtomicBool;
	fn id(&self) -> &Self::Key;
	fn duplicate(&self) -> Self;

	fn get_lock(&self) -> bool {
		let lock = self.lock();
		lock.load(Ordering::Relaxed)
	}

	fn close(&self) {
		let lock = self.lock();
		lock.store(CLOSED, Ordering::Relaxed)
	}

	fn try_close(&self) -> Result<bool, bool> {
		let lock = self.lock();
		lock.compare_exchange(OPEN, CLOSED, Ordering::Acquire, Ordering::Relaxed)
	}

	fn open(&self) {
		let lock = self.lock();
		lock.store(OPEN, Ordering::Relaxed)
	}

	fn try_open(&self) -> Result<bool, bool> {
		let lock = self.lock();
		lock.compare_exchange(CLOSED, OPEN, Ordering::Acquire, Ordering::Relaxed)
	}

	fn load(&self) -> Option<Self::Params> {
		let params = self.params();
		let params = params.lock();
		let params = params.unwrap();
		params.clone()
	}

	fn store(&self, params: Option<<Self as Node>::Params>) {
		let self_params = self.params();
		let self_params = self_params.lock();
		let mut self_params = self_params.unwrap();
		*self_params = params;
	}

	fn connect(&self, target: &Self, params: Option<<<Self as Node>::Edge as Edge<Self>>::Params>) -> Option<Self::Edge> {
		if !self.find_outbound(target).is_some() {
			let edge = Self::Edge::new(self, target, params);
			let mut self_outbound = self.outbound().write().unwrap();
			let mut target_inbound = target.inbound().write().unwrap();
			self_outbound.push(edge.clone());
			target_inbound.push(edge.clone());
			Some(edge)
		}
		else { None	}
	}

	fn disconnect(&self, target: &Self) -> bool {
		let mut self_outbound = self.outbound().write().unwrap();
		let mut target_inbound = target.inbound().write().unwrap();
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
			.unwrap()
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
			.unwrap()
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
			.unwrap()
			.push(edge.clone());
	}

	fn add_inbound(&self, edge: Self::Edge) {
		self.inbound()
			.write()
			.unwrap()
			.push(edge.clone());
	}

	fn filter_outbound(&self, predicate: &dyn Fn(&Self::Edge) -> bool) {
		self.outbound()
			.write()
			.unwrap()
			.retain(predicate);
	}

	fn filter_inbound(&self, predicate: &dyn Fn(&Self::Edge) -> bool) {
		self.inbound()
			.write()
			.unwrap()
			.retain(predicate);
	}

	fn remove_outbound(&self, edge: Self::Edge) {
		self.filter_outbound(&|e: &Self::Edge| e.source() != edge.source())
	}

	fn remove_inbound(&self, edge: Self::Edge) {
		self.filter_inbound(&|e: &Self::Edge| e.target() != edge.target())
	}

	fn map_outbound<F>(&self, f: F) -> Vec<Self::Edge>
		where F: Fn(&Self::Edge) -> Option<Option<Self::Edge>>
	{
		let mut result = Vec::new();
		for edge in self.outbound().read().unwrap().iter() {
			match edge.target().try_close() {
				Ok(_) => {
					match f(&edge) {
						Some(Some(edge)) => result.push(edge),
						Some(None) => { break; },
						None => ()
					}
				},
				Err(_) => {}
			}
		}
		result
	}

	fn map_inbound<F>(&self, f: F) -> Vec<Self::Edge>
		where F: Fn(&Self::Edge) -> Option<Option<Self::Edge>>
	{
		let mut result = Vec::new();
		for edge in self.inbound().read().unwrap().iter() {
			match edge.source().try_close() {
				Ok(_) => {
					match f(&edge) {
						Some(Some(edge)) => result.push(edge),
						Some(None) => { break; },
						None => ()
					}
				},
				Err(_) => {}
			}
		}
		result
	}

	// pub fn parallel_directed_breadth_traversal<K, N, E, F>(
	// 	source: &Arc<Node<K, N, E>>,
	// 	explorer: F,
	// ) -> Option<Vec<Weak<Edge<K, N, E>>>>
	// where
	// 	K: Hash + Eq + Clone + Display + Sync + Send,
	// 	N: Clone + Display + Sync + Send,
	// 	E: Clone + Display + Sync + Send,
	// 	F: Fn(&Arc<Edge<K, N, E>>) -> Traverse + Sync + Send + Copy,
	// {
	// 	let mut frontiers: Vec<Weak<Edge<K, N, E>>>;
	// 	let mut bounds: (usize, usize) = (0, 0);
	// 	let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
	// 	source.close();
	// 	match source.map_adjacent_dir(&explorer) {
	// 		Continue::No(segment) => {
	// 			open_locks(&segment);
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
	// 						let node = edge.upgrade().unwrap().target();
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
	// 	open_locks(&frontiers);
	// 	if terminate.load(Ordering::Relaxed) == true {
	// 		Some(frontiers)
	// 	} else {
	// 		None
	// 	}
	// }

	fn explore_recursion<'a>(
		node: &'a Self,
		result: &'a mut Vec<Self::Edge>,
		position: usize,
		f: &dyn Fn (&Self::Edge) -> Collect<Self>,
	) -> &'a Vec<Self::Edge> {
		node.close();
		let outbound_len = node.outbound().read().unwrap().len();
		for (i, edge) in node.outbound().read().unwrap().iter().enumerate() {
			match edge.try_close() {
				Ok(_) => {
					match f(edge) {
						Collect::Yes(travel) => {
							result.push(edge.clone());
							match travel {
								Travel::NextDepth => {
									Self::explore_recursion(edge.target(), result, position, f);
								},
								Travel::NextBreadth => {
									if i == outbound_len {
										println!("I'M GOING TO THE NEXT NODE => {}!", result[position].target().id());
										if result[position].target().get_lock() == OPEN {
											Self::explore_recursion(&result[position].target().clone(), result, position + 1, f);
										}
									}
									else if position == result.len() + outbound_len - i {
										println!("I HAVE EXPLORED ALL EDGES OF THIS NODE!");
										break
									}
									else {
										println!("I'M EXPLORING EDGES OF NODE {}", node.id());
										continue
									}
								},
								Travel::Prev => {
									let prev = result.pop();
									match prev {
										Some(prev) => {
											Self::explore_recursion(prev.source(), result, position, f);
										},
										None => { break }
									}
									Self::explore_recursion(node, result, position, f);
								},
								Travel::Goto(node) => { Self::explore_recursion(&node, result, position, f); },
								Travel::Stop => { println!("I'M STOPPING!"); return result; }
							}
						},
						Collect::No(travel) => {
							match travel {
								Travel::NextDepth => {
									Self::explore_recursion(edge.target(), result, position, f);
								},
								Travel::NextBreadth => {
									if i == outbound_len - 1 && position < result.len() + outbound_len - i {
										Self::explore_recursion(&result[position].target().clone(), result, position + 1, f);
									}
									else if position == result.len() + outbound_len - i { break }
									else { continue }
								},
								Travel::Prev => {
									let prev = result.pop();
									match prev {
										Some(prev) => {
											Self::explore_recursion(prev.source(), result, position, f);
										},
										None => { break }
									}
									Self::explore_recursion(node, result, position, f);
								},
								Travel::Goto(node) => { Self::explore_recursion(&node, result, position, f); },
								Travel::Stop => { break }
							}
						},
					}
				},
				Err(_) => { continue }
			}
		}
		result
	}

	fn explore(&self, f: &dyn Fn (&Self::Edge) -> Collect<Self>) -> Vec<Self::Edge>
	{
		let mut result = Vec::new();
		Self::explore_recursion(self, &mut result, 0, f);
		result
	}

	fn breadth_first_search(&self, target: &Self) -> Option<Vec<Self::Edge>>
	{
		let edges = self.explore(&| edge: &Self::Edge | {
			if edge.target() == target {
				Collect::Yes(Travel::Stop)
			}
			else {
				Collect::Yes(Travel::NextBreadth)
			}
		});

		let mut res = Vec::new();
		if edges.len() == 0 && target != edges.last().unwrap().target() {
			return None;
		}
		let w = edges.get(edges.len() - 1).unwrap();
		res.push(w.clone());
		let mut i = 0;
		for edge in edges.iter().rev() {
			let source = res[i].source();
			if edge.target() == source {
				res.push(edge.clone());
				i += 1;
			}
		}
		res.reverse();
		Some(res)
	}

	fn depth_first_search(&self, target: &Self) -> Vec<Self::Edge>
	{
		self.explore(&| edge: &Self::Edge | {
			if edge.target() == target {
				Collect::Yes(Travel::Stop)
			}
			else {
				Collect::Yes(Travel::NextDepth)
			}
		})
	}
}

pub struct GraphEdgeInner<E, NT>
where
	NT: PartialEq + Clone + Sync + Send
{
	source: NT,
	target: NT,
	params: Mutex<Option<E>>,
	lock: AtomicBool,
}

#[derive(Clone)]
pub struct GraphEdge<E, NT>
where
	NT: Node + PartialEq + Clone + Sync + Send,
	E: Clone + Sync + Send,
{
	handle: Arc<GraphEdgeInner<E, NT>>,
}

impl<E, NT> std::fmt::Display for GraphEdge<E, NT>
where
	NT: Node + PartialEq + Clone + Sync + Send + std::fmt::Display,
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
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + Hash + PartialEq + Display
{
	id : K,
	params: Mutex<Option<N>>,
	adjacent: Adjacent<GraphEdge<E, GraphNode<K, N, E>>>,
	lock: AtomicBool,
}

pub struct GraphNode<K, N, E>
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + Hash + PartialEq + Display
{
	handle: Arc<GraphNodeInner<K, N, E>>,
}

impl<K, N, E> std::fmt::Display for GraphNode<K, N, E>
where
	E: Clone + Sync + Send + std::fmt::Display,
	N: Clone + Sync + Send + std::fmt::Display,
	K: Clone + Sync + Send + Hash + PartialEq + Display
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut outbound = String::new();
		for edge in self.outbound().read().unwrap().iter() {
			outbound.push_str(&format!(" {},", edge.target().id()));
		}
		outbound.pop();
		write!(f, "{} => [{} ]", self.handle.id, outbound)
	}
}

impl<K, N, E> Clone for GraphNode<K, N, E>
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + Hash + PartialEq + Display
{
	fn clone(&self) -> Self {
		GraphNode {
			handle: self.handle.clone()
		}
	}
}

impl<K, N, E> PartialEq for GraphNode<K, N, E>
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + Hash + PartialEq + Display
{
	fn eq(&self, other: &Self) -> bool {
		self.handle.id == other.handle.id
	}
}

impl<K, N, E> Node for GraphNode<K, N, E>
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + Hash + PartialEq + Display
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

	fn duplicate(&self) -> Self {
		let handle = Arc::new(GraphNodeInner {
			id: self.handle.id.clone(),
			params: Mutex::new(self.params().lock().unwrap().clone()),
			adjacent: Adjacent {
				outbound: RwLock::new(self.outbound().read().unwrap().iter().map(|e| e.clone()).collect()),
				inbound: RwLock::new(self.inbound().read().unwrap().iter().map(|e| e.clone()).collect()),
			},
			lock: AtomicBool::new(OPEN),
		});
		GraphNode {
			handle
		}
	}
}
