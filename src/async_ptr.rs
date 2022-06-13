///////////////////////////////////////////////////////////////////////////////
// Async Data Pointer
///////////////////////////////////////////////////////////////////////////////

use parking_lot::RwLock;
use std::sync::{Arc, Weak};

///////////////////////////////////////////////////////////////////////////////
// AsyncPtr
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AsyncPtr<T> { ptr: Option<Arc<RwLock<T>>>, }

impl <T: Clone> AsyncPtr<T> {
	pub fn null() -> Self {
		AsyncPtr { ptr: None }
	}

	pub fn from(ptr: T) -> Self {
		AsyncPtr { ptr: Some(Arc::new(RwLock::new(ptr))) }
	}

	pub fn as_ref(&self) -> AsyncRef<T> {
		AsyncRef { ptr: Arc::downgrade(&self.ptr.clone().unwrap()) }
	}

	pub fn read(&self) -> T {
		match self.ptr {
			Some(ref ptr) => ptr.read().clone(),
			None => panic!("AsyncPtr::read: null pointer"),
		}
	}

	pub fn write(&self, new: T) {
		match self.ptr {
			Some(ref ptr) => ptr.write().clone_from(&new),
			None => panic!("AsyncPtr::write: null pointer"),
		}
	}

	pub fn update(&self, f: impl FnOnce(&mut T)) {
		match self.ptr {
			Some(ref ptr) => {
				let mut lock = ptr.write();
				f(&mut lock);
			},
			None => panic!("AsyncPtr::update: null pointer"),
		}
	}
}

impl<T: std::fmt::Display + Clone> std::fmt::Display for AsyncPtr<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.read())
	}
}

///////////////////////////////////////////////////////////////////////////////
// AsyncRef
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AsyncRef<T> { ptr: Weak<RwLock<T>>, }

impl <T: Clone> AsyncRef<T> {
	pub fn as_ptr(&self) -> AsyncPtr<T> {
		AsyncPtr { ptr: Some(self.ptr.upgrade().unwrap()) }
	}

	pub fn read(&self) -> T {
		let lock = self.ptr.upgrade().unwrap();
		let lock = lock.read();
		lock.clone()
	}

	pub fn write(&self, new: T) {
		let lock = self.ptr.upgrade().unwrap();
		*lock.write() = new;
	}

	pub fn update(&self, f: impl FnOnce(&mut T)) {
		let lock = self.ptr.upgrade().unwrap();
		f(&mut lock.write());
	}
}

impl<T: std::fmt::Display + Clone> std::fmt::Display for AsyncRef<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.read())
	}
}