use super::{method::*, *};
use ahash::AHashSet as HashSet;
use std::{fmt::Display, hash::Hash};

#[derive(Clone, Copy)]
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

    pub fn postorder(root: &'a Node<K, N, E>) -> Self {
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
        let edges = self.search_edges();
        
        match (self.transpose, self.order) {
            (Transposition::Outbound, Ordering::Pre) => {
                // Preorder: root first, then children
                nodes.push(self.root.clone());
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
            }
            (Transposition::Outbound, Ordering::Post) => {
                // Postorder: children first, then root
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
                nodes.push(self.root.clone());
            }
            (Transposition::Inbound, Ordering::Pre) => {
                // Preorder: root first, then children
                nodes.push(self.root.clone());
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
            }
            (Transposition::Inbound, Ordering::Post) => {
                // Postorder: children first, then root
                let mut coll = edges.iter().map(|Edge(_, v, _)| v.clone()).collect();
                nodes.append(&mut coll);
                nodes.push(self.root.clone());
            }
        }
        
        nodes
    }

    pub fn search_edges(&mut self) -> Vec<Edge<K, N, E>> {
        match (self.transpose, self.order) {
            (Transposition::Outbound, Ordering::Pre) => self.dfs_preorder_edges_outbound(),
            (Transposition::Outbound, Ordering::Post) => self.dfs_postorder_edges_outbound(),
            (Transposition::Inbound, Ordering::Pre) => self.dfs_preorder_edges_inbound(),
            (Transposition::Inbound, Ordering::Post) => self.dfs_postorder_edges_inbound(),
        }
    }


    
    // DFS Pre-order traversal for edges (outbound)
    fn dfs_preorder_edges_outbound(&mut self) -> Vec<Edge<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut stack = Vec::new();
        
        stack.push(self.root.clone());
        visited.insert(self.root.key().clone());
        
        while let Some(node) = stack.pop() {
            // Add neighbors to stack and collect edges
            let mut neighbors = Vec::new();
            for edge in node.iter_out() {
                let neighbor = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                    visited.insert(neighbor.key().clone());
                    result.push(edge);
                    neighbors.push(neighbor);
                }
            }
            // Reverse to maintain consistent order
            neighbors.reverse();
            stack.extend(neighbors);
        }
        
        result
    }
    
    // DFS Post-order traversal for edges (outbound)
    fn dfs_postorder_edges_outbound(&mut self) -> Vec<Edge<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut stack = Vec::new();
        let mut edge_stack = Vec::new();
        
        stack.push(self.root.clone());
        visited.insert(self.root.key().clone());
        
        while let Some(node) = stack.pop() {
            // Add neighbors to stack and collect edges
            for edge in node.iter_out() {
                let neighbor = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                    visited.insert(neighbor.key().clone());
                    edge_stack.push(edge);
                    stack.push(neighbor);
                }
            }
        }
        
        // Post-order means we process edges in reverse order
        edge_stack.reverse();
        result.extend(edge_stack);
        result
    }
    
    // DFS Pre-order traversal for edges (inbound) - recursive approach to match original
    fn dfs_preorder_edges_inbound(&mut self) -> Vec<Edge<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        visited.insert(self.root.key().clone());
        self.dfs_preorder_inbound_recursive(self.root.clone(), &mut result, &mut visited);
        result
    }
    
    fn dfs_preorder_inbound_recursive(&mut self, node: Node<K, N, E>, result: &mut Vec<Edge<K, N, E>>, visited: &mut HashSet<K>) {
        for edge in node.iter_in() {
            let neighbor = edge.0.clone();
            let reversed_edge = edge.reverse();
            if self.method.exec(&reversed_edge) && !visited.contains(neighbor.key()) {
                visited.insert(neighbor.key().clone());
                result.push(reversed_edge);
                self.dfs_preorder_inbound_recursive(neighbor, result, visited);
            }
        }
    }
    
    // DFS Post-order traversal for edges (inbound) - recursive approach to match original
    fn dfs_postorder_edges_inbound(&mut self) -> Vec<Edge<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        visited.insert(self.root.key().clone());
        self.dfs_postorder_inbound_recursive(self.root.clone(), &mut result, &mut visited);
        result
    }
    
    fn dfs_postorder_inbound_recursive(&mut self, node: Node<K, N, E>, result: &mut Vec<Edge<K, N, E>>, visited: &mut HashSet<K>) {
        for edge in node.iter_in() {
            let neighbor = edge.0.clone();
            let reversed_edge = edge.reverse();
            if self.method.exec(&reversed_edge) && !visited.contains(neighbor.key()) {
                visited.insert(neighbor.key().clone());
                result.push(reversed_edge);
                self.dfs_postorder_inbound_recursive(neighbor, result, visited);
            }
        }
    }
}
