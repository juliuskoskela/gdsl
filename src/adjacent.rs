///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	fmt:: {
		Debug,
		Display,
	},
	hash::Hash,
};

use crate::global::*;
use rayon::prelude::*;

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// Adjacent

#[derive(Debug, Clone)]
pub struct Adjacent<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	list: Vec<RefEdge<K, N, E>>,
}

///////////////////////////////////////////////////////////////////////////////
///
/// Adjacent: Implementations

impl<K, N, E> Adjacent<K, N, E>
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

	pub fn add(&mut self, edge: RefEdge<K, N, E>) {
		self.list.push(edge);
	}

	pub fn len(&self) -> usize {
		self.list.len()
	}

	pub fn find(&self, source: &RefNode<K, N, E>, target: &RefNode<K, N, E>) -> Option<RefEdge<K, N, E>> {
        for edge in self.iter() {
            if edge.target() == *target && edge.source() == *source{
                return Some(edge.clone());
            }
        }
        None
    }

	pub fn find_index(&self, target: &RefNode<K, N, E>) -> Option<usize> {
		for (i, e) in self.iter().enumerate() {
			if e.target() == *target {
				return Some(i);
			}
		}
		None
	}

	pub fn is_empty(&self) -> bool {
		self.list.is_empty()
	}

	pub fn iter(&self) -> std::slice::Iter<RefEdge<K, N, E>> {
		self.list.iter()
	}

	pub fn par_iter(&self) -> rayon::slice::Iter<RefEdge<K, N, E>> {
		self.list.par_iter()
	}

	pub fn del(&mut self, target: &RefNode<K, N, E>) -> bool {
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

	pub fn backtrack(&self) -> Option<Adjacent<K, N, E>> {
		if self.list.len() == 0 {
			return None;
		}
		let mut res = Adjacent::new();
		res.add(self.list[self.list.len() - 1].clone());
		let mut i = 0;
		for edge in self.list.iter().rev() {
			let source = &res.list[i].source();
			if edge.target() == *source {
				res.list.push(edge.clone());
				i += 1;
			}
		}
		Some(res)
	}
}

///////////////////////////////////////////////////////////////////////////////
