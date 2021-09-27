///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

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

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// EdgeList

#[derive(Debug, Clone)]
pub struct EdgeList<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	list: Vec<EdgeWeak<K, N, E>>,
}

///////////////////////////////////////////////////////////////////////////////
///
/// EdgeList: Implementations

impl<K, N, E> EdgeList<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	pub fn new() -> Self {
		Self {
			list: Vec::new(),
		}
	}

	pub fn add(&mut self, edge: &EdgeRef<K, N, E>) {
		self.list.push(Arc::downgrade(edge));
	}

	pub fn add_weak(&mut self, edge: &EdgeWeak<K, N, E>) {
		self.list.push(edge.clone());
	}

	pub fn find(&self, source: &NodeRef<K, N, E>, target: &NodeRef<K, N, E>) -> Option<EdgeRef<K, N, E>> {
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

	pub fn find_index(&self, target: &NodeRef<K, N, E>) -> Option<usize> {
		for (i, weak) in self.iter().enumerate() {
			let alive = weak.upgrade();
			match alive {
				Some(edge) => {
					if edge.target() == *target {
						return Some(i);
					}
				}
				None => {}
			}
		}
		None
	}

	pub fn is_empty(&self) -> bool {
		self.list.is_empty()
	}

	pub fn iter(&self) -> std::slice::Iter<EdgeWeak<K, N, E>> {
		self.list.iter()
	}

	pub fn par_iter(&self) -> rayon::slice::Iter<EdgeWeak<K, N, E>> {
		self.list.par_iter()
	}

	pub fn del(&mut self, target: &NodeRef<K, N, E>) -> bool {
		let index = self.find_index(target);
		match index {
			Some(i) => {
				self.list.remove(i);
				return true;
			},
			None => { return false; },
		}
	}

	pub fn del_index(&mut self, index: usize) {
		if index < self.list.len() {
			self.list.remove(index);
		}
	}

	pub fn open_all(&self) -> &Self {
		for weak in self.list.iter() {
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

	pub fn backtrack(&self) -> Option<EdgeList<K, N, E>> {
		if self.list.len() == 0 {
			return None;
		}
		let mut res = EdgeList::new();
		let w = &self.list[self.list.len() - 1];
		res.add_weak(w);
		let mut i = 0;
		for edge in self.list.iter().rev() {
			let source = res.list[i].upgrade().unwrap().source();
			if edge.upgrade().unwrap().target() == source {
				res.list.push(edge.clone());
				i += 1;
			}
		}
		Some(res)
	}
}

///////////////////////////////////////////////////////////////////////////////
