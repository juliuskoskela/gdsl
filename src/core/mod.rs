//! # Core Graph Abstractions
//!
//! This module provides the core abstractions and traits for graph data structures,
//! solving the circular dependency issue by separating the graph interface from
//! the concrete implementations.

pub mod traits;
pub mod static_graph;
pub mod generic_algorithms;

pub use traits::*;
pub use static_graph::*;
pub use generic_algorithms::*;