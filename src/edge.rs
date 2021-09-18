///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	fmt:: {
		Debug,
		Display,
		Formatter,
	},
	hash::Hash,
	sync:: {
		Mutex,
		atomic:: {
			AtomicBool,
			Ordering
		}
	}
};

use crate::global::*;


///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// EDGE
///
/// Internal edge struct. Only accessible trough the Digraph object.

#[derive(Debug)]
pub struct Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	source: Vertex<K, N, E>,
	target: Vertex<K, N, E>,
	data: Mutex<E>,
	lock: AtomicBool,
}

///////////////////////////////////////////////////////////////////////////////

impl<K, N, E> Clone for Edge<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display,
	N: Clone + Debug + Display,
	E: Clone + Debug + Display,
{
    fn clone(&self) -> Self {
        Edge {
			source: self.source.clone(),
            target: self.target.clone(),
            data: Mutex::new(self.data.lock().unwrap().clone()),
            lock: AtomicBool::new(self.lock.load(Ordering::Relaxed)),
        }
    }
}

impl<K, N, E> Display for Edge<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display,
	N: Clone + Debug + Display,
	E: Clone + Debug + Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Edge \"{}\" {}", self.source().key(), self.to_string())
	}
}

///////////////////////////////////////////////////////////////////////////////

impl<K, N, E> Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	pub fn new(
		source: Vertex<K, N, E>,
		target: Vertex<K, N, E>,
		data: E) -> Edge<K, N, E> {
		Edge {
			source,
			target,
			data: Mutex::new(data),
			lock: AtomicBool::new(OPEN),
		}
	}

	pub fn lock(&self) -> bool {
		self.lock.load(Ordering::Relaxed)
	}

	pub fn close(&self) {
		self.lock.store(CLOSED, Ordering::Relaxed)
	}

	pub fn open(&self) {
		self.lock.store(OPEN, Ordering::Relaxed)
	}

	pub fn source(&self) -> Vertex<K, N, E> {
		self.source.clone()
	}

	pub fn target(&self) -> Vertex<K, N, E> {
		self.target.clone()
	}

	pub fn load(&self) -> E {
		self.data.lock().unwrap().clone()
	}

	pub fn store(&self, data: E) {
		*self.data.lock().unwrap() = data;
	}

	pub fn to_string(&self) -> String {
		let lock_state = if self.lock() == false {"OPEN"} else {"CLOSED"};
		format!("-> \"{}\" : \"{}\" : \"{}\"",
			self.target().key(),
			lock_state,
			self.data.lock().unwrap())
	}
}

///////////////////////////////////////////////////////////////////////////////
