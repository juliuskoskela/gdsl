/// Includes

use crate::adjacent::*;
use crate::edge::*;
use crate::edge_list::*;
use crate::global::*;

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Weak,
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
    outbound: RefAdjacent<K, N, E>,
    inbound: RefCell<EdgeList<K, N, E>>,
    lock: Arc<AtomicBool>,
}

/// Node: Traits

unsafe impl<K, N, E> Sync for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
}

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
            outbound: RefAdjacent::new(Adjacent::new()),
            inbound: RefCell::new(EdgeList::new()),
            lock: Arc::new(AtomicBool::new(OPEN)),
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
            outbound: RefAdjacent::new(Adjacent::new()),
            inbound: RefCell::new(EdgeList::new()),
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

    pub fn outbound(&self) -> Ref<Adjacent<K, N, E>> {
        self.outbound.borrow()
    }

    pub fn outbound_mut(&self) -> RefMut<Adjacent<K, N, E>> {
        self.outbound.borrow_mut()
    }

    pub fn inbound(&self) -> Ref<EdgeList<K, N, E>> {
        self.inbound.borrow()
    }
    pub fn inbound_mut(&self) -> RefMut<EdgeList<K, N, E>> {
        self.inbound.borrow_mut()
    }

	pub fn degree(&self) -> usize {
		self.outbound().len()
	}

	pub fn is_leaf(&self) -> bool {
		self.outbound().len() == 0
	}

    pub fn display_string(&self) -> String {
        let mut outbound = vec![];
        let mut inbound = vec![];
        for edge in self.outbound().iter() {
            outbound.push(format!("	{}", edge.to_string()));
        }
        for edge in self.inbound().iter() {
            inbound.push(format!("	{}", edge.upgrade().unwrap().to_string()));
        }
        let lock_state = if self.try_lock() { "CLOSED" } else { "OPEN" };
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
fn overlaps<K, N, E>(source: &RefNode<K, N, E>, target: &RefNode<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    source.outbound().find(source, target).is_some()
}

pub fn connect<K, N, E>(source: &RefNode<K, N, E>, target: &RefNode<K, N, E>, data: E) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    if !overlaps(source, target) {
        let new_edge = RefEdge::new(Edge::new(source, target, data));
        target.inbound_mut().add_weak(&Arc::downgrade(&new_edge));
        source.outbound_mut().add(new_edge);
        return true;
    }
    false
}

pub fn disconnect<K, N, E>(source: &RefNode<K, N, E>, target: &RefNode<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    let sr = source.outbound_mut().del(target);
    let tr = target.inbound_mut().del(source);
    sr && tr
}

fn depth_traversal_directed_recursion<K, N, E, F>(
    source: &RefNode<K, N, E>,
    results: &mut EdgeList<K, N, E>,
    locks: &mut Vec<Weak<AtomicBool>>,
    f: F,
) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    source.close();
    locks.push(source.get_lock());
    for edge in source.outbound().iter() {
        if edge.try_lock() == OPEN && edge.target().try_lock() == OPEN {
            edge.target().close();
            edge.close();
            locks.push(edge.get_lock());
            let traverse = f(edge);
            match traverse {
                crate::node::Traverse::Traverse => {
                    results.add(&edge);
                    locks.push(edge.target().get_lock());
                }
				crate::node::Traverse::Finish => {
                    results.add(&edge);
                    locks.push(edge.target().get_lock());
                    return true;
                }
                crate::node::Traverse::Skip => {
					edge.target().open();
				}
            }
            return depth_traversal_directed_recursion(&edge.target(), results, locks, f);
        }
    }
    false
}

pub fn depth_traversal_directed<K, N, E, F>(
    source: &RefNode<K, N, E>,
    f: F,
) -> Option<EdgeList<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    let mut result = EdgeList::new();
    let mut locks = Vec::new();
    let res = depth_traversal_directed_recursion(source, &mut result, &mut locks, f);
    for weak in locks {
        let arc = weak.upgrade().unwrap();
        arc.store(OPEN, Ordering::Relaxed);
    }
    match res {
        true => Some(result),
        false => None,
    }
}

fn breadth_traversal_node<K, N, E, F>(
    source: &RefNode<K, N, E>,
    queue: &mut VecDeque<RefNode<K, N, E>>,
    result: &mut EdgeList<K, N, E>,
    locks: &mut Vec<Weak<AtomicBool>>,
    f: &F,
) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    for edge in source.outbound().iter() {
        if edge.try_lock() == OPEN && edge.target().try_lock() == OPEN {
			edge.target().close();
            edge.close();
            locks.push(edge.get_lock());
            let traverse = f(edge);
            match traverse {
                crate::node::Traverse::Traverse => {
                    queue.push_back(edge.target());
                    result.add(&edge);
                    locks.push(edge.target().get_lock());
                }
				crate::node::Traverse::Finish => {
                    queue.push_back(edge.target());
                    result.add(&edge);
                    locks.push(edge.target().get_lock());
                    return true;
                }
                crate::node::Traverse::Skip => {
					edge.target().open();
				}
            }
        }
    }
    false
}

pub fn breadth_traversal_directed<K, N, E, F>(
    source: &RefNode<K, N, E>,
    f: F,
) -> Option<EdgeList<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    let mut result = EdgeList::new();
    let mut locks = Vec::new();
    let mut queue = VecDeque::new();
    source.close();
    locks.push(source.get_lock());
    if breadth_traversal_node(source, &mut queue, &mut result, &mut locks, &f) {
        for weak in locks {
            let lock = weak.upgrade().unwrap();
            lock.store(OPEN, Ordering::Relaxed);
        }
        return Some(result);
    }
    while let Some(node) = queue.pop_front() {
        if breadth_traversal_node(&node, &mut queue, &mut result, &mut locks, &f) {
            for weak in locks {
                let lock = weak.upgrade().unwrap();
                lock.store(OPEN, Ordering::Relaxed);
            }
            return Some(result);
        }
    }
    for weak in locks {
        let lock = weak.upgrade().unwrap();
        lock.store(OPEN, Ordering::Relaxed);
    }
    None
}

///////////////////////////////////////////////////////////////////////////////
