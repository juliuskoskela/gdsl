// Includes

use std:: {
	fmt:: {
		Debug,
		Display,
	},
	hash::Hash,
	sync::Arc,
};

use crate::global::*;
use rayon::prelude::*;

/// Path

#[derive(Debug, Clone)]
pub struct Path<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	edges: Vec<WeakEdge<K, N, E>>,
}

/// Path: Implementations

impl<K, N, E> Path<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	pub fn new() -> Self {
		Self {
			edges: Vec::new(),
		}
	}

	pub fn add(&mut self, edge: &RefEdge<K, N, E>) {
		self.edges.push(Arc::downgrade(edge));
	}

	pub fn add_weak(&mut self, edge: &WeakEdge<K, N, E>) {
		self.edges.push(edge.clone());
	}

	pub fn find(&self, source: &RefNode<K, N, E>, target: &RefNode<K, N, E>) -> Option<RefEdge<K, N, E>> {
        for weak in self.iter() {
			let alive = weak.upgrade();
			match alive {
				Some(edge) => {
					if edge.source() == *source && edge.target() == *target {
						return Some(edge);
					}
				}
				None => {}
			}
        }
        None
    }

	pub fn find_index(&self, target: &RefNode<K, N, E>) -> Option<usize> {
		for (i, weak) in self.iter().enumerate() {
			if let Some(edge) = weak.upgrade() {
				if edge.target() == *target {
					return Some(i);
				}
			}
		}
		None
	}

	pub fn is_empty(&self) -> bool {
		self.edges.is_empty()
	}

	pub fn iter(&self) -> std::slice::Iter<WeakEdge<K, N, E>> {
		self.edges.iter()
	}

	pub fn par_iter(&self) -> rayon::slice::Iter<WeakEdge<K, N, E>> {
		self.edges.par_iter()
	}

	pub fn del(&mut self, target: &RefNode<K, N, E>) -> bool {
		let index = self.find_index(target);
		match index {
			Some(i) => {
				self.edges.remove(i);
				return true;
			},
			None => { return false; },
		}
	}

	pub fn del_index(&mut self, index: usize) {
		if index < self.edges.len() {
			self.edges.remove(index);
		}
	}

	pub fn open_all(&self) -> &Self {
		for weak in self.edges.iter() {
			let alive = weak.upgrade();
			match alive {
				Some(edge) => {
					edge.open();
					edge.target().open();
					edge.source().open();
				}
				None => { panic!("Weak reference not alive!") }
			}
		}
		self
	}

	pub fn backtrack(&self) -> Option<Path<K, N, E>> {
		if self.edges.len() == 0 {
			return None;
		}
		let mut res = Path::new();
		let w = &self.edges[self.edges.len() - 1];
		res.add_weak(w);
		let mut i = 0;
		for edge in self.edges.iter().rev() {
			let source = res.edges[i].upgrade().unwrap().source();
			if edge.upgrade().unwrap().target() == source {
				res.edges.push(edge.clone());
				i += 1;
			}
		}
		Some(res)
	}
}
