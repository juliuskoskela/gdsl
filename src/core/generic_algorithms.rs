//! # Generic Graph Algorithms
//!
//! This module demonstrates how to implement graph algorithms generically
//! to eliminate code duplication between different graph implementations.

use crate::digraph;
use crate::ungraph;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

/// Generic DFS implementation that works with any node type that has the required methods
pub fn generic_dfs_preorder<N, E, F>(
    start: &N,
    mut get_neighbors: F,
) -> Vec<N>
where
    N: Clone + Hash + Eq,
    F: FnMut(&N) -> Vec<N>,
{
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    
    stack.push(start.clone());
    
    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            visited.insert(node.clone());
            result.push(node.clone());
            
            // Add neighbors in reverse order to maintain consistent traversal
            let mut neighbors = get_neighbors(&node);
            neighbors.reverse();
            stack.extend(neighbors);
        }
    }
    
    result
}

/// Generic BFS implementation
pub fn generic_bfs<N, F>(
    start: &N,
    mut get_neighbors: F,
) -> Vec<N>
where
    N: Clone + Hash + Eq,
    F: FnMut(&N) -> Vec<N>,
{
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start.clone());
    visited.insert(start.clone());
    
    while let Some(node) = queue.pop_front() {
        result.push(node.clone());
        
        for neighbor in get_neighbors(&node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back(neighbor);
            }
        }
    }
    
    result
}

/// Generic Dijkstra's algorithm
pub fn generic_dijkstra<N, F>(
    start: &N,
    end: &N,
    mut get_neighbors_with_weights: F,
) -> Option<(Vec<N>, f64)>
where
    N: Clone + Hash + Eq,
    F: FnMut(&N) -> Vec<(N, f64)>,
{
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    
    #[derive(Clone)]
    struct State<N> {
        cost: f64,
        node: N,
    }
    
    impl<N> PartialEq for State<N> {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost
        }
    }
    
    impl<N> Eq for State<N> {}
    
    impl<N> PartialOrd for State<N> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl<N> Ord for State<N> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
        }
    }
    
    let mut distances = HashMap::new();
    let mut previous: HashMap<N, N> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    distances.insert(start.clone(), 0.0);
    heap.push(State { cost: 0.0, node: start.clone() });
    
    while let Some(State { cost, node }) = heap.pop() {
        if node == *end {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = end.clone();
            
            loop {
                path.push(current.clone());
                if let Some(prev) = previous.get(&current) {
                    current = prev.clone();
                } else {
                    break;
                }
            }
            
            path.reverse();
            return Some((path, cost));
        }
        
        if cost > *distances.get(&node).unwrap_or(&f64::INFINITY) {
            continue;
        }
        
        for (neighbor, weight) in get_neighbors_with_weights(&node) {
            let next_cost = cost + weight;
            
            if next_cost < *distances.get(&neighbor).unwrap_or(&f64::INFINITY) {
                distances.insert(neighbor.clone(), next_cost);
                previous.insert(neighbor.clone(), node.clone());
                heap.push(State { cost: next_cost, node: neighbor });
            }
        }
    }
    
    None
}

/// Extension trait for digraph nodes that provides generic algorithm access
pub trait DigraphAlgorithms<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    /// Generic DFS preorder traversal
    fn generic_dfs_preorder(&self) -> Vec<digraph::Node<K, N, E>>;
    
    /// Generic BFS traversal
    fn generic_bfs(&self) -> Vec<digraph::Node<K, N, E>>;
    
    /// Generic DFS with custom neighbor function
    fn generic_dfs_with<F>(&self, get_neighbors: F) -> Vec<digraph::Node<K, N, E>>
    where
        F: FnMut(&digraph::Node<K, N, E>) -> Vec<digraph::Node<K, N, E>>;
}

impl<K, N, E> DigraphAlgorithms<K, N, E> for digraph::Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    fn generic_dfs_preorder(&self) -> Vec<digraph::Node<K, N, E>> {
        // Use keys for the generic algorithm, then map back to nodes
        let start_key = self.key().clone();
        let mut node_map = HashMap::new();
        let mut to_visit = vec![self.clone()];
        let mut visited_nodes = HashMap::new();
        
        // Build a map of all reachable nodes
        while let Some(node) = to_visit.pop() {
            let key = node.key().clone();
            if !visited_nodes.contains_key(&key) {
                visited_nodes.insert(key.clone(), node.clone());
                node_map.insert(key, node.clone());
                for edge in node.iter_out() {
                    to_visit.push(edge.1.clone());
                }
            }
        }
        
        // Now run DFS on keys
        let key_result = generic_dfs_preorder::<K, (), _>(&start_key, |key| {
            if let Some(node) = node_map.get(key) {
                node.iter_out().map(|edge| edge.1.key().clone()).collect()
            } else {
                vec![]
            }
        });
        
        // Map keys back to nodes
        key_result.into_iter()
            .filter_map(|key| visited_nodes.get(&key).cloned())
            .collect()
    }
    
    fn generic_bfs(&self) -> Vec<digraph::Node<K, N, E>> {
        // Similar approach for BFS
        let start_key = self.key().clone();
        let mut node_map = HashMap::new();
        let mut to_visit = vec![self.clone()];
        let mut visited_nodes = HashMap::new();
        
        // Build a map of all reachable nodes
        while let Some(node) = to_visit.pop() {
            let key = node.key().clone();
            if !visited_nodes.contains_key(&key) {
                visited_nodes.insert(key.clone(), node.clone());
                node_map.insert(key, node.clone());
                for edge in node.iter_out() {
                    to_visit.push(edge.1.clone());
                }
            }
        }
        
        // Now run BFS on keys
        let key_result = generic_bfs::<K, _>(&start_key, |key| {
            if let Some(node) = node_map.get(key) {
                node.iter_out().map(|edge| edge.1.key().clone()).collect()
            } else {
                vec![]
            }
        });
        
        // Map keys back to nodes
        key_result.into_iter()
            .filter_map(|key| visited_nodes.get(&key).cloned())
            .collect()
    }
    
    fn generic_dfs_with<F>(&self, mut get_neighbors: F) -> Vec<digraph::Node<K, N, E>>
    where
        F: FnMut(&digraph::Node<K, N, E>) -> Vec<digraph::Node<K, N, E>>,
    {
        // For custom neighbor functions, we need to work directly with nodes
        // This is a simplified DFS implementation
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        
        stack.push(self.clone());
        
        while let Some(node) = stack.pop() {
            let key = node.key().clone();
            if !visited.contains(&key) {
                visited.insert(key);
                result.push(node.clone());
                
                let mut neighbors = get_neighbors(&node);
                neighbors.reverse();
                stack.extend(neighbors);
            }
        }
        
        result
    }
}

/// Extension trait for ungraph nodes that provides generic algorithm access
pub trait UngraphAlgorithms<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    /// Generic DFS preorder traversal
    fn generic_dfs_preorder(&self) -> Vec<ungraph::Node<K, N, E>>;
    
    /// Generic BFS traversal
    fn generic_bfs(&self) -> Vec<ungraph::Node<K, N, E>>;
}

impl<K, N, E> UngraphAlgorithms<K, N, E> for ungraph::Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    fn generic_dfs_preorder(&self) -> Vec<ungraph::Node<K, N, E>> {
        // Use keys for the generic algorithm, then map back to nodes
        let start_key = self.key().clone();
        let mut node_map = HashMap::new();
        let mut to_visit = vec![self.clone()];
        let mut visited_nodes = HashMap::new();
        
        // Build a map of all reachable nodes
        while let Some(node) = to_visit.pop() {
            let key = node.key().clone();
            if !visited_nodes.contains_key(&key) {
                visited_nodes.insert(key.clone(), node.clone());
                node_map.insert(key, node.clone());
                for edge in node.iter() {
                    let neighbor = if edge.0.key() == node.key() {
                        edge.1.clone()
                    } else {
                        edge.0.clone()
                    };
                    to_visit.push(neighbor);
                }
            }
        }
        
        // Now run DFS on keys
        let key_result = generic_dfs_preorder::<K, (), _>(&start_key, |key| {
            if let Some(node) = node_map.get(key) {
                node.iter().map(|edge| {
                    if edge.0.key() == node.key() {
                        edge.1.key().clone()
                    } else {
                        edge.0.key().clone()
                    }
                }).collect()
            } else {
                vec![]
            }
        });
        
        // Map keys back to nodes
        key_result.into_iter()
            .filter_map(|key| visited_nodes.get(&key).cloned())
            .collect()
    }
    
    fn generic_bfs(&self) -> Vec<ungraph::Node<K, N, E>> {
        // Similar approach for BFS
        let start_key = self.key().clone();
        let mut node_map = HashMap::new();
        let mut to_visit = vec![self.clone()];
        let mut visited_nodes = HashMap::new();
        
        // Build a map of all reachable nodes
        while let Some(node) = to_visit.pop() {
            let key = node.key().clone();
            if !visited_nodes.contains_key(&key) {
                visited_nodes.insert(key.clone(), node.clone());
                node_map.insert(key, node.clone());
                for edge in node.iter() {
                    let neighbor = if edge.0.key() == node.key() {
                        edge.1.clone()
                    } else {
                        edge.0.clone()
                    };
                    to_visit.push(neighbor);
                }
            }
        }
        
        // Now run BFS on keys
        let key_result = generic_bfs::<K, _>(&start_key, |key| {
            if let Some(node) = node_map.get(key) {
                node.iter().map(|edge| {
                    if edge.0.key() == node.key() {
                        edge.1.key().clone()
                    } else {
                        edge.0.key().clone()
                    }
                }).collect()
            } else {
                vec![]
            }
        });
        
        // Map keys back to nodes
        key_result.into_iter()
            .filter_map(|key| visited_nodes.get(&key).cloned())
            .collect()
    }
}

/// Demonstrates how the same algorithm can work with different graph types
pub fn demonstrate_generic_algorithms() {
    // Create a digraph
    let d1 = digraph::Node::new(1, "A");
    let d2 = digraph::Node::new(2, "B");
    let d3 = digraph::Node::new(3, "C");
    
    d1.connect(&d2, 1.0);
    d2.connect(&d3, 2.0);
    d3.connect(&d1, 3.0);
    
    // Create an ungraph
    let u1 = ungraph::Node::new(1, "A");
    let u2 = ungraph::Node::new(2, "B");
    let u3 = ungraph::Node::new(3, "C");
    
    u1.connect(&u2, 1.0);
    u2.connect(&u3, 2.0);
    u3.connect(&u1, 3.0);
    
    // Use the same generic algorithm for both
    println!("Digraph DFS: {:?}", d1.generic_dfs_preorder().iter().map(|n| n.key()).collect::<Vec<_>>());
    println!("Ungraph DFS: {:?}", u1.generic_dfs_preorder().iter().map(|n| n.key()).collect::<Vec<_>>());
    
    println!("Digraph BFS: {:?}", d1.generic_bfs().iter().map(|n| n.key()).collect::<Vec<_>>());
    println!("Ungraph BFS: {:?}", u1.generic_bfs().iter().map(|n| n.key()).collect::<Vec<_>>());
}