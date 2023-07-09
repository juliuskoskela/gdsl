use super::{method::*, *};
use ahash::AHashSet as HashSet;
use std::{fmt::Display, hash::Hash};

pub enum Ordering {
    Pre,
    Post,
}

pub struct Order<'a, K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    root: &'a Node<K, N, E>,
    method: Method<'a, K, N, E>,
    order: Ordering,
    transpose: Transposition,
}

impl<'a, K, N, E> Order<'a, K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    pub fn preorder(root: &'a Node<K, N, E>) -> Self {
        Self {
            root,
            method: Method::Empty,
            order: Ordering::Pre,
            transpose: Transposition::Outbound,
        }
    }

    pub fn postroder(root: &'a Node<K, N, E>) -> Self {
        Self {
            root,
            method: Method::Empty,
            order: Ordering::Post,
            transpose: Transposition::Inbound,
        }
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

    pub fn search_nodes(&mut self) -> Vec<Node<K, N, E>> {
        let mut nodes = vec![];
        let mut edges = vec![];
        let mut queue = vec![];
        let mut visited = HashSet::default();

        queue.push(self.root.clone());
        visited.insert(self.root.key().clone());

        match self.transpose {
            Transposition::Outbound => match self.order {
                Ordering::Pre => {
                    self.preorder_forward(&mut edges, &mut visited, &mut queue);
                    nodes.push(self.root.clone());
                    let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                    nodes.append(&mut coll);
                }
                Ordering::Post => {
                    self.postorder_forward(&mut edges, &mut visited, &mut queue);
                    let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                    nodes.append(&mut coll);
                    nodes.push(self.root.clone());
                }
            },
            Transposition::Inbound => match self.order {
                Ordering::Pre => {
                    self.preorder_backward(&mut edges, &mut visited, &mut queue);
                    nodes.push(self.root.clone());
                    let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                    nodes.append(&mut coll);
                }
                Ordering::Post => {
                    self.postorder_backward(&mut edges, &mut visited, &mut queue);
                    let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                    nodes.append(&mut coll);
                    nodes.push(self.root.clone());
                }
            },
        }
        nodes
    }

    pub fn search_edges(&mut self) -> Vec<Edge<K, N, E>> {
        let mut edges = vec![];
        let mut queue = vec![];
        let mut visited = HashSet::default();

        queue.push(self.root.clone());
        visited.insert(self.root.key().clone());

        match self.transpose {
            Transposition::Outbound => match self.order {
                Ordering::Pre => {
                    self.preorder_forward(&mut edges, &mut visited, &mut queue);
                }
                Ordering::Post => {
                    self.postorder_forward(&mut edges, &mut visited, &mut queue);
                }
            },
            Transposition::Inbound => match self.order {
                Ordering::Pre => {
                    self.preorder_backward(&mut edges, &mut visited, &mut queue);
                }
                Ordering::Post => {
                    self.postorder_backward(&mut edges, &mut visited, &mut queue);
                }
            },
        }
        edges
    }

    fn preorder_forward(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) -> bool {
        if let Some(node) = queue.pop() {
            for edge in node.iter_out() {
                let v = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(v.key()) {
                    visited.insert(v.key().clone());
                    queue.push(v.clone());
                    result.push(edge);
                    self.preorder_forward(result, visited, queue);
                }
            }
        }
        false
    }

    fn preorder_backward(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) -> bool {
        if let Some(node) = queue.pop() {
            for edge in node.iter_in() {
                let edge = edge.reverse();
                let v = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(v.key()) {
                    visited.insert(v.key().clone());
                    queue.push(v.clone());
                    result.push(edge);
                    self.preorder_backward(result, visited, queue);
                }
            }
        }
        false
    }

    fn postorder_forward(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) -> bool {
        if let Some(node) = queue.pop() {
            for edge in node.iter_out() {
                let v = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(v.key()) {
                    visited.insert(v.key().clone());
                    queue.push(v.clone());
                    result.push(edge);
                    self.postorder_forward(result, visited, queue);
                }
            }
        }
        false
    }

    fn postorder_backward(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) -> bool {
        if let Some(node) = queue.pop() {
            for edge in node.iter_in() {
                let edge = edge.reverse();
                let v = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(v.key()) {
                    visited.insert(v.key().clone());
                    queue.push(v.clone());
                    result.push(edge);
                    self.postorder_backward(result, visited, queue);
                }
            }
        }
        false
    }
}
