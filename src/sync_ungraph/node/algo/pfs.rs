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

    pub fn target(mut self, target: &'a K) -> Self {
        self.target = Some(target.clone());
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

    fn loop_min(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut BinaryHeap<Reverse<Node<K, N, E>>>,
    ) -> bool {
        while let Some(Reverse(node)) = queue.pop() {
            for edge in node.iter() {
                if self.method.exec(&edge) {
                    let v = edge.1.clone();
                    if !visited.contains(v.key()) {
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return true;
                            }
                        }
                        visited.insert(v.key().clone());
                        result.push(edge);
                        queue.push(Reverse(v));
                    }
                }
            }
        }
        false
    }

    fn loop_max(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut BinaryHeap<Node<K, N, E>>,
    ) -> bool {
        while let Some(node) = queue.pop() {
            for edge in node.iter() {
                if self.method.exec(&edge) {
                    let v = edge.1.clone();
                    if !visited.contains(v.key()) {
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return true;
                            }
                        }
                        visited.insert(v.key().clone());
                        result.push(edge);
                        queue.push(v);
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

        match self.priority {
            Priority::Min => {
                let mut queue = BinaryHeap::new();
                queue.push(Reverse(self.root.clone()));
                match self.loop_min(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
            Priority::Max => {
                let mut queue = BinaryHeap::new();
                queue.push(self.root.clone());
                match self.loop_max(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
        }
    }

    pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
        let mut edges = vec![];
        let mut visited = HashSet::default();

        visited.insert(self.root.key().clone());

        match self.priority {
            Priority::Min => {
                let mut queue = BinaryHeap::new();
                queue.push(Reverse(self.root.clone()));
                match self.loop_min(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
            Priority::Max => {
                let mut queue = BinaryHeap::new();
                queue.push(self.root.clone());
                match self.loop_max(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
        }
    }
}
