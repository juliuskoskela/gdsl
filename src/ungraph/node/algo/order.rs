use std::{fmt::Display, hash::Hash, collections::VecDeque};

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
        match self.order {
            Ordering::PreOrder => self.dfs_preorder_nodes(),
            Ordering::PostOrder => self.dfs_postorder_nodes(),
            Ordering::InOrder => {
                // For graphs, in-order doesn't have a standard definition
                // We'll implement it as a variant of DFS
                self.dfs_preorder_nodes()
            }
            Ordering::LevelOrder => self.bfs_levelorder_nodes(),
        }
    }

    pub fn search_edges(&mut self) -> Vec<Edge<K, N, E>> {
        match self.order {
            Ordering::PreOrder => self.dfs_preorder_edges(),
            Ordering::PostOrder => self.dfs_postorder_edges(),
            Ordering::InOrder => {
                // For graphs, in-order doesn't have a standard definition
                // We'll implement it as a variant of DFS
                self.dfs_preorder_edges()
            }
            Ordering::LevelOrder => self.bfs_levelorder_edges(),
        }
    }

    // DFS Pre-order traversal for nodes
    fn dfs_preorder_nodes(&mut self) -> Vec<Node<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut stack = Vec::new();
        
        stack.push(self.root.clone());
        
        while let Some(node) = stack.pop() {
            if !visited.contains(node.key()) {
                visited.insert(node.key().clone());
                result.push(node.clone());
                
                // Add neighbors to stack (in reverse order for consistent traversal)
                let mut neighbors = Vec::new();
                for edge in node.iter() {
                    let neighbor = edge.1.clone();
                    if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                        neighbors.push(neighbor);
                    }
                }
                // Reverse to maintain consistent order
                neighbors.reverse();
                stack.extend(neighbors);
            }
        }
        
        result
    }
    
    // DFS Post-order traversal for nodes
    fn dfs_postorder_nodes(&mut self) -> Vec<Node<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut stack = Vec::new();
        let mut post_stack = Vec::new();
        
        stack.push(self.root.clone());
        
        while let Some(node) = stack.pop() {
            if !visited.contains(node.key()) {
                visited.insert(node.key().clone());
                post_stack.push(node.clone());
                
                // Add neighbors to stack
                for edge in node.iter() {
                    let neighbor = edge.1.clone();
                    if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                        stack.push(neighbor);
                    }
                }
            }
        }
        
        // Post-order means we process nodes in reverse order of discovery
        post_stack.reverse();
        result.extend(post_stack);
        result
    }
    
    // BFS Level-order traversal for nodes
    fn bfs_levelorder_nodes(&mut self) -> Vec<Node<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut queue = VecDeque::new();
        
        queue.push_back(self.root.clone());
        visited.insert(self.root.key().clone());
        
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            // Add neighbors to queue
            for edge in node.iter() {
                let neighbor = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                    visited.insert(neighbor.key().clone());
                    queue.push_back(neighbor);
                }
            }
        }
        
        result
    }
    
    // DFS Pre-order traversal for edges
    fn dfs_preorder_edges(&mut self) -> Vec<Edge<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut stack = Vec::new();
        
        stack.push(self.root.clone());
        visited.insert(self.root.key().clone());
        
        while let Some(node) = stack.pop() {
            // Add neighbors to stack and collect edges
            let mut neighbors = Vec::new();
            for edge in node.iter() {
                let neighbor = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                    visited.insert(neighbor.key().clone());
                    result.push(edge.reverse());
                    neighbors.push(neighbor);
                }
            }
            // Reverse to maintain consistent order
            neighbors.reverse();
            stack.extend(neighbors);
        }
        
        result
    }
    
    // DFS Post-order traversal for edges
    fn dfs_postorder_edges(&mut self) -> Vec<Edge<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut stack = Vec::new();
        let mut edge_stack = Vec::new();
        
        stack.push(self.root.clone());
        visited.insert(self.root.key().clone());
        
        while let Some(node) = stack.pop() {
            // Add neighbors to stack and collect edges
            for edge in node.iter() {
                let neighbor = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                    visited.insert(neighbor.key().clone());
                    edge_stack.push(edge.reverse());
                    stack.push(neighbor);
                }
            }
        }
        
        // Post-order means we process edges in reverse order
        edge_stack.reverse();
        result.extend(edge_stack);
        result
    }
    
    // BFS Level-order traversal for edges
    fn bfs_levelorder_edges(&mut self) -> Vec<Edge<K, N, E>> {
        let mut result = Vec::new();
        let mut visited = HashSet::default();
        let mut queue = VecDeque::new();
        
        queue.push_back(self.root.clone());
        visited.insert(self.root.key().clone());
        
        while let Some(node) = queue.pop_front() {
            // Add neighbors to queue and collect edges
            for edge in node.iter() {
                let neighbor = edge.1.clone();
                if self.method.exec(&edge) && !visited.contains(neighbor.key()) {
                    visited.insert(neighbor.key().clone());
                    result.push(edge.reverse());
                    queue.push_back(neighbor);
                }
            }
        }
        
        result
    }
}
