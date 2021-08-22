use std::sync::atomic::{AtomicBool, Ordering};
use std::fmt;

#[derive(Debug)]
pub struct
GraphEdge<E>
where E: Clone + fmt::Debug {
	pub u: usize,
	pub v: usize,
	pub arg: E,
	pub valid: AtomicBool,
}

impl<E> Clone
for GraphEdge<E>
where E: Clone + fmt::Debug {
	fn clone(&self) -> Self {
		GraphEdge {
			u: self.u,
			v: self.v,
			arg: self.arg.clone(),
			valid: AtomicBool::new(self.valid.load(Ordering::Relaxed)),
		}
	}
}

impl<E>
GraphEdge<E>
where E: Clone + fmt::Debug {
	pub fn new(u: usize, v: usize, arg: E) -> Self {
		Self {
			u,
			v,
			arg,
			valid: AtomicBool::new(true),
		}
	}
	pub fn open(&self) {
		self.valid.store(true, Ordering::Relaxed);
	}
	pub fn close(&self) {
		self.valid.store(false, Ordering::Relaxed);
	}
}
