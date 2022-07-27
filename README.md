# Graph Data Structure Library

GDSL is a graph data structure library providing efficient and easy-to-use
abstractions for working with either directed or undirected graphs. The aim of
this library is not to implement specific graph algorithms, but rather work as
a building block for graphs and/or connected nodes for more specific use-cases.

## Example: Simple Graph

A simple example on how to create a graph without a container or using any
macros for convenience. The `DiGraph` and `UnGraph` types are just containers,
and they are not needed for most of the functionality which actually resides in
the `DiNode` and `UnNode` abstractions. The nodes contain methods for
connecting (adding edges), disconnecting, searching etc. They also behave as
"smart pointers" and implement `Deref` so that they can be dereferenced into
their inner values.

```rust

fn main() {
	use gdsl::digraph::node::DiNode;
	use gdsl::*;

	// We create directed nodes with characters as keys and integers as values.
	// The turbofish type-signature is included in the first line for clarity,
	// but the types could be completely inferred. Note that in order to infer
	// the type for the edge, `connect()` or `connect!()` must be used.
	let node_a = DiNode::<char, i32, Empty>::new('A', 1);
	let node_b = DiNode::new('B', 2);
	let node_c = DiNode::new('C', 3);

	// We connect nodes a -> b and b -> c. The Empty struct is used to denote
	// that the edge has no value associated with it.
	node_a.connect(&node_b, Empty);
	node_b.connect(&node_c, Empty);

	// Check that a -> b && b -> c && !(a -> c)
	assert!(node_a.is_connected(&node_b));
	assert!(node_b.is_connected(&node_c));
	assert!(!node_a.is_connected(&node_c));
}

```

## Example: Using the Macros

TODO!

## Example: Dijkstra's Shortest Path

Below is a commented example of an implementation of Dijkstra's shortest path
algorithm using abstractions from the graph library.

```rust

fn main() {
	use gdsl::*;
	use std::cell::Cell;

	// We create a directed graph using the `graph!` macro. In the macro
	// invocation we specify the type of the nodes and the type of the edges
	// by specifying the type-signature `(NodeKey, NodeValue) => [EdgeValue]`.
	//
	// The `NodeKey` type is used to identify the nodes in the graph. The
	// `NodeValue` type is used to store the value of the node. The `EdgeValue`
	// type is used to store the value of the edge.
	//
	// The macro also specifies if the graph is directed or undirected. In this
	// case it is directed. If we want to create an undirected graph we have to
	// use the `:` operator instead of the `=>` operator. The macro returns
	// either a `DiGraph` or `UnGraph` type respectively.
	//
	// In this example the node stores the distance to the source node of the
	// search. The edge stores the weight of the edge. The distance is wrapped
	// in a `Cell` to allow for mutable access. We initialize the distance to
	// `std::u64::MAX` to indicate that the node is not part of the shortest
	// path.

	let g = graph![
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

	// In order to perform a dijkstra's we take the source node and call the
	// `pfs_min()` function which returns a search object. A search object is
	// like an iterator. From the search object we call the `search_map()`
	// function which let's us read each edge in the search and to manipulate
	// the corresponding nodes.
	//
	// The `pfs_min()` function is a "priority first search". As opposed to a
	// breadth-first search or a depth first search, the priority first
	// search traverses the nodes in the graph in a priority order. The priority
	// of a node is determined by the node's value and thus has to implement
	// the `Ord` trait. Since `u64` implements the `Ord` trait we can use the
	// distance stored in the node as the priority.
	//
	// The `search_map()` function takes a `target` node and a closure which
	// is called for each edge in the search. The target is optional, in case
	// we want to search the whole graph. In this case the target is `None`,
	// so we will calculate the distance to all nodes.

	g['A'].pfs_min().search_map(None, &|u, v, e| {

		// Now we check if the distance stored in the node `v` is smaller than
		// the distance stored in the node `u` + the length (weight) of the
		// edge `e`. If this is the case we update the distance stored in the
		// node `v`.

		if v.get() > u.get() + e {
			v.set(u.get() + e);
		}
	});

	// We expect that the distance to the node `E` is 21.
	assert!(g['E'].take() == 21);
}

```
