# Fastgraph

A graph API offering a powerful graph trait that allows for the construction of
many types of graphs for different use-cases. Implements powerful parallel traversal
algorithms offering speedups when traversing the graph with the `breadth_first` and
`depth_first` methods.

# Example Usage

```rust

fn test() {
	let mut g = Digraph::<usize, Empty, Empty>::new();

	g.add_node(1, Empty);
	g.add_node(2, Empty);
	g.add_node(3, Empty);
	g.add_node(4, Empty);
	g.add_node(5, Empty);
	g.add_node(6, Empty);

	g.add_edge(&1, &2, Empty);
	g.add_edge(&1, &3, Empty);
	g.add_edge(&2, &1, Empty);
	g.add_edge(&2, &3, Empty);
	g.add_edge(&3, &1, Empty);
	g.add_edge(&3, &5, Empty);
	g.add_edge(&5, &2, Empty);
	g.add_edge(&5, &4, Empty);
	g.add_edge(&5, &1, Empty);
	g.add_edge(&4, &5, Empty);
	g.add_edge(&4, &3, Empty);
	g.add_edge(&4, &2, Empty);
	g.add_edge(&4, &6, Empty);

	let sink = g.get_node(&6).unwrap();
	let res1 = g.par_breadth_first(&1,
		|edge|{
			if edge.target() == sink {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		}).unwrap();

	let shortest_path = backtrack_edges(&res1);

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