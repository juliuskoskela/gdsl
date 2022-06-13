use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};
use std::fmt::Display;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use parking_lot::{RwLock, Mutex};

const OPEN: bool = false;
const CLOSED: bool = true;

pub enum Sig {
	Continue,
	Terminate,
}

pub enum Coll {
	Include,
	Exclude,
}

pub enum Move<N: GraphNode> {
	Next,
	Prev,
	Jump(N),
}

#[derive(Clone, Debug)]
pub struct Empty;

impl Display for Empty {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "_")
    }
}

pub trait GraphEdge<N: GraphNode>: Clone + Send + Sync
{
	// Associated types
	type Params: Clone + Sync + Send + Display;

	// Required implementations
	fn new(source: &N, target: &N, data: Self::Params) -> Self;
	fn source(&self) -> &N;
	fn target(&self) -> &N;
	fn params(&self) -> &Mutex<Self::Params>;
	fn lock(&self) -> &AtomicBool;

	// Trivial implementations
	fn get_lock(&self) -> bool { self.lock().load(Ordering::Relaxed) }
	fn close(&self) { self.lock().store(CLOSED, Ordering::Relaxed) }
	fn try_close(&self) -> Result<bool, bool> { self.lock().compare_exchange(OPEN, CLOSED, Ordering::Acquire, Ordering::Relaxed) }
	fn open(&self) { self.lock().store(OPEN, Ordering::Relaxed) }
	fn try_open(&self) -> Result<bool, bool> { self.lock().compare_exchange(CLOSED, OPEN, Ordering::Acquire, Ordering::Relaxed) }
	fn load(&self) -> Self::Params { self.params().lock().clone() }
	fn store(&self, params: Self::Params) { *self.params().lock() = params; }
	fn to_tuple(&self) -> (&N, &N, Self::Params) { (self.source(), self.target(), self.params().lock().clone()) }
}

pub trait GraphNode: PartialEq + Clone + Sync + Send
{
	// Associated types
	type Edge: GraphEdge<Self>;
	type Params: Clone + Sync + Send + Display;
	type Key: Clone + Sync + Send + PartialEq + Display;

	// Required implementations
	fn new(id: Self::Key, params: Self::Params) -> Self;
	fn params(&self) -> &Mutex<Self::Params>;
	fn outbound(&self) -> &RwLock<Vec<Self::Edge>>;
	fn inbound(&self) -> &RwLock<Vec<Self::Edge>>;
	fn lock(&self) -> &AtomicBool;
	fn id(&self) -> &Self::Key;

	// Trivial implementations
	fn get_lock(&self) -> bool { self.lock().load(Ordering::Relaxed) }
	fn close(&self) { self.lock().store(CLOSED, Ordering::Relaxed) }
	fn try_close(&self) -> Result<bool, bool> {self.lock().compare_exchange(OPEN, CLOSED, Ordering::Acquire, Ordering::Relaxed) }
	fn open(&self) { self.lock().store(OPEN, Ordering::Relaxed) }
	fn try_open(&self) -> Result<bool, bool> { self.lock().compare_exchange(CLOSED, OPEN, Ordering::Acquire, Ordering::Relaxed) }
	fn load(&self) -> Self::Params { self.params().lock().clone() }
	fn store(&self, params: <Self as GraphNode>::Params) { *self.params().lock() = params; }

	// Node connections
	fn connect(&self, target: &Self, params: <<Self as GraphNode>::Edge as GraphEdge<Self>>::Params) -> Option<Self::Edge> {
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

	fn isolate(&self) {
		for edge in self.inbound().read().iter() {
			edge.target().disconnect(self);
		}
		self.outbound().write().clear();
	}

	// Edge list operations
	fn find_outbound(&self, target: &Self) -> Option<Self::Edge> {
		match self.outbound()
			.read()
			.iter()
			.find(|edge| edge.target() == target)
		{
			Some(edge) => Some(edge.clone()),
			None => None
		}
	}

	fn find_inbound(&self, source: &Self) -> Option<Self::Edge> {
		match self.inbound()
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

	fn sort_outbound(&self)
	where
		Self::Edge: Ord
	{
		self.outbound()
			.write()
			.sort();
	}

	fn sort_inbound(&self)
	where
		Self::Edge: Ord
	{
		self.inbound()
			.write()
			.sort();
	}

	fn min_outbound(&self) -> Option<Self::Edge>
	where
		Self::Edge: Ord
	{
		self.outbound()
			.read()
			.iter()
			.cloned()
			.min()
	}

	fn min_inbound(&self) -> Option<Self::Edge>
	where
		Self::Edge: Ord
	{
		self.inbound()
			.read()
			.iter()
			.cloned()
			.min()
	}

	fn max_outbound(&self) -> Option<Self::Edge>
	where
		Self::Edge: Ord
	{
		self.outbound()
			.read()
			.iter()
			.cloned()
			.max()
	}

	fn max_inbound(&self) -> Option<Self::Edge>
	where
		Self::Edge: Ord
	{
		self.inbound()
			.read()
			.iter()
			.cloned()
			.max()
	}

	fn remove_outbound(&self, edge: Self::Edge) {
		self.filter_outbound(&|e: &Self::Edge| e.source() != edge.source())
	}

	fn remove_inbound(&self, edge: Self::Edge) {
		self.filter_inbound(&|e: &Self::Edge| e.target() != edge.target())
	}

	// Algorithms
	fn bfs_map_adjacent<F>(&self, f: F) -> (Vec<Self::Edge>, Sig)
	where
		F: Fn(&Self, &Self, <Self::Edge as GraphEdge<Self>>::Params) -> (Coll, Sig) + Send + Sync,
	{
		let mut result = Vec::<Self::Edge>::new();
		for edge in self.outbound().read().iter() {
			match edge.target().try_close() {
				Ok(_) => {
					match f(edge.source(), edge.target(), edge.load()) {
						(Coll::Include, Sig::Continue) => { result.push(edge.clone()); }
						(Coll::Exclude, Sig::Continue) => {edge.target().open(); }
						(Coll::Include, Sig::Terminate) => { result.push(edge.clone()); return (result, Sig::Terminate); }
						(Coll::Exclude, Sig::Terminate) => { edge.target().open(); return (result, Sig::Terminate); }
					}
				},
				Err(_) => {}
			}
		}
		(result, Sig::Continue)
	}

	fn bfs<F>(&self, f: F) -> Option<Path<Self>>
	where
		F: Fn(&Self, &Self, <Self::Edge as GraphEdge<Self>>::Params) -> (Coll, Sig) + Send + Sync,
	{
		let mut bounds: (usize, usize) = (0, 0);
		let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
		self.close();
		match self.bfs_map_adjacent(&f) {
			(segment, Sig::Terminate) => {
				for edge in segment.iter() { edge.source().open(); edge.target().open(); }
				return Some(Path::from_edge_tree(segment));
			}
			(mut segment, Sig::Continue) => {
				loop {
					bounds.1 = segment.len();
					if bounds.0 == bounds.1 { break; }
					let current_frontier = &segment[bounds.0..bounds.1];
					bounds.0 = bounds.1;
					let frontier_segments: Vec<_> = current_frontier
						.into_par_iter()
						.map(|edge| {
							match terminate.load(Ordering::Relaxed) {
								true => { None }
								false => {
									let node = edge.target();
									match node.bfs_map_adjacent(&f) {
										(segment, Sig::Terminate) => {
											terminate.store(true, Ordering::Relaxed);
											Some(segment)
										}
										(segment, Sig::Continue) => Some(segment),
									}
								}
							}
						})
						.while_some()
						.collect();
					for mut segments in frontier_segments { segment.append(&mut segments); }
					if terminate.load(Ordering::Relaxed) == true { break; }
				}
				for edge in segment.iter() { edge.source().open(); edge.target().open(); }
				if terminate.load(Ordering::Relaxed) == true {
					Some(Path::from_edge_tree(segment))
				} else {
					None
				}
			}
		}
	}

	fn dfs_recursion<F>(&self, result: &mut Vec<Self::Edge>, f: &F) -> Option<Path<Self>>
	where
		F: Fn(&Self, &Self, <Self::Edge as GraphEdge<Self>>::Params) -> (Coll, Sig) + Send + Sync,
	{
		for edge in self.outbound().read().iter() {
			match edge.target().try_close() {
				Ok(_) => {
					match f(edge.source(), edge.target(), edge.load()) {
						(Coll::Include, Sig::Continue) => { result.push(edge.clone()); }
						(Coll::Exclude, Sig::Continue) => {edge.target().open(); }
						(Coll::Include, Sig::Terminate) => {
							result.push(edge.clone());
							for edge in result.iter() { edge.source().open(); edge.target().open(); }
							return Some(Path::from_edge_tree(result.clone()));
						}
						(Coll::Exclude, Sig::Terminate) => {
							edge.target().open();
							for edge in result.iter() { edge.source().open(); edge.target().open(); }
							return Some(Path::from_edge_tree(result.clone()));
						}
					}
					return edge.target().dfs_recursion(result, f);
				},
				Err(_) => {}
			}
		}
		for edge in result.iter() { edge.source().open(); edge.target().open(); }
		None
	}

	fn dfs<F>(&self, f: F) -> Option<Path<Self>>
	where
		F: Fn(&Self, &Self, <Self::Edge as GraphEdge<Self>>::Params) -> (Coll, Sig) + Send + Sync,
	{
		self.close();
		self.dfs_recursion(&mut Vec::<Self::Edge>::new(), &f)
	}

	fn dfsm_recursion<F>(&self, result: &mut Vec<Self::Edge>, f: &F) -> Option<Path<Self>>
	where
		F: Fn(&Self, &Self, <Self::Edge as GraphEdge<Self>>::Params) -> (Coll, Sig, Move<Self>) + Send + Sync,
	{
		for edge in self.outbound().read().iter() {
			match edge.target().try_close() {
				Ok(_) => {
					match f(edge.source(), edge.target(), edge.load()) {

						(Coll::Include, Sig::Continue, Move::Next) => { result.push(edge.clone()); }
						(Coll::Exclude, Sig::Continue, Move::Next) => { edge.target().open(); }

						(Coll::Include, Sig::Continue, Move::Prev) => { result.push(edge.clone()); return edge.source().dfsm_recursion(result, f); }
						(Coll::Exclude, Sig::Continue, Move::Prev) => { edge.target().open(); return edge.source().dfsm_recursion(result, f); }

						(Coll::Include, Sig::Continue, Move::Jump(next)) => { result.push(edge.clone()); return next.dfsm_recursion(result, f); }
						(Coll::Exclude, Sig::Continue, Move::Jump(next)) => { edge.target().open(); return next.dfsm_recursion(result, f); }

						(Coll::Include, Sig::Terminate, _) => {
							result.push(edge.clone());
							for edge in result.iter() { edge.source().open(); edge.target().open(); }
							return Some(Path::from_edge_tree(result.clone()));
						}
						(Coll::Exclude, Sig::Terminate, _) => {
							edge.target().open();
							for edge in result.iter() { edge.source().open(); edge.target().open(); }
							return Some(Path::from_edge_tree(result.clone()));
						}
					}
					return edge.target().dfsm_recursion(result, f);
				},
				Err(_) => {}
			}
		}
		for edge in result.iter() { edge.source().open(); edge.target().open(); }
		None
	}

	fn dfsm<F>(&self, f: F) -> Option<Path<Self>>
	where
		F: Fn(&Self, &Self, <Self::Edge as GraphEdge<Self>>::Params) -> (Coll, Sig, Move<Self>) + Send + Sync,
	{
		self.close();
		self.dfsm_recursion(&mut Vec::<Self::Edge>::new(), &f)
	}
}

pub struct EdgeInner<E, NT> {
	source: NT,
	target: NT,
	params: Mutex<E>,
	lock: AtomicBool,
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

	fn params(&self) -> &Mutex<Self::Params> {
		&self.handle.params
	}

	fn lock(&self) -> &AtomicBool {
		&self.handle.lock
	}
}

struct NodeInner<K, N, E> {
	id : K,
	params: Mutex<N>,
	outbound: RwLock<Vec<Edge<E, Node<K, N, E>>>>,
	inbound: RwLock<Vec<Edge<E, Node<K, N, E>>>>,
	lock: AtomicBool,
}

#[derive(Clone)]
pub struct Node<K, N, E> { handle: Arc<NodeInner<K, N, E>> }

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
				lock: AtomicBool::new(OPEN),
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

pub struct Path<N: GraphNode> {
	pub edges: Vec<N::Edge>,
}

impl<N: GraphNode> std::ops::Index<usize> for Path<N> {
	type Output = N::Edge;
	fn index(&self, index: usize) -> &Self::Output {
		self.edges.get(index).unwrap()
	}
}

impl <N: GraphNode> Path<N> {
	fn new() -> Path<N> { Path { edges: Vec::new() } }
	pub fn node_count(&self) -> usize { self.edges.len() + 1 }
	pub fn edge_count(&self) -> usize { self.edges.len() }

	fn from_edge_tree(edge_tree: Vec<N::Edge>) -> Path<N> {
		let mut path: Path<N> = Path::new();
		let w = edge_tree.get(edge_tree.len() - 1).unwrap();
		path.edges.push(w.clone());
		let mut i = 0;
		for edge in edge_tree.iter().rev() {
			let source = path.edges[i].source();
			if edge.target() == source {
				path.edges.push(edge.clone());
				i += 1;
			}
		}
		path.edges.reverse();
		path
	}

	pub fn walk<F>(&self, mut f: F)
		where F: FnMut(&N, &N, <<N as GraphNode>::Edge as GraphEdge<N>>::Params)
	{
		for edges in self.edges.iter() {
			f(edges.source(), edges.target(), edges.load());
		}
	}
}
