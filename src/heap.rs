// Just a min heap wrapper for Binary Heap

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub type MaxHeap<T> = BinaryHeap<T>;

pub struct MinHeap<T: Ord> {
	heap: BinaryHeap<Reverse<T>>
}

impl<T: Ord> MinHeap<T> {
	pub fn new() -> MinHeap<T> {
		MinHeap {
			heap: BinaryHeap::new()
		}
	}

	pub fn push(&mut self, value: T) {
		self.heap.push(Reverse(value));
	}

	pub fn pop(&mut self) -> Option<T> {
		self.heap.pop().map(|x| x.0)
	}

	pub fn peek(&self) -> Option<&T> {
		self.heap.peek().map(|x| &x.0)
	}

	pub fn len(&self) -> usize {
		self.heap.len()
	}

	pub fn is_empty(&self) -> bool {
		self.heap.is_empty()
	}

	pub fn clear(&mut self) {
		self.heap.clear();
	}
}