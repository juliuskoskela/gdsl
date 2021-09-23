use crate::edge::*;
use crate::edge_list::*;
use crate::global::*;
/// Includes
use std::{
    collections::VecDeque,
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
		Arc,
    },
};
use std::slice::Iter;

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
    lock: AtomicBool,
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
            outbound: self.outbound.clone(),
            inbound: self.inbound.clone(),
            lock: AtomicBool::new(self.lock.load(Ordering::Relaxed)),
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

    pub fn outbound(&self) -> ListRef<K, N, E> {
    	self.outbound.clone()
    }

    pub fn find_outbound(&self, target: &NodeRef<K, N, E>) -> Option<usize> {
        for (i, edge) in self.outbound.borrow().list.iter().enumerate() {
            if edge.target() == *target {
                return Some(i);
            }
        }
        None
    }

    pub fn find_inbound(&self, source: &NodeRef<K, N, E>) -> Option<usize> {
        for (i, edge) in self.inbound.borrow().list.iter().enumerate() {
            if edge.target() == *source {
                return Some(i);
            }
        }
        None
    }

    fn traverse_consume_node(
        &self,
        target: &NodeRef<K, N, E>,
        queue: &mut VecDeque<NodeRef<K, N, E>>,
        result: &mut EdgeList<K, N, E>,
    ) -> bool {
        for edge in &self.outbound.borrow().list {
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
        for edge in self.outbound.borrow().list.iter() {
            outbound.push(format!("	{}", edge.to_string()));
        }
        for edge in self.inbound.borrow().list.iter() {
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

pub fn connect<K, N, E>(source: &NodeRef<K, N, E>, target: &NodeRef<K, N, E>, data: E) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    match source.find_outbound(target) {
        Some(_) => false,
        None => {
            let mut outbound = source.outbound.borrow_mut();
            let mut inbound = target.inbound.borrow_mut();
            let new_edge = EdgeRef::new(Edge::new(source.clone(), target.clone(), data));
            outbound.add(new_edge.clone());
            inbound.add(new_edge);
            true
        }
    }
}

pub fn disconnect<K, N, E>(source: &NodeRef<K, N, E>, target: &NodeRef<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    let mut source_outbound = source.outbound.borrow_mut();
    for (i, edge_out) in source_outbound.list.iter().enumerate() {
        if edge_out.target() == *target {
            let mut target_inbound = target.inbound.borrow_mut();
            for (j, edge_in) in target_inbound.list.iter().enumerate() {
                if edge_in.target() == *target {
                    target_inbound.del_index(j);
                    break;
                }
            }
            source_outbound.del_index(i);
            return true;
        }
    }
    false
}

///////////////////////////////////////////////////////////////////////////////
