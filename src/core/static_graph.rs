//! # Static Graph Implementation
//!
//! This module provides a high-performance static graph implementation that
//! doesn't use smart pointers and is optimized for read-only operations.

use super::traits::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/// A simple static directed graph that uses indices instead of smart pointers
#[derive(Debug, Clone)]
pub struct StaticDirectedGraph<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    nodes: Vec<StaticNode<K, N>>,
    edges: Vec<StaticEdge<E>>,
    key_to_index: HashMap<K, usize>,
    out_edges: Vec<Vec<usize>>, // out_edges[node_index] = vec of edge indices
    in_edges: Vec<Vec<usize>>,  // in_edges[node_index] = vec of edge indices
}

impl<K, N, E> StaticDirectedGraph<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    /// Create a new empty static directed graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            key_to_index: HashMap::new(),
            out_edges: Vec::new(),
            in_edges: Vec::new(),
        }
    }
    
    /// Add a node to the graph
    pub fn add_node(&mut self, key: K, value: N) -> usize {
        if let Some(&index) = self.key_to_index.get(&key) {
            return index;
        }
        
        let index = self.nodes.len();
        self.nodes.push(StaticNode {
            key: key.clone(),
            value,
            index,
        });
        self.key_to_index.insert(key, index);
        self.out_edges.push(Vec::new());
        self.in_edges.push(Vec::new());
        
        index
    }
    
    /// Add an edge to the graph
    pub fn add_edge(&mut self, source_key: &K, target_key: &K, value: E) -> Option<usize> {
        let source_index = *self.key_to_index.get(source_key)?;
        let target_index = *self.key_to_index.get(target_key)?;
        
        let edge_index = self.edges.len();
        self.edges.push(StaticEdge {
            source_index,
            target_index,
            value,
        });
        
        self.out_edges[source_index].push(edge_index);
        self.in_edges[target_index].push(edge_index);
        
        Some(edge_index)
    }
    
    /// Get a node by key
    pub fn get_node(&self, key: &K) -> Option<&StaticNode<K, N>> {
        let index = *self.key_to_index.get(key)?;
        self.nodes.get(index)
    }
    
    /// Get all nodes
    pub fn nodes(&self) -> &[StaticNode<K, N>] {
        &self.nodes
    }
    
    /// Get all edges
    pub fn edges(&self) -> &[StaticEdge<E>] {
        &self.edges
    }
    
    /// Get outbound edges for a node
    pub fn out_edges(&self, node_index: usize) -> &[usize] {
        &self.out_edges[node_index]
    }
    
    /// Get inbound edges for a node
    pub fn in_edges(&self, node_index: usize) -> &[usize] {
        &self.in_edges[node_index]
    }
}

/// A static node that uses indices instead of pointers
#[derive(Debug, Clone)]
pub struct StaticNode<K, N>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
{
    key: K,
    value: N,
    index: usize,
}

/// A static edge that uses indices instead of pointers
#[derive(Debug, Clone)]
pub struct StaticEdge<E>
where
    E: Clone,
{
    source_index: usize,
    target_index: usize,
    value: E,
}

impl<K, N> GraphNode for StaticNode<K, N>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
{
    type Key = K;
    type Value = N;
    type NodeId = usize;
    
    fn key(&self) -> &Self::Key {
        &self.key
    }
    
    fn value(&self) -> &Self::Value {
        &self.value
    }
    
    fn id(&self) -> Self::NodeId {
        self.index
    }
}


