//! # Adapters for Existing Graph Types
//!
//! This module provides adapters that make the existing graph implementations
//! work with the new generic trait system, eliminating code duplication.

use super::traits::*;
use crate::digraph;
use crate::ungraph;
use std::fmt::Display;
use std::hash::Hash;

/// Adapter for digraph nodes to work with the generic trait system
impl<K, N, E> GraphNode for digraph::Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    type Key = K;
    type Value = N;
    type EdgeValue = E;
    type NodeId = K; // Use key as ID for existing implementation
    
    fn key(&self) -> &Self::Key {
        self.key()
    }
    
    fn value(&self) -> &Self::Value {
        self.value()
    }
    
    fn id(&self) -> Self::NodeId {
        self.key().clone()
    }
}

/// Adapter for digraph edges to work with the generic trait system
impl<K, N, E> GraphEdge for digraph::Edge<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    type Node = digraph::Node<K, N, E>;
    type EdgeValue = E;
    
    fn source(&self) -> &Self::Node {
        &self.0
    }
    
    fn target(&self) -> &Self::Node {
        &self.1
    }
    
    fn value(&self) -> &Self::EdgeValue {
        &self.2
    }
    
    fn reverse(&self) -> Self {
        self.reverse()
    }
}

/// Adapter for ungraph nodes to work with the generic trait system
impl<K, N, E> GraphNode for ungraph::Node<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    type Key = K;
    type Value = N;
    type EdgeValue = E;
    type NodeId = K; // Use key as ID for existing implementation
    
    fn key(&self) -> &Self::Key {
        self.key()
    }
    
    fn value(&self) -> &Self::Value {
        self.value()
    }
    
    fn id(&self) -> Self::NodeId {
        self.key().clone()
    }
}

/// Adapter for ungraph edges to work with the generic trait system
impl<K, N, E> GraphEdge for ungraph::Edge<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    type Node = ungraph::Node<K, N, E>;
    type EdgeValue = E;
    
    fn source(&self) -> &Self::Node {
        &self.0
    }
    
    fn target(&self) -> &Self::Node {
        &self.1
    }
    
    fn value(&self) -> &Self::EdgeValue {
        &self.2
    }
    
    fn reverse(&self) -> Self {
        self.reverse()
    }
}

/// Wrapper that makes a digraph node work as a directed graph
pub struct DigraphNodeWrapper<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    root: digraph::Node<K, N, E>,
}

impl<K, N, E> DigraphNodeWrapper<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    pub fn new(root: digraph::Node<K, N, E>) -> Self {
        Self { root }
    }
}

impl<K, N, E> DirectedGraph for DigraphNodeWrapper<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    type Node = digraph::Node<K, N, E>;
    type Edge = digraph::Edge<K, N, E>;
    type NodeIter = std::iter::Empty<Self::Node>; // Simplified for now
    type EdgeIter = std::iter::Empty<Self::Edge>; // Simplified for now
    type OutEdgeIter = crate::digraph::IterOut<'static, K, N, E>;
    type InEdgeIter = crate::digraph::IterIn<'static, K, N, E>;
    
    fn nodes(&self) -> Self::NodeIter {
        std::iter::empty() // Would need graph traversal to implement properly
    }
    
    fn edges(&self) -> Self::EdgeIter {
        std::iter::empty() // Would need graph traversal to implement properly
    }
    
    fn out_edges(&self, node: &Self::Node) -> Self::OutEdgeIter {
        node.iter_out()
    }
    
    fn in_edges(&self, node: &Self::Node) -> Self::InEdgeIter {
        node.iter_in()
    }
    
    fn get_node(&self, key: &K) -> Option<Self::Node> {
        // This would require a full graph traversal in the current implementation
        // For now, just check if it's the root node
        if self.root.key() == key {
            Some(self.root.clone())
        } else {
            None
        }
    }
    
    fn contains_node(&self, key: &K) -> bool {
        self.get_node(key).is_some()
    }
    
    fn node_count(&self) -> usize {
        1 // Simplified - would need full traversal
    }
    
    fn edge_count(&self) -> usize {
        0 // Simplified - would need full traversal
    }
}

/// Wrapper that makes an ungraph node work as an undirected graph
pub struct UngraphNodeWrapper<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    root: ungraph::Node<K, N, E>,
}

impl<K, N, E> UngraphNodeWrapper<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    pub fn new(root: ungraph::Node<K, N, E>) -> Self {
        Self { root }
    }
}

impl<K, N, E> UndirectedGraph for UngraphNodeWrapper<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    type Node = ungraph::Node<K, N, E>;
    type Edge = ungraph::Edge<K, N, E>;
    type NodeIter = std::iter::Empty<Self::Node>; // Simplified for now
    type EdgeIter = std::iter::Empty<Self::Edge>; // Simplified for now
    type AdjacentEdgeIter = crate::ungraph::NodeIterator<'static, K, N, E>;
    
    fn nodes(&self) -> Self::NodeIter {
        std::iter::empty() // Would need graph traversal to implement properly
    }
    
    fn edges(&self) -> Self::EdgeIter {
        std::iter::empty() // Would need graph traversal to implement properly
    }
    
    fn adjacent_edges(&self, node: &Self::Node) -> Self::AdjacentEdgeIter {
        node.iter()
    }
    
    fn get_node(&self, key: &K) -> Option<Self::Node> {
        // This would require a full graph traversal in the current implementation
        // For now, just check if it's the root node
        if self.root.key() == key {
            Some(self.root.clone())
        } else {
            None
        }
    }
    
    fn contains_node(&self, key: &K) -> bool {
        self.get_node(key).is_some()
    }
    
    fn node_count(&self) -> usize {
        1 // Simplified - would need full traversal
    }
    
    fn edge_count(&self) -> usize {
        0 // Simplified - would need full traversal
    }
}