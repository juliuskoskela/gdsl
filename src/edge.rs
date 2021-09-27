///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	fmt:: {
		Debug,
		Display,
		Formatter,
	}, hash::Hash, sync:: {
		Mutex,
		Arc,
		Weak,
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
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	source: NodeWeak<K, N, E>,
	target: NodeWeak<K, N, E>,
	data: Mutex<E>,
	lock: Arc<AtomicBool>,
}

///////////////////////////////////////////////////////////////////////////////

unsafe impl<K, N, E> Sync for Edge<K, N, E> where
	K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{}

impl<K, N, E> Clone for Edge<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn clone(&self) -> Self {
        Edge {
			source: self.source.clone(),
            target: self.target.clone(),
            data: Mutex::new(self.data.lock().unwrap().clone()),
            lock: Arc::new(AtomicBool::new(self.lock.load(Ordering::Relaxed))),
        }
    }
}

impl<K, N, E> Display for Edge<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Edge \"{}\" {}", self.source().key(), self.to_string())
	}
}

///////////////////////////////////////////////////////////////////////////////

impl<K, N, E> Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	pub fn new(
		source: &NodeRef<K, N, E>,
		target: &NodeRef<K, N, E>,
		data: E) -> Edge<K, N, E> {
		Edge {
			source: Arc::downgrade(source),
			target: Arc::downgrade(target),
			data: Mutex::new(data),
			lock: Arc::new(AtomicBool::new(OPEN)),
		}
	}

	pub fn try_lock(&self) -> bool {
		self.lock.load(Ordering::Relaxed)
	}

	pub fn get_lock(&self) -> Weak<AtomicBool> {
		Arc::downgrade(&self.lock)
	}

	pub fn close(&self) {
		self.lock.store(CLOSED, Ordering::Relaxed)
	}

	pub fn open(&self) {
		self.lock.store(OPEN, Ordering::Relaxed)
	}

	pub fn source(&self) -> NodeRef<K, N, E> {
		self.source.upgrade().unwrap()
	}

	pub fn target(&self) -> NodeRef<K, N, E> {
		self.target.upgrade().unwrap()
	}

	pub fn load(&self) -> E {
		self.data.lock().unwrap().clone()
	}

	pub fn store(&self, data: E) {
		let mut x = self.data.lock().expect("Error locking mutex!");
		*x = data;
	}

	pub fn to_string(&self) -> String {
		let lock_state = if self.try_lock() == false {"OPEN"} else {"CLOSED"};
		format!("-> \"{}\" : \"{}\" : \"{}\"",
			self.target().key(),
			lock_state,
			self.data.lock().unwrap())
	}
}

///////////////////////////////////////////////////////////////////////////////
