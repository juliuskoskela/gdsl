//! # Generic Graph Algorithms
//!
//! This module implements graph algorithms generically over the graph traits,
//! eliminating code duplication between different graph implementations.

use super::traits::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Generic depth-first search implementation
pub struct DFS<G> {
    graph: G,
}

impl<G> DFS<G> {
    pub fn new(graph: G) -> Self {
        Self { graph }
    }
}

impl<G> DFS<G>
where
    G: DirectedGraph,
    <G::Node as GraphNode>::NodeId: Hash + Eq,
{
    /// Perform DFS traversal starting from a given node
    pub fn traverse(&self, start: &G::Node, ordering: Ordering, transposition: Transposition) -> Vec<G::Node> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        
        match ordering {
            Ordering::PreOrder => {
                self.dfs_preorder(start, &mut result, &mut visited, transposition);
            }
            Ordering::PostOrder => {
                self.dfs_postorder(start, &mut result, &mut visited, transposition);
            }
            _ => unimplemented!("InOrder and LevelOrder not implemented for DFS"),
        }
        
        result
    }
    
    fn dfs_preorder(&self, node: &G::Node, result: &mut Vec<G::Node>, visited: &mut HashSet<<G::Node as GraphNode>::NodeId>, transposition: Transposition) {
        if visited.contains(&node.id()) {
            return;
        }
        
        visited.insert(node.id());
        result.push(node.clone());
        
        let edges: Box<dyn Iterator<Item = G::Edge>> = match transposition {
            Transposition::Outbound => Box::new(self.graph.out_edges(node)),
            Transposition::Inbound => Box::new(self.graph.in_edges(node)),
        };
        
        for edge in edges {
            let next_node = match transposition {
                Transposition::Outbound => edge.target().clone(),
                Transposition::Inbound => edge.source().clone(),
            };
            self.dfs_preorder(&next_node, result, visited, transposition);
        }
    }
    
    fn dfs_postorder(&self, node: &G::Node, result: &mut Vec<G::Node>, visited: &mut HashSet<<G::Node as GraphNode>::NodeId>, transposition: Transposition) {
        if visited.contains(&node.id()) {
            return;
        }
        
        visited.insert(node.id());
        
        let edges: Box<dyn Iterator<Item = G::Edge>> = match transposition {
            Transposition::Outbound => Box::new(self.graph.out_edges(node)),
            Transposition::Inbound => Box::new(self.graph.in_edges(node)),
        };
        
        for edge in edges {
            let next_node = match transposition {
                Transposition::Outbound => edge.target().clone(),
                Transposition::Inbound => edge.source().clone(),
            };
            self.dfs_postorder(&next_node, result, visited, transposition);
        }
        
        result.push(node.clone());
    }
}

impl<G> DFS<G>
where
    G: UndirectedGraph,
    <G::Node as GraphNode>::NodeId: Hash + Eq,
{
    /// Perform DFS traversal on undirected graph
    pub fn traverse_undirected(&self, start: &G::Node, ordering: Ordering) -> Vec<G::Node> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        
        match ordering {
            Ordering::PreOrder => {
                self.dfs_preorder_undirected(start, &mut result, &mut visited);
            }
            Ordering::PostOrder => {
                self.dfs_postorder_undirected(start, &mut result, &mut visited);
            }
            _ => unimplemented!("InOrder and LevelOrder not implemented for DFS"),
        }
        
        result
    }
    
    fn dfs_preorder_undirected(&self, node: &G::Node, result: &mut Vec<G::Node>, visited: &mut HashSet<<G::Node as GraphNode>::NodeId>) {
        if visited.contains(&node.id()) {
            return;
        }
        
        visited.insert(node.id());
        result.push(node.clone());
        
        for edge in self.graph.adjacent_edges(node) {
            let next_node = if edge.source().id() == node.id() {
                edge.target().clone()
            } else {
                edge.source().clone()
            };
            self.dfs_preorder_undirected(&next_node, result, visited);
        }
    }
    
    fn dfs_postorder_undirected(&self, node: &G::Node, result: &mut Vec<G::Node>, visited: &mut HashSet<<G::Node as GraphNode>::NodeId>) {
        if visited.contains(&node.id()) {
            return;
        }
        
        visited.insert(node.id());
        
        for edge in self.graph.adjacent_edges(node) {
            let next_node = if edge.source().id() == node.id() {
                edge.target().clone()
            } else {
                edge.source().clone()
            };
            self.dfs_postorder_undirected(&next_node, result, visited);
        }
        
        result.push(node.clone());
    }
}

/// Generic breadth-first search implementation
pub struct BFS<G> {
    graph: G,
}

impl<G> BFS<G> {
    pub fn new(graph: G) -> Self {
        Self { graph }
    }
}

impl<G> BFS<G>
where
    G: DirectedGraph,
    <G::Node as GraphNode>::NodeId: Hash + Eq,
{
    /// Perform BFS traversal starting from a given node
    pub fn traverse(&self, start: &G::Node, transposition: Transposition) -> Vec<G::Node> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(start.clone());
        visited.insert(start.id());
        
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            let edges: Box<dyn Iterator<Item = G::Edge>> = match transposition {
                Transposition::Outbound => Box::new(self.graph.out_edges(&node)),
                Transposition::Inbound => Box::new(self.graph.in_edges(&node)),
            };
            
            for edge in edges {
                let next_node = match transposition {
                    Transposition::Outbound => edge.target().clone(),
                    Transposition::Inbound => edge.source().clone(),
                };
                
                if !visited.contains(&next_node.id()) {
                    visited.insert(next_node.id());
                    queue.push_back(next_node);
                }
            }
        }
        
        result
    }
}

impl<G> BFS<G>
where
    G: UndirectedGraph,
    <G::Node as GraphNode>::NodeId: Hash + Eq,
{
    /// Perform BFS traversal on undirected graph
    pub fn traverse_undirected(&self, start: &G::Node) -> Vec<G::Node> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(start.clone());
        visited.insert(start.id());
        
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            for edge in self.graph.adjacent_edges(&node) {
                let next_node = if edge.source().id() == node.id() {
                    edge.target().clone()
                } else {
                    edge.source().clone()
                };
                
                if !visited.contains(&next_node.id()) {
                    visited.insert(next_node.id());
                    queue.push_back(next_node);
                }
            }
        }
        
        result
    }
}

/// Generic shortest path algorithms
pub struct ShortestPath<G> {
    graph: G,
}

impl<G> ShortestPath<G> {
    pub fn new(graph: G) -> Self {
        Self { graph }
    }
}

impl<G> ShortestPath<G>
where
    G: DirectedGraph,
    <G::Node as GraphNode>::NodeId: Hash + Eq,
    <G::Edge as GraphEdge>::EdgeValue: Into<f64> + Clone,
{
    /// Dijkstra's shortest path algorithm
    pub fn dijkstra(&self, start: &G::Node, end: &G::Node) -> Option<(Vec<G::Node>, f64)> {
        use std::collections::BinaryHeap;
        use std::cmp::Ordering;
        
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
        let mut previous: HashMap<<G::Node as GraphNode>::NodeId, G::Node> = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        distances.insert(start.id(), 0.0);
        heap.push(State { cost: 0.0, node: start.clone() });
        
        while let Some(State { cost, node }) = heap.pop() {
            if node.id() == end.id() {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current = end.clone();
                
                loop {
                    path.push(current.clone());
                    if let Some(prev) = previous.get(&current.id()) {
                        current = prev.clone();
                    } else {
                        break;
                    }
                }
                
                path.reverse();
                return Some((path, cost));
            }
            
            if cost > *distances.get(&node.id()).unwrap_or(&f64::INFINITY) {
                continue;
            }
            
            for edge in self.graph.out_edges(&node) {
                let next = edge.target().clone();
                let next_cost = cost + edge.value().clone().into();
                
                if next_cost < *distances.get(&next.id()).unwrap_or(&f64::INFINITY) {
                    distances.insert(next.id(), next_cost);
                    previous.insert(next.id(), node.clone());
                    heap.push(State { cost: next_cost, node: next });
                }
            }
        }
        
        None
    }
}