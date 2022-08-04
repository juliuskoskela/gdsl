# Graph Data Structure Library

GDSL is a graph data structure library providing efficient and easy-to-use
abstractions for working with either directed or undirected graphs. The aim of
this library is not to implement specific graph algorithms, but rather work as
a building block for graphs and/or connected nodes for more specific use-cases.

- Node types don't need to be part of any graph container. They are self-contained
"smart pointers".

- Node contain both their inbound and outbound edges also in case of a directed graph.
This is so that the graph would be "dynamic" ie. easily modifiable during run-time. If
we want to efficiently remove a node from a graph for example, it is more efficient if
we know the inbound connections of the node as well so that we can easily disconnect
a node from another.

- The library provides a `graph![]` macro that can be used to create both
directed and undirected graphs by writing out an edge-list representation of the graph.

**This library is still in early development and the API might experience breaking
changes.**

## Example: Simple Graph

A simple example on how to create a graph without a container or using any
macros for convenience. Most of the functionality which actually resides in
the `Node`. The nodes contains methods for connecting (adding edges), disconnecting,
searching etc. They also behave as "smart pointers" and implement `Deref` so that they
can be dereferenced into their inner values.

```rust

use gdsl::digraph::node::Node;
use gdsl::*;

fn main() {

	// We create directed nodes with characters as keys and integers as values.
	// The turbofish type-signature is included in the first line for clarity,
	// but the types could be completely inferred. Note that in order to infer
	// the type for the edge, `connect()` or `connect!()` must be used.

	let node_a = Node::<char, i32, ()>::new('A', 1);
	let node_b = Node::new('B', 2);
	let node_c = Node::new('C', 3);

	// We connect nodes a -> b and b -> c. The () empty tuple is used to denote
	// that the edge has no value associated with it.

	node_a.connect(&node_b, ());
	node_b.connect(&node_c, ());

	// Check that a -> b && b -> c && !(a -> c)

	assert!(node_a.is_connected(&node_b));
	assert!(node_b.is_connected(&node_c));
	assert!(!node_a.is_connected(&node_c));
}

```

## Example: Using the Macros

Different ways of creating a graph with varying type-signatures.

```rust

use gdsl::*;

fn main() {

	// <&str, _, _>
	let g1 = digraph![
		(&str)
		("A") => ["B", "C"]
		("B") => ["C"]
		("C") => ["D"]
		("D") => []
	];

	// <&str, i32, _>
	let g2 = digraph![
		(&str, i32)
		("A", 42) => ["B", "C"]
		("B", 42) => ["C"]
		("C", 42) => ["D"]
		("D", 42) => []
	];

	// <&str, _, i32>
	let g3 = digraph![
		(&str) => [i32]
		("A") => [("B", 42), ("C", 42)]
		("B") => [("C", 42)]
		("C") => [("D", 42)]
		("D") => []
	];

	// <&str, i32, f64>
	let g4 = digraph![
		(&str, i32) => [f64]
		("A", 42) => [("B", 3.14), ("C", 3.14), ("D", 3.14)]
		("B", 42) => [("C", 3.14), ("D", 3.14)]
		("C", 42) => [("D", 3.14)]
		("D", 42) => []
	];
}

```

## Example: Dijkstra's Shortest Path

Below is a commented example of an implementation of Dijkstra's shortest path
algorithm using abstractions from the graph library.

```rust

use gdsl::*;
use std::cell::Cell;

fn main() {

	// We create a directed graph using the `digraph![]` macro. In the macro
	// invocation we specify the type of the nodes and the type of the edges
	// by specifying the type-signature `(NodeKey, NodeValue) => [EdgeValue]`.
	//
	// The `NodeKey` type is used to identify the nodes in the graph. The
	// `NodeValue` type is used to store the value of the node. The `EdgeValue`
	// type is used to store the value of the edge.
	//
	// In this example the node stores the distance to the source node of the
	// search. The edge stores the weight of the edge. The distance is wrapped
	// in a `Cell` to allow for mutable access. We initialize the distance to
	// `std::u64::MAX` to indicate that the node is not part of the shortest
	// path.
	let g = digraph![
		(char, Cell<u64>) => [u64]
		('A', Cell::new(u64::MAX)) => [ ('B', 4), ('H', 8) ]
		('B', Cell::new(u64::MAX)) => [ ('A', 4), ('H', 11), ('C', 8) ]
		('C', Cell::new(u64::MAX)) => [ ('B', 8), ('C', 2), ('F', 4), ('D', 7) ]
		('D', Cell::new(u64::MAX)) => [ ('C', 7), ('F', 14), ('E', 9) ]
		('E', Cell::new(u64::MAX)) => [ ('D', 9), ('F', 10) ]
		('F', Cell::new(u64::MAX)) => [ ('G', 2), ('C', 4), ('D', 14), ('E', 10) ]
		('G', Cell::new(u64::MAX)) => [ ('H', 1), ('I', 6), ('F', 2) ]
		('H', Cell::new(u64::MAX)) => [ ('A', 8), ('B', 11), ('I', 7), ('G', 1) ]
		('I', Cell::new(u64::MAX)) => [ ('H', 7), ('C', 2), ('G', 6) ]
	];

	// In order to find the shortest path we need to specify the source node and
	// set its distance to 0.
	g['A'].set(0);

	// In order to perform a dijkstra's we can use the priority first search or
	// `pfs` for short. We determine a  source node create a `PFS` search-object
	// by calling the `pfs()` method on the node.
	//
	// If we find a shorter distance to a node we are traversing, we need to
	// update the distance of the node. We do this by using the `map()` method
	// on the PFS search object. The `map()` method takes a closure as argument
	// and calls it for each edge that is traversed. This way we can manipulate
	// the distance of the node. based on the edge that is traversed.
	//
	// The search-object evaluates lazily. This means that the search is only
	// executed when calling either `search()` or `search_path()`.
	g['A'].pfs().map(&|u, v, e| {

		// Since we are using a `Cell` to store the distance we use `get()` to
		// read the distance values.
		let (u_dist, v_dist) = (u.get(), v.get());

		// Now we check if the distance stored in the node `v` is smaller than
		// the distance stored in the node `u` + the length (weight) of the
		// edge `e`. If this is the case we update the distance stored in the
		// node `v`.
		if v_dist > u_dist + e { v.set(u_dist + e); }
	}).search();

	// We expect that the distance to the node `E` is 21.
	assert!(g['E'].take() == 21);
}

```
