use super::{method::*, path::*, *};
use ahash::AHashSet as HashSet;
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, hash::Hash};

enum Priority {
    Min,
    Max,
}

pub struct Pfs<'a, K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    root: Node<K, N, E>,
    target: Option<K>,
    method: Method<'a, K, N, E>,
    transpose: Transposition,
    priority: Priority,
}

impl<'a, K, N, E> Pfs<'a, K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone + Ord,
    E: Clone,
{
    pub fn new(root: &Node<K, N, E>) -> Self {
        Pfs {
            root: root.clone(),
            target: None,
            method: Method::Empty,
            transpose: Transposition::Outbound,
            priority: Priority::Min,
        }
    }

    pub fn min(mut self) -> Self {
        self.priority = Priority::Min;
        self
    }

    pub fn max(mut self) -> Self {
        self.priority = Priority::Max;
        self
    }

    pub fn target(mut self, target: &K) -> Self {
        self.target = Some(target.clone());
        self
    }

    pub fn transpose(mut self) -> Self {
        self.transpose = Transposition::Inbound;
        self
    }

    pub fn for_each(mut self, f: ForEach<'a, K, N, E>) -> Self {
        self.method = Method::ForEach(f);
        self
    }

    pub fn filter(mut self, f: Filter<'a, K, N, E>) -> Self {
        self.method = Method::Filter(f);
        self
    }

    fn loop_outbound_min(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut BinaryHeap<Reverse<Node<K, N, E>>>,
    ) -> bool {
        while let Some(node) = queue.pop() {
            let node = node.0;
            for edge in node.iter_out() {
                if self.method.exec(&edge) {
                    let v = edge.1.clone();
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        result.push(edge);
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return true;
                            }
                        }
                        queue.push(Reverse(v.clone()));
                    }
                }
            }
        }
        false
    }

    fn loop_inbound_min(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut BinaryHeap<Reverse<Node<K, N, E>>>,
    ) -> bool {
        while let Some(node) = queue.pop() {
            let node = node.0;
            for edge in node.iter_out() {
                let edge = edge.reverse();
                if self.method.exec(&edge) {
                    let v = edge.1.clone();
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        result.push(edge);
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return true;
                            }
                        }
                        queue.push(Reverse(v.clone()));
                    }
                }
            }
        }
        false
    }

    fn loop_outbound_max(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut BinaryHeap<Node<K, N, E>>,
    ) -> bool {
        while let Some(node) = queue.pop() {
            for edge in node.iter_out() {
                if self.method.exec(&edge) {
                    let v = edge.1.clone();
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        result.push(edge);
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return true;
                            }
                        }
                        queue.push(v.clone());
                    }
                }
            }
        }
        false
    }

    fn loop_inbound_max(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut BinaryHeap<Node<K, N, E>>,
    ) -> bool {
        while let Some(node) = queue.pop() {
            for edge in node.iter_in() {
                let edge = edge.reverse();
                if self.method.exec(&edge) {
                    let v = edge.1.clone();
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        result.push(edge);
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return true;
                            }
                        }
                        queue.push(v.clone());
                    }
                }
            }
        }
        false
    }

    pub fn search(&mut self) -> Option<Node<K, N, E>> {
        self.search_path().map(|path| path.last_node().unwrap().clone())
    }

    pub fn search_cycle(&'a mut self) -> Option<Path<K, N, E>> {
        let mut edges = vec![];
        let mut visited = HashSet::default();

        self.target = Some(self.root.key().clone());

        match self.transpose {
            Transposition::Outbound => match self.priority {
                Priority::Min => {
                    let mut queue = BinaryHeap::new();
                    queue.push(Reverse(self.root.clone()));
                    match self.loop_outbound_min(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
                Priority::Max => {
                    let mut queue = BinaryHeap::new();
                    queue.push(self.root.clone());
                    match self.loop_outbound_max(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
            },
            Transposition::Inbound => match self.priority {
                Priority::Min => {
                    let mut queue = BinaryHeap::new();
                    queue.push(Reverse(self.root.clone()));
                    match self.loop_outbound_min(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
                Priority::Max => {
                    let mut queue = BinaryHeap::new();
                    queue.push(self.root.clone());
                    match self.loop_outbound_max(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
            },
        }
    }

    pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
        let mut edges = vec![];
        let mut visited = HashSet::default();

        visited.insert(self.root.key().clone());

        match self.transpose {
            Transposition::Outbound => match self.priority {
                Priority::Min => {
                    let mut queue = BinaryHeap::new();
                    queue.push(Reverse(self.root.clone()));
                    match self.loop_outbound_min(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
                Priority::Max => {
                    let mut queue = BinaryHeap::new();
                    queue.push(self.root.clone());
                    match self.loop_outbound_max(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
            },
            Transposition::Inbound => match self.priority {
                Priority::Min => {
                    let mut queue = BinaryHeap::new();
                    queue.push(Reverse(self.root.clone()));
                    match self.loop_inbound_min(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
                Priority::Max => {
                    let mut queue = BinaryHeap::new();
                    queue.push(self.root.clone());
                    match self.loop_inbound_max(&mut edges, &mut visited, &mut queue) {
                        true => Some(Path::from_edge_tree(edges)),
                        false => None,
                    }
                }
            },
        }
    }
}
