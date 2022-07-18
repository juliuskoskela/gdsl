use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fmt::Display;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::iter::IntoParallelRefIterator;
use parking_lot::{RwLock, Mutex};
use crate::enums::*;
use crate::edge::*;
use crate::path::*;

pub trait GraphNode: PartialEq + Clone + Sync + Send {

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
				found = true;
				self_outbound.remove(i);
				break;
			}
		}
		if found {
			for (i, edge) in target_inbound.iter().enumerate() {
				if edge.source() == self {
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

	fn next(&self) -> Option<(Self::Edge, Self)> {
		match self.outbound().read().iter().find(|edge| {
			match edge.target().try_close() {
				Ok(true) => true,
				_ => false
			}
		}) {
			Some(edge) => Some((edge.clone(), edge.target().clone())),
			None => None
		}
	}

	fn prev(&self) -> Option<(<<Self as GraphNode>::Edge as GraphEdge<Self>>::Params, Self)> {
		match self.inbound().read().iter().find(|edge| {
			match edge.source().try_open() {
				Ok(true) => true,
				_ => false
			}
		}) {
			Some(edge) => Some((edge.load(), edge.target().clone())),
			None => None
		}
	}

	fn adjacent<F>(&self, mut f: F)
	where
		F: FnMut(&Self, <Self::Edge as GraphEdge<Self>>::Params) -> bool
	{
		for edge in self.outbound().read().iter() {
			if !f(edge.target(), edge.load()) {
				break;
			}
		}
	}

	fn par_adjacent<F>(&self, f: F)
	where
		F: Fn(&Self, <Self::Edge as GraphEdge<Self>>::Params) -> bool + Sync + Send,
		Self::Edge: Send
	{
		let adj = self.outbound().read();
		adj.par_iter().map(|edge| {
			if f(edge.target(), edge.load()) {
				Some(edge.clone())
			}
			else { None }
		}).while_some().collect::<Vec<_>>();
	}

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
