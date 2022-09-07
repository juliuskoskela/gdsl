//! # Graph Data Structure Library
//!
//! GDSL is a graph data structure library providing efficient and easy-to-use
//! abstractions for working with connected nodes and graphs. Contrary to many other
//! graph implementations, in GDSL a graph is mostly just a container and the
//! functionality is in the `Node<K, N, E>` structure which can then be used to
//! create different graph representations or be used more freely as a part of
//! some other data structure.
//!
//! - Directed and undirected graph and node types.
//!
//! - Normal and sync versions of the node and graph types. Normally a node is
//! wrapped in a `Rc` pointer and adjacent edges in a `RefCell`. In the sync
//! versions these are `Arc` and `RwLock` respectively.
//!
//! - Nodes implement building blocks for algorithms in the form of breadth-first,
//! depth-firs and priority-first traversals as well as post- and preordering.
//!
//! - Macros for creating inline graphs in an easy-to-read style.
//!
//! - Graphs implement Serde's serialization and deserialization.
//!
//! - Removing or inserting connections or otherwise manipulating the graph
//! or any of its nodes is stable. Any references to nodes or edges remain
//! consistent. This is due to not relying on an underlying container where
//! nodes and edges would be represented as separate lists and indexed into,
//! in GDSL a node "owns" all it's incoming and outgoing connections.
//!
//! Motivation for creating this library has been to explore the idea of graphs and
//! connected nodes as more generic data-structures that store data without
//! depending on a central graph-container which in turn implements the graph-logic.
//!
//! Commented examples of algorithms implemented with GDSL can be found from the `examples` folder.
//!
//! # Examples
//!
//! ```
//! use gdsl::*;
//! use std::cell::Cell;
//!
//! // We create a directed graph using the `digraph![]` macro. In the macro
//! // invocation we specify the type of the nodes and the type of the edges
//! // by specifying the type-signature `(NodeKey, NodeValue) => [EdgeValue]`.
//! //
//! // The `NodeKey` type is used to identify the nodes in the graph. The
//! // `NodeValue` type is used to store the value of the node. The `EdgeValue`
//! // type is used to store the value of the edge.
//! //
//! // In this example the node stores the distance to the source node of the
//! // search. The edge stores the weight of the edge. The distance is wrapped
//! // in a `Cell` to allow for mutable access. We initialize the distance to
//! // `std::u64::MAX` to indicate that the node is not part of the shortest
//! // path.
//! let g = digraph![
//! 	(char, Cell<u64>) => [u64]
//! 	('A', Cell::new(u64::MAX)) => [ ('B', 4), ('H', 8) ]
//! 	('B', Cell::new(u64::MAX)) => [ ('A', 4), ('H', 11), ('C', 8) ]
//! 	('C', Cell::new(u64::MAX)) => [ ('B', 8), ('C', 2), ('F', 4), ('D', 7) ]
//! 	('D', Cell::new(u64::MAX)) => [ ('C', 7), ('F', 14), ('E', 9) ]
//! 	('E', Cell::new(u64::MAX)) => [ ('D', 9), ('F', 10) ]
//! 	('F', Cell::new(u64::MAX)) => [ ('G', 2), ('C', 4), ('D', 14), ('E', 10) ]
//! 	('G', Cell::new(u64::MAX)) => [ ('H', 1), ('I', 6), ('F', 2) ]
//! 	('H', Cell::new(u64::MAX)) => [ ('A', 8), ('B', 11), ('I', 7), ('G', 1) ]
//! 	('I', Cell::new(u64::MAX)) => [ ('H', 7), ('C', 2), ('G', 6) ]
//! ];
//!
//! // In order to find the shortest path we need to specify the source node and
//! // set its distance to 0.
//! g['A'].set(0);
//!
//! // In order to perform a dijkstra's we can use the priority first search or
//! // `pfs` for short. We determine a  source node create a `PFS` search-object
//! // by calling the `pfs()` method on the node.
//! //
//! // If we find a shorter distance to a node we are traversing, we need to
//! // update the distance of the node. We do this by using the `map()` method
//! // on the PFS search object. The `map()` method takes a closure as argument
//! // and calls it for each edge that is traversed. This way we can manipulate
//! // the distance of the node. based on the edge that is traversed.
//! //
//! // The search-object evaluates lazily. This means that the search is only
//! // executed when calling either `search()` or `search_path()`.
//! g['A'].pfs().map(&|u, v, e| {
//!
//! 	// Since we are using a `Cell` to store the distance we use `get()` to
//! 	// read the distance values.
//! 	let (u_dist, v_dist) = (u.get(), v.get());
//!
//! 	// Now we check if the distance stored in the node `v` is smaller than
//! 	// the distance stored in the node `u` + the length (weight) of the
//! 	// edge `e`. If this is the case we update the distance stored in the
//! 	// node `v`.
//! 	if v_dist > u_dist + e { v.set(u_dist + e); }
//! }).search();
//!
//! // We expect that the distance to the node `E` is 21.
//! assert!(g['E'].take() == 21);
//! ```

pub mod digraph;
pub mod ungraph;
pub mod sync_digraph;
pub mod sync_ungraph;
