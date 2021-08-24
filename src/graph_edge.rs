use std::sync::atomic::{AtomicBool, Ordering};
use std::fmt::Debug;
use crate::graph_types::Lock;

#[derive(Debug)]
pub struct
GraphEdge<E>
where E: Clone + Debug {
	source: usize,
	target: usize,
	arg: E,
	lock: AtomicBool,
}

impl<E> Clone
for GraphEdge<E>
where E: Clone + Debug {
	fn clone(&self) -> Self {
		GraphEdge {
			source: self.source,
			target: self.target,
			arg: self.arg.clone(),
			lock: AtomicBool::new(self.lock.load(Ordering::Relaxed)),
		}
	}
}

impl<E>
GraphEdge<E>
where E: Clone + Debug {
	pub fn new(source: usize, target: usize, arg: E) -> Self {
		Self {
			source,
			target,
			arg,
			lock: AtomicBool::new(false),
		}
	}
	pub fn lock_open(&self) {
		self.lock.store(false, Ordering::Relaxed);
	}
	pub fn lock_close(&self) {
		self.lock.store(true, Ordering::Relaxed);
	}
	pub fn lock_try(&self) -> Lock {
		let lock_bool = self.lock.load(Ordering::Relaxed);
		if lock_bool == false {
			return Lock::OPEN;
		}
		else {
			return Lock::CLOSED;
		}
	}
	pub fn get_source(&self) -> usize {
		self.source
	}
	pub fn get_target(&self) -> usize {
		self.target
	}
	pub fn get_arg(&self) -> &E {
		&self.arg
	}
	pub fn get_arg_mut(&mut self) -> &mut E {
		&mut self.arg
	}
}
