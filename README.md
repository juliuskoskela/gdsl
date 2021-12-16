# Fastgraph

A graph API offering a powerful graph trait that allows for the construction of
many types of graphs for different use-cases. Implements powerful parallel traversal
algorithms offering speedups when traversing the graph.

# Example Usage

```rust
use fastgraph::core::{Empty, Traverse};
use fastgraph::collections::*;

fn main() {
	let mut g = Digraph::<usize, Empty, Empty>::new();

	g.add_node(1, Empty);
	g.add_node(2, Empty);
	g.add_node(3, Empty);
	g.add_node(4, Empty);
	g.add_node(5, Empty);
	g.add_node(6, Empty);

	g.add_edge(1, 2, Empty);
	g.add_edge(1, 3, Empty);
	g.add_edge(2, 1, Empty);
	g.add_edge(2, 3, Empty);
	g.add_edge(3, 1, Empty);
	g.add_edge(3, 5, Empty);
	g.add_edge(5, 2, Empty);
	g.add_edge(5, 4, Empty);
	g.add_edge(5, 1, Empty);
	g.add_edge(4, 5, Empty);
	g.add_edge(4, 3, Empty);
	g.add_edge(4, 2, Empty);
	g.add_edge(4, 6, Empty);

	let sink = g.get_node(6).unwrap();
	let shortest_tree = g.par_breadth_first(1,
		|edge|{
			if edge.target() == sink {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		}).unwrap();

	let shortest_path = fastgraph::core::backtrack_edges(&shortest_tree);

	for edge in shortest_path {
		println!("{}", edge.upgrade().unwrap())
	}
}

```

```
1 -> 3
3 -> 5
5 -> 4
4 -> 6
```

# Implementation

Fastgraph is implemented ion a way as to allow for fast and concurrent
processing of a graph. The underlying representation is basically an adjacency
list, but both edges and nodes are implemented as their own structures and may
contain arbitary data available atomically to each thread during traversal.

## Types

Fastgraph has `Edge<K, N, E>` and `Node<K, N, E>`. These types work without any
container type and implement method's for basic operations and traversal.

The `Graph` trait can be used to implement the graph on top of any kind of
container. The crate provides generic `Digraph` and `Ungraph` types, but other
types with different types of containers can be implemented easily just by
implementing how nodes are accessed through the chosen container.

## Traversal

When we traverse a graph we need to keep track of `visited` nodes. This is often
done with some sort of a lookup such as a hash map or binary tree which we use
to check if a node has been visited or not. This is a very inefficient method
and when used in a multithreaded scenarion, would incur blocking when wiritng to
the datastructure from one thread.

In fastgraph each node itelf contains an atomic lock which we can toggle safely
from any thread and thus block the node from traversal. When the traversal is
finished, we open the locks again.

When we conduct a traversal we do create an additional data-structure, namely a
shortest path tree. This tree is then returned from the `breadth_first` and
`depth_first` (and their parallel counterparts). If we were looking for a
shortest path to a spesific node, that node will be the target node of the last
edge in the list and we can collect the shortest path by backtracking using the
shortest path tree.

# Performance

This repository contains benchmarks comparing the speed of sequential vs.
parallel iteration. Thiose may be run using the cargo `bench command`. In
detailed tests we observe a very nice performance gain in parallel iteration
over sequantial iteration as long as the graph's size is big enough to justify
the overhead. Under the hood fastgraph uses the `rayon` library to parallelize
traversal loops.

!Continue
