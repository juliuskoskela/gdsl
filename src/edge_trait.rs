use std::sync::atomic::{AtomicBool, Ordering};
use std::fmt::Display;
use parking_lot::Mutex;
use crate::enums::*;
use crate::node_trait::*;

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
