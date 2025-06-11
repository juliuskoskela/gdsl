//! # Core Graph Traits
//!
//! This module defines the core traits for graph data structures, solving the
//! circular dependency issue by using associated types and trait objects.

use std::fmt::Display;
use std::hash::Hash;

/// Core trait for graph nodes that avoids circular dependencies
pub trait GraphNode: Clone {
    type Key: Clone + Hash + PartialEq + Eq + Display;
    type Value: Clone;
    type NodeId: Clone + PartialEq + Eq + Hash;
    
    /// Get the node's key
    fn key(&self) -> &Self::Key;
    
    /// Get the node's value
    fn value(&self) -> &Self::Value;
    
    /// Get a unique identifier for this node
    fn id(&self) -> Self::NodeId;
}

/// Trait for graph edges
pub trait GraphEdge: Clone {
    type Node: GraphNode;
    type EdgeValue: Clone;
    
    /// Get the source node
    fn source(&self) -> &Self::Node;
    
    /// Get the target node  
    fn target(&self) -> &Self::Node;
    
    /// Get the edge value
    fn value(&self) -> &Self::EdgeValue;
    
    /// Create a reversed edge
    fn reverse(&self) -> Self where Self: Sized;
}

/// Core trait for directed graphs
pub trait DirectedGraph {
    type Node: GraphNode;
    type Edge: GraphEdge<Node = Self::Node>;
    type NodeIter: Iterator<Item = Self::Node>;
    type EdgeIter: Iterator<Item = Self::Edge>;
    type OutEdgeIter: Iterator<Item = Self::Edge>;
    type InEdgeIter: Iterator<Item = Self::Edge>;
    
    /// Get all nodes in the graph
    fn nodes(&self) -> Self::NodeIter;
    
    /// Get all edges in the graph
    fn edges(&self) -> Self::EdgeIter;
    
    /// Get outbound edges from a node
    fn out_edges(&self, node: &Self::Node) -> Self::OutEdgeIter;
    
    /// Get inbound edges to a node
    fn in_edges(&self, node: &Self::Node) -> Self::InEdgeIter;
    
    /// Get a node by its key
    fn get_node(&self, key: &<Self::Node as GraphNode>::Key) -> Option<Self::Node>;
    
    /// Check if the graph contains a node
    fn contains_node(&self, key: &<Self::Node as GraphNode>::Key) -> bool;
    
    /// Get the number of nodes
    fn node_count(&self) -> usize;
    
    /// Get the number of edges
    fn edge_count(&self) -> usize;
}

/// Core trait for undirected graphs
pub trait UndirectedGraph {
    type Node: GraphNode;
    type Edge: GraphEdge<Node = Self::Node>;
    type NodeIter: Iterator<Item = Self::Node>;
    type EdgeIter: Iterator<Item = Self::Edge>;
    type AdjacentEdgeIter: Iterator<Item = Self::Edge>;
    
    /// Get all nodes in the graph
    fn nodes(&self) -> Self::NodeIter;
    
    /// Get all edges in the graph
    fn edges(&self) -> Self::EdgeIter;
    
    /// Get adjacent edges from a node
    fn adjacent_edges(&self, node: &Self::Node) -> Self::AdjacentEdgeIter;
    
    /// Get a node by its key
    fn get_node(&self, key: &<Self::Node as GraphNode>::Key) -> Option<Self::Node>;
    
    /// Check if the graph contains a node
    fn contains_node(&self, key: &<Self::Node as GraphNode>::Key) -> bool;
    
    /// Get the number of nodes
    fn node_count(&self) -> usize;
    
    /// Get the number of edges
    fn edge_count(&self) -> usize;
}

/// Trait for mutable directed graphs
pub trait MutableDirectedGraph: DirectedGraph {
    /// Add a node to the graph
    fn add_node(&mut self, key: <Self::Node as GraphNode>::Key, value: <Self::Node as GraphNode>::Value) -> Self::Node;
    
    /// Remove a node from the graph
    fn remove_node(&mut self, key: &<Self::Node as GraphNode>::Key) -> Option<Self::Node>;
    
    /// Add an edge to the graph
    fn add_edge(&mut self, source: &Self::Node, target: &Self::Node, value: <Self::Edge as GraphEdge>::EdgeValue) -> Self::Edge;
    
    /// Remove an edge from the graph
    fn remove_edge(&mut self, source: &Self::Node, target: &Self::Node) -> Option<Self::Edge>;
}

/// Trait for mutable undirected graphs
pub trait MutableUndirectedGraph: UndirectedGraph {
    /// Add a node to the graph
    fn add_node(&mut self, key: <Self::Node as GraphNode>::Key, value: <Self::Node as GraphNode>::Value) -> Self::Node;
    
    /// Remove a node from the graph
    fn remove_node(&mut self, key: &<Self::Node as GraphNode>::Key) -> Option<Self::Node>;
    
    /// Add an edge to the graph
    fn add_edge(&mut self, source: &Self::Node, target: &Self::Node, value: <Self::Edge as GraphEdge>::EdgeValue) -> Self::Edge;
    
    /// Remove an edge from the graph
    fn remove_edge(&mut self, source: &Self::Node, target: &Self::Node) -> Option<Self::Edge>;
}

/// Ordering types for graph traversal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    PreOrder,
    PostOrder,
    InOrder,
    LevelOrder,
}

/// Transposition types for directed graph traversal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transposition {
    Outbound,
    Inbound,
}