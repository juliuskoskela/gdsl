///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	collections::VecDeque,
	fmt:: {
		Debug,
		Display,
		Formatter,
	},
	hash::Hash,
	ops::Deref,
	sync:: {
		Mutex,
		atomic:: {
			AtomicBool,
			Ordering
		}
	}
};
use crate::global::*;
use crate::edge::*;
use crate::edge_list::*;

///////////////////////////////////////////////////////////////////////////////
///
/// Node

#[derive(Debug)]
pub struct Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	key: K,
	data: Mutex<N>,
	pub outbound: AdjacencyList<K, N, E>,
	pub inbound: AdjacencyList<K, N, E>,
	lock: AtomicBool,
}

///////////////////////////////////////////////////////////////////////////////
///
/// Node: Traits

impl<K, N, E> Clone for Node<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display,
	N: Clone + Debug + Display,
	E: Clone + Debug + Display,
{
    fn clone(&self) -> Self {
        Node {
            key: self.key.clone(),
            data: Mutex::new(self.data.lock().unwrap().clone()),
            outbound: Mutex::new(self.outbound.lock().unwrap().clone()),
			inbound: Mutex::new(self.inbound.lock().unwrap().clone()),
            lock: AtomicBool::new(self.lock.load(Ordering::Relaxed)),
        }
    }
}

impl<K, N, E> PartialEq for Node<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display,
	N: Clone + Debug + Display,
	E: Clone + Debug + Display,
{
    fn eq(&self, other: &Self) -> bool {
        if self.key == other.key {
			return true;
		}
		false
    }
}

impl<K, N, E> Display for Node<K, N, E>
where
	K: Hash + Eq + Clone + Debug + Display,
	N: Clone + Debug + Display,
	E: Clone + Debug + Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Node {}", self.to_string())
    }
}

///////////////////////////////////////////////////////////////////////////////
///
/// Node: Implementations

impl<K, N, E> Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	pub fn new(key: K, data: N) -> Node<K, N, E> {
		Node {
			key,
			data: Mutex::new(data),
			outbound: AdjacencyList::new(EdgeList::new()),
			inbound: AdjacencyList::new(EdgeList::new()),
			lock: AtomicBool::new(false),
		}
	}

	pub fn load(&self) -> N {
		self.data.lock().unwrap().clone()
	}

	pub fn store(&self, data: N) {
		*self.data.lock().unwrap() = data;
	}

	pub fn key(&self) -> &K {
		&self.key
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

	///////////////////////////////////////////////////////////////////////////

	pub fn find_outbound(&self, target: Vertex<K, N, E>) -> Option<usize> {
		let mut i: usize = 0;
		for edge in self.outbound.lock().unwrap().list.iter() {
			if edge.target() == target {
				return Some(i);
			}
			i += 1;
		}
		None
	}

	pub fn find_inbound(&self, source: Vertex<K, N, E>) -> Option<usize> {
		let mut i: usize = 0;
		for edge in self.inbound.lock().unwrap().list.iter() {
			if edge.target() == source {
				return Some(i);
			}
			i += 1;
		}
		None
	}

	///////////////////////////////////////////////////////////////////////////

	pub fn bfs_directed(&self, target: &Vertex<K, N, E>) -> EdgeList<K, N, E> {
        let mut queue = VecDeque::new();
		let mut edge_list = EdgeList::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            let edges =
				queue.pop_front()
				.unwrap()
				.outbound;
            for e in edges.lock().unwrap().list.iter() {
				if e.lock() == OPEN && e.target().lock() == OPEN {
					e.close();
					e.target().close();
					e.source().close();
					edge_list.add(e.clone());
					if e.target() == *target {
						edge_list.open_all();
						return edge_list;
					}
					queue.push_back(e.target().deref().clone());
				}
            }
        }
		edge_list.open_all();
		edge_list
	}

	pub fn bfs_undirected(&self, target: &Vertex<K, N, E>) -> EdgeList<K, N, E> {
        let mut queue = VecDeque::new();
		let mut edge_list = EdgeList::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
			let node = queue.pop_front()
				.unwrap();
            for e in node.outbound.lock().unwrap().list.iter() {
				if e.lock() == OPEN && e.target().lock() == OPEN {
					e.close();
					e.target().close();
					e.source().close();
					edge_list.add(e.clone());
					if e.target() == *target {
						edge_list.open_all();
						return edge_list;
					}
					queue.push_back(e.target().deref().clone());
				}
            }
			for e in node.inbound.lock().unwrap().list.iter() {
				if e.lock() == OPEN && e.target().lock() == OPEN {
					e.close();
					e.target().close();
					e.source().close();
					edge_list.add(e.clone());
					if e.target() == *target {
						edge_list.open_all();
						return edge_list;
					}
					queue.push_back(e.target().deref().clone());
				}
			}
        }
		edge_list.open_all();
		edge_list
	}

	///////////////////////////////////////////////////////////////////////////

	pub fn to_string(&self) -> String {
		let mut outbound = vec![];
		for edge in self.outbound.lock().unwrap().list.iter() {
			outbound.push(format!("	{}", edge.to_string()));
		}
		let lock_state = if self.lock() == false {"OPEN"} else {"CLOSED"};
		let header = format!("\"{}\" : \"{}\" : \"{}\" {{ ",
			self.key,
			lock_state,
			self.data.lock().unwrap());
		let body : String;
		if outbound.len() > 0 {
			body = "\n".to_string() + &outbound.join("\n") + "\n";
		}
		else {
			body = "".to_string();
		}
		let end = "}";
		header + &body + end
	}
}

///////////////////////////////////////////////////////////////////////////////
///
/// Node: Procedural Implementations

pub fn connect<K, N, E>(source: &Vertex<K, N, E>, target: &Vertex<K, N, E>, data: E) -> bool
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	match source.find_outbound(target.clone()) {
		Some(_) => false,
		None => {
			let mut outbound = source.outbound.lock().unwrap();
			let mut inbound = target.inbound.lock().unwrap();
			let new_edge = EdgeRef::new(Edge::new(source.clone(), target.clone(), data.clone()));
			outbound.add(new_edge.clone());
			inbound.add(new_edge);
			true
		}
	}
}

pub fn disconnect<K, N, E>(source: &Vertex<K, N, E>, target: &Vertex<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display,
    N: Clone + Debug + Display,
    E: Clone + Debug + Display,
{
	let mut i: usize = 0;
	let mut j: usize = 0;
	let mut inbound = source.outbound.lock().unwrap();
	for edge_out in inbound.list.iter() {
		if edge_out.target() == *target {
			let mut outbound = target.outbound.lock().unwrap();
			for edge_in in outbound.list.iter() {
				if edge_in.source() == *source {
					outbound.del_index(j);
					break
				}
				j += 1;
			}
			inbound.del_index(i);
			return true;
		}
		i += 1;
	}
	false
}

///////////////////////////////////////////////////////////////////////////////
