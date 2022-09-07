# Graph Data Structure Library

[![crates][]](https://crates.io/crates/gdsl/) ![license][]

You can find the [API Documentation here](https://docs.rs/gdsl/latest/gdsl/)

GDSL is a graph data structure library providing efficient and easy-to-use
abstractions for working with connected nodes and graphs. Contrary to many other
graph implementations, in GDSL a graph is mostly just a container and the
functionality is in the `Node<K, N, E>` structure which can then be used to
create different graph representations or be used more freely as a part of
some other data structure.

- Directed and undirected graph and node types.

- Normal and sync versions of the node and graph types. Normally a node is
wrapped in a `Rc` pointer and adjacent edges in a `RefCell`. In the sync
versions these are `Arc` and `RwLock` respectively.

- Nodes implement building blocks for algorithms in the form of breadth-first,
depth-firs and priority-first traversals as well as post- and preordering.

- Macros for creating inline graphs in an easy-to-read style.

- Removing or inserting connections or otherwise manipulating the graph
or any of its nodes is stable. Any references to nodes or edges remain
consistent. This is due to not relying on an underlying container where
nodes and edges would be represented as separate lists and indexed into,
in GDSL a node "owns" all it's incoming and outgoing connections. On the other
hand, each node represents a heap allocation and store's its adjacent
edges inside a `RefCell` or an `RwLock`.

- Graphs implement Serde's serialization and deserialization.

Motivation for creating this library has been to explore the idea of graphs and
connected nodes as more generic data-structures that store data without
depending on a central graph-container which in turn implements the graph-logic.

Commented examples of algorithms implemented with GDSL can be found from the `examples` folder.

## Overview

Node types don't need to be part of a graph container. They are self-contained
"connected smart pointers" and can be connected to other nodes and dereferenced
using pointer syntax. Node uniqueness is determined by a generic key of type `K`.

```rust
let n1 = Node::<char, i32, f64>::new('A', 42);
let n2 = Node::<char, i32, f64>::new('B', 6);

n1.connect(&n2, 0.5);

assert!(*n1 == 42);
assert!(n2.key() == &'B');

// Get the next edge from the outbound iterator.
let (u, v, e) = n1.iter_out().next().unwrap();

assert!(u.key() == &'A');
assert!(v == n2);
assert!(e == 0.5);
```

Node contain both their inbound and outbound edges also in case of a directed graph.
This is so that the graph would be "dynamic" i.e. easily modifiable during run-time. If
we want to efficiently remove a node from a graph for example, it is more efficient if
we know the inbound connections of the node as well so that we can easily disconnect
a node from another. Iterators are implemented to iterate over either outbound or
inbound edges in case of a directed graph or adjacent edges in the case of an undirected
graph.

```rust
for (u, v, e) in &node {
    println!("{} -> {} : {}", u.key(), v.key(), e);
}

// Transposed iteration i.e. iterating the inbound edges of a node in digrap.
for (u, v, e) in node.iter_in() {
    println!("{} <- {} : {}", u.key(), v.key(), e);
}
```

As seen in the above example, an edge is represented as a tuple `(u, v, e)` in the
iterator and in other structures or as parameters `|u, v, e|` in closures. The
inner representation of the edge is not exposed to the user.

Nodes contain interfaces for searching and ordering nodes. These are implemented
as `search objects` which work a bit like iterators.

```rust
let path = node.dfs()
    .target(&'A')
    .search_path();

let ordering = node.order()
    .post()
    .filter(&|u, v, v| /* filter search */)
    .search_nodes();
```

The library provides macros for creating graphs such as `digraph![]` and `ungraph![]`
with a special syntax and with differing type signatures.

```rust
let g = digraph![
    (usize)
    (0) => [1, 2]
    (1) => [1]
    (2) => [0]
];
```

## Example: Simple Graph

A simple example on how to create a graph without a container or using any
macros for convenience. Most of the functionality actually resides in
the `Node`. The nodes contain methods for connecting (adding edges), disconnecting,
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

    // the distance stored in the node `u` + the length (weight) of the
    // Now we check if the distance stored in the node `v` is smaller than
    // edge `e`. If this is the case we update the distance stored in the
    // node `v`.
    if v_dist > u_dist + e {
        v.set(u_dist + e);
    }
}).search();

// We expect that the distance to the node `E` is 21.
assert!(g['E'].take() == 21);

```

## Similar Crates

- [Petgraph](https://docs.rs/petgraph/latest/petgraph/) is probably the most used graph library in Rust. Offers more graph representations, but all are tied to a container.

## Contact Developer

me@juliuskoskela.dev

[crates]: https://img.shields.io/crates/v/gdsl
[license]: https://img.shields.io/apm/l/vim-mode