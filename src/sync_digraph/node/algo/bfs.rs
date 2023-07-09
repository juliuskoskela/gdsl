use super::{method::*, path::*, *};
use ahash::AHashSet as HashSet;
use std::{collections::VecDeque, fmt::Display, hash::Hash};

pub struct Bfs<'a, K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    root: Node<K, N, E>,
    target: Option<K>,
    method: Method<'a, K, N, E>,
    transpose: Transposition,
}

impl<'a, K, N, E> Bfs<'a, K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    pub fn new(root: &Node<K, N, E>) -> Self {
        Bfs {
            root: root.clone(),
            target: None,
            method: Method::Empty,
            transpose: Transposition::Outbound,
        }
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

    pub fn search(&'a mut self) -> Option<Node<K, N, E>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::default();

        queue.push_back(self.root.clone());
        visited.insert(self.root.key().clone());

        match self.transpose {
            Transposition::Outbound => self.loop_outbound_find(&mut visited, &mut queue),
            Transposition::Inbound => self.loop_inbound_find(&mut visited, &mut queue),
        }
    }

    pub fn search_cycle(&'a mut self) -> Option<Path<K, N, E>> {
        let mut edges = vec![];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::default();

        self.target = Some(self.root.key().clone());
        queue.push_back(self.root.clone());

        match self.transpose {
            Transposition::Outbound => {
                match self.loop_outbound(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
            Transposition::Inbound => {
                match self.loop_inbound(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
        }
    }

    pub fn search_path(&mut self) -> Option<Path<K, N, E>> {
        let mut edges = vec![];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::default();

        queue.push_back(self.root.clone());
        visited.insert(self.root.key().clone());

        match self.transpose {
            Transposition::Outbound => {
                match self.loop_outbound(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
            Transposition::Inbound => {
                match self.loop_inbound(&mut edges, &mut visited, &mut queue) {
                    true => Some(Path::from_edge_tree(edges)),
                    false => None,
                }
            }
        }
    }

    fn loop_outbound(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut VecDeque<Node<K, N, E>>,
    ) -> bool {
        while let Some(node) = queue.pop_front() {
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
                        queue.push_back(v.clone());
                    }
                }
            }
        }
        false
    }

    fn loop_inbound(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut VecDeque<Node<K, N, E>>,
    ) -> bool {
        while let Some(node) = queue.pop_front() {
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
                        queue.push_back(v.clone());
                    }
                }
            }
        }
        false
    }

    fn loop_outbound_find(
        &mut self,
        visited: &mut HashSet<K>,
        queue: &mut VecDeque<Node<K, N, E>>,
    ) -> Option<Node<K, N, E>> {
        while let Some(node) = queue.pop_front() {
            for edge in node.iter_out() {
                if self.method.exec(&edge) {
                    let Edge(_, v, _) = edge;
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return Some(v);
                            }
                        }
                        queue.push_back(v);
                    }
                }
            }
        }
        None
    }

    fn loop_inbound_find(
        &mut self,
        visited: &mut HashSet<K>,
        queue: &mut VecDeque<Node<K, N, E>>,
    ) -> Option<Node<K, N, E>> {
        while let Some(node) = queue.pop_front() {
            for edge in node.iter_in() {
                let edge = edge.reverse();
                if self.method.exec(&edge) {
                    let Edge(_, v, _) = edge;
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        if let Some(ref t) = self.target {
                            if v.key() == t {
                                return Some(v);
                            }
                        }
                        queue.push_back(v);
                    }
                }
            }
        }
        None
    }
}
