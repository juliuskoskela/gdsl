use std::{fmt::Display, hash::Hash};

use ahash::HashSet;

use super::method::*;
use super::*;

pub enum Ordering {
    PreOrder,
    PostOrder,
    InOrder,
    LevelOrder,
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
            order: Ordering::PreOrder,
        }
    }

    pub fn pre(mut self) -> Self {
        self.order = Ordering::PreOrder;
        self
    }

    pub fn post(mut self) -> Self {
        self.order = Ordering::PostOrder;
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
            Ordering::PreOrder => {
                self.iterative_preorder(&mut edges, &mut visited, &mut queue);
                nodes.push(self.root.clone());
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
            }
            Ordering::PostOrder => {
                self.iterative_postorder(&mut edges, &mut visited, &mut queue);
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
                nodes.push(self.root.clone());
            }
            Ordering::InOrder => {
                todo!()
            }
            Ordering::LevelOrder => {
                todo!()
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
            Ordering::PreOrder => {
                self.iterative_preorder(&mut edges, &mut visited, &mut queue);
            }
            Ordering::PostOrder => {
                self.iterative_postorder(&mut edges, &mut visited, &mut queue);
            }
            Ordering::InOrder => {
                todo!()
            }
            Ordering::LevelOrder => {
                todo!()
            }
        }

        edges
    }

    fn iterative_preorder(
        &mut self,
        edges: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) {
        while let Some(node) = queue.pop() {
            for edge in node.iter().rev() {
                let edge = edge.reverse();
                let v = edge.1.clone();
                if self.method.exec(&edge) && visited.insert(v.key().clone()) {
                    queue.push(v.clone());
                    edges.push(edge);
                }
            }
        }
    }

    fn iterative_postorder(
        &mut self,
        edges: &mut Vec<Edge<K, N, E>>,
        visited: &mut HashSet<K>,
        queue: &mut Vec<Node<K, N, E>>,
    ) {
        let mut stack = Vec::new();

        while let Some(node) = queue.pop() {
            stack.push(node.clone());
            for edge in node.iter() {
                let v = edge.1.clone();
                if self.method.exec(&edge) && visited.insert(v.key().clone()) {
                    queue.push(v);
                }
            }
        }

        while let Some(node) = stack.pop() {
            for edge in node.iter().rev() {
                edges.push(edge.reverse());
            }
        }
    }
}
