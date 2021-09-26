/// Includes

use crate::edge::*;
use crate::edge_list::*;
use crate::global::*;

use std:: {
	hash::Hash,
	collections::VecDeque,
    cell:: {
		Ref,
		RefMut
	},
    fmt:: {
		Debug,
		Display,
		Formatter
	},
    sync:: {
        atomic:: {
			AtomicBool,
			Ordering
		},
        Mutex,
		Arc,
    },
};

/// Node

#[derive(Debug)]
pub struct Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    key: K,
    data: Mutex<N>,
    pub outbound: ListRef<K, N, E>,
    pub inbound: ListRef<K, N, E>,
    lock: Arc<AtomicBool>,
}

/// Node: Traits

unsafe impl<K, N, E> Sync for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{ }

impl<K, N, E> Clone for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn clone(&self) -> Self {
        Node {
            key: self.key.clone(),
            data: Mutex::new(self.data.lock().unwrap().clone()),
            outbound: self.outbound.clone(),
            inbound: self.inbound.clone(),
            lock: Arc::new(AtomicBool::new(self.lock.load(Ordering::Relaxed))),
        }
    }
}

impl<K, N, E> PartialEq for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
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
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Node {}", self.display_string())
    }
}

/// Node: Implementations

impl<K, N, E> Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    pub fn new(key: K, data: N) -> Node<K, N, E> {
        Node {
            key,
            data: Mutex::new(data),
            outbound: ListRef::new(EdgeList::new()),
            inbound: ListRef::new(EdgeList::new()),
            lock: Arc::new(AtomicBool::new(false)),
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

    pub fn outbound(&self) -> Ref<EdgeList<K, N, E>> {
        self.outbound.borrow()
    }

    pub fn outbound_mut(&self) -> RefMut<EdgeList<K, N, E>> {
        self.outbound.borrow_mut()
    }

    pub fn inbound(&self) -> Ref<EdgeList<K, N, E>> {
        self.inbound.borrow()
    }
    pub fn inbound_mut(&self) -> RefMut<EdgeList<K, N, E>> {
        self.inbound.borrow_mut()
    }

    fn traverse_consume_node(
        &self,
        target: &NodeRef<K, N, E>,
        queue: &mut VecDeque<NodeRef<K, N, E>>,
        result: &mut EdgeList<K, N, E>,
    ) -> bool {
        for edge in self.outbound().iter() {
            if edge.lock() == OPEN && edge.target().lock() == OPEN {
                edge.close();
                edge.target().close();
                queue.push_back(edge.target.clone());
                result.add(edge.clone());
                if edge.target == *target {
                    return true;
                }
            }
        }
        false
    }

    pub fn traverse_breadth(&self, target: &NodeRef<K, N, E>) -> Option<EdgeList<K, N, E>> {
        let mut queue = VecDeque::new();
        let mut result = EdgeList::new();
        self.close();
        if self.traverse_consume_node(target, &mut queue, &mut result) {
            result.open_all();
            return Some(result);
        }
        while let Some(node) = queue.pop_front() {
            if node.traverse_consume_node(target, &mut queue, &mut result) {
                result.open_all();
                return Some(result);
            }
        }
        result.open_all();
        None
    }

    pub fn shortest_path(&self, target: &NodeRef<K, N, E>) -> Option<EdgeList<K, N, E>> {
        let mut queue = VecDeque::new();
        let mut result = EdgeList::new();
        self.close();
        if self.traverse_consume_node(target, &mut queue, &mut result) {
            result.open_all();
            return Some(result);
        }
        while let Some(node) = queue.pop_front() {
            if node.traverse_consume_node(target, &mut queue, &mut result) {
                return result.backtrack();
            }
        }
        result.open_all();
        None
    }

    pub fn display_string(&self) -> String {
        let mut outbound = vec![];
        let mut inbound = vec![];
        for edge in self.outbound().iter() {
            outbound.push(format!("	{}", edge.to_string()));
        }
        for edge in self.inbound().iter() {
            inbound.push(format!("	{}", edge.to_string()));
        }
        let lock_state = if self.lock() { "CLOSED" } else { "OPEN" };
        let header = format!(
            "\"{}\" : \"{}\" : \"{}\" {{ ",
            self.key,
            lock_state,
            self.data.lock().unwrap()
        );
        let mut body: String;
        if outbound.is_empty() {
            body = "\n".to_string() + &outbound.join("\n") + "\n";
        } else {
            body = "".to_string();
        }
        if inbound.is_empty() {
            body = body + "\n" + &inbound.join("\n") + "\n";
        }
        let end = "}";
        header + &body + end
    }
}

/// Node: Procedural Implementations
pub fn overlaps<K, N, E>(source: &NodeRef<K, N, E>, target: &NodeRef<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	match source.outbound().find(source, target) {
        Some(_) => true,
        None => false
    }
}

pub fn connect<K, N, E>(source: &NodeRef<K, N, E>, target: &NodeRef<K, N, E>, data: E) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	if !overlaps(source, target) {
		let new_edge = EdgeRef::new(Edge::new(source.clone(), target.clone(), data));
		source.outbound_mut().add(new_edge.clone());
		target.inbound_mut().add(new_edge);
		return true;
	}
    false
}

pub fn disconnect<K, N, E>(source: &NodeRef<K, N, E>, target: &NodeRef<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    source.outbound_mut().del(target);
	target.inbound_mut().del(source);
    false
}

pub enum Traverse {
	Skip,
	Collect,
	Finish,
}

fn depth_traversal_directed_recursion<K, N, E>(
	results: &mut EdgeList<K, N, E>,
	source: &NodeRef<K, N, E>,
	target: &NodeRef<K, N, E>,
	f: fn (&EdgeRef<K, N, E>, &NodeRef<K, N, E>) -> Traverse
) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	source.close();
	for edge in source.outbound().iter() {
		if edge.lock() == OPEN && edge.target().lock() == OPEN {
			edge.close();
			edge.target().close();
			let traverse = f(edge, target);
			match traverse {
				crate::node::Traverse::Collect => { results.add(edge.clone()); }
				crate::node::Traverse::Finish => { results.add(edge.clone()); return true; }
				crate::node::Traverse::Skip => {}
			}
			return depth_traversal_directed_recursion(results, &edge.target(), target, f);
		}
	}
	false
}

pub fn depth_traversal_directed<K, N, E>(
	source: &NodeRef<K, N, E>,
	target: &NodeRef<K, N, E>,
	f: fn (&EdgeRef<K, N, E>, &NodeRef<K, N, E>) -> Traverse
) -> EdgeList<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	let mut edge_list = EdgeList::new();
	let res = depth_traversal_directed_recursion(&mut edge_list, source, target, f);
	match res {
		true => { edge_list.open_all(); edge_list }
		false => { edge_list.open_all(); EdgeList::new() }
	}
}

fn breadth_traversal_node<'a, K, N, E>(
	source: &NodeRef<K, N, E>,
	target: &NodeRef<K, N, E>,
	queue: &mut VecDeque<NodeRef<K, N, E>>,
	result: &mut EdgeList<K, N, E>,
	locks: &mut Vec<Arc<AtomicBool>>,
	f: fn (&EdgeRef<K, N, E>) -> Traverse,
) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	for edge in source.outbound().iter() {
		if edge.lock() == OPEN && edge.target().lock() == OPEN {
			edge.close();
			locks.push(edge.lock.clone());
			let traverse = f(edge);
			match traverse {
				crate::node::Traverse::Skip => { }
				crate::node::Traverse::Collect => {
					edge.target().close();
					queue.push_back(edge.target.clone());
					result.add(edge.clone());
					locks.push(edge.target().lock.clone());
					if edge.target() == *target {
						return true;
					}
				}
				crate::node::Traverse::Finish => {
					result.add(edge.clone());
					return true;
				}
			}
		}
	}
	false
}

pub fn breadth_traversal_directed<K, N, E>(
	source: &NodeRef<K, N, E>,
	target: &NodeRef<K, N, E>,
	f: fn (&EdgeRef<K, N, E>) -> Traverse
) -> Option<EdgeList<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	let mut locks = Vec::new();
	let mut queue = VecDeque::new();
	let mut result = EdgeList::new();
	source.close();
	locks.push(source.lock.clone());
	if breadth_traversal_node(source, target, &mut queue, &mut result, &mut locks, f) {
		for l in locks {
			l.store(OPEN, Ordering::Relaxed);
		}
		return Some(result);
	}
	while let Some(node) = queue.pop_front() {
		if breadth_traversal_node(&node, target, &mut queue, &mut result, &mut locks, f) {
			for l in locks {
				l.store(OPEN, Ordering::Relaxed);
			}
			return Some(result);
		}
	}
	for l in locks {
		l.store(OPEN, Ordering::Relaxed);
	}
	None
}

///////////////////////////////////////////////////////////////////////////////
