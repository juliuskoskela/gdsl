//==== Includes ===============================================================

use std::{fmt::Display, hash::Hash};

use ahash::HashSet;

use super::method::*;
use super::*;

//==== Ordering ===============================================================

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
}

impl<'a, K, N, E> Order<'a, K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    pub fn new(root: &'a Node<K, N, E>) -> Self {
        Self {
            root,
            method: Method::Empty,
            order: Ordering::Pre,
        }
    }

    pub fn pre(mut self) -> Self {
        self.order = Ordering::Pre;
        self
    }

    pub fn post(mut self) -> Self {
        self.order = Ordering::Post;
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

        match self.order {
            Ordering::Pre => {
                self.recurse_preorder(&mut edges, &mut visited, &mut queue);
                nodes.push(self.root.clone());
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
            }
            Ordering::Post => {
                self.recurse_postorder(&mut edges, &mut visited, &mut queue);
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
                nodes.push(self.root.clone());
            }
        }
        nodes
    }

    pub fn search_edges(&mut self) -> Vec<Edge<K, N, E>> {
        let mut edges = vec![];
        let mut queue = vec![];
        let mut visited = HashSet::default();

        queue.push(self.root.clone());
        visited.insert(self.root.key().clone());

        match self.order {
            Ordering::Pre => {
                self.recurse_preorder(&mut edges, &mut visited, &mut queue);
            }
            Ordering::Post => {
                self.recurse_postorder(&mut edges, &mut visited, &mut queue);
            }
        }
        edges
    }

    fn recurse_preorder(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) -> bool {
        if let Some(node) = queue.pop() {
            for edge in node.iter() {
                let edge = edge.reverse();
                let v = edge.1.clone();
                if self.method.exec(&edge) {
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        queue.push(v.clone());
                        result.push(edge);
                        self.recurse_preorder(result, visited, queue);
                    }
                }
            }
        }
        false
    }

    fn recurse_postorder(
        &mut self,
        result: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) -> bool {
        if let Some(node) = queue.pop() {
            for edge in node.iter() {
                let v = edge.1.clone();
                if self.method.exec(&edge) {
                    if !visited.contains(v.key()) {
                        visited.insert(v.key().clone());
                        queue.push(v.clone());
                        self.recurse_postorder(result, visited, queue);
                        result.push(edge);
                    }
                }
            }
        }
        false
    }
}
