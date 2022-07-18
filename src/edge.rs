use std::fmt::Display;
use parking_lot::Mutex;
use crate::node::*;

pub trait GraphEdge<N: GraphNode>: Clone + Send + Sync
{
	// Associated types
	type Params: Clone + Sync + Send + Display;

	// Required implementations
	fn new(source: &N, target: &N, data: Self::Params) -> Self;
	fn source(&self) -> &N;
	fn target(&self) -> &N;
	fn params(&self) -> &Mutex<Self::Params>;

	// Trivial implementations
	fn load(&self) -> Self::Params { self.params().lock().clone() }
	fn store(&self, params: Self::Params) { *self.params().lock() = params; }
}
