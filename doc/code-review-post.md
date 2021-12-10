# Overview

I have been writing my own graph API for Rust and one thorny problem I wished to tackle was a
parallel breadth first traversal of the graph. The API itself will have different graph
flavours, but the one I have built for these tests is an undirected variant without double
edges.

I have created a breadth first traversal algorithm which takes in a user defined `explorer`
function determining how the graph should be traversed and what data processing should be
executed for each node and edge. I wish to showcase the algorithm and some use cases and
welcome input and ideas for further development and optimization.

# Usage

You can pull the repository from:

https://github.com/juliuskoskela/rust_graph

and run the benchmarks using cargo:

```
cargo bench
```

The example's folder contains some algorithm's built with the API such as maximum flow and
graph colouring.

# API

Below is a simple example of how the API can be used to fin the shortest path in an unweighted
graph:

```rust
// Create the graph. Generics are <node_key_type,
// node_data_type, edge_data_type>.
fn create_graph() -> Digraph<usize, Empty, Empty> {
	let mut g = Digraph::<usize, Empty, Empty>::new();

	g.insert(1, Empty);
	g.insert(2, Empty);
	g.insert(3, Empty);
	g.insert(4, Empty);
	g.insert(5, Empty);
	g.insert(6, Empty);

	g.connect(&1, &2, Empty);
	g.connect(&1, &3, Empty);
	g.connect(&2, &1, Empty);
	g.connect(&2, &3, Empty);
	g.connect(&3, &1, Empty);
	g.connect(&3, &5, Empty);
	g.connect(&5, &2, Empty);
	g.connect(&5, &4, Empty);
	g.connect(&5, &1, Empty);
	g.connect(&4, &5, Empty);
	g.connect(&4, &3, Empty);
	g.connect(&4, &2, Empty);
	g.connect(&4, &6, Empty);
	g
}

// Find shortes path without considering edge weights.
fn find_shortest_path() {
	let g = create_graph();

	// We conduct a breadth first traversal returning an enum
	// that actually has 3 states Include, Finish and Skip, but
	// we only use two. In each loop we have the data of the
	// edge's source node, target node and the edge itself to
	// prcess. Here we just check if we have found the sink
	// node 6.
	let res = g.breadth_first(&1,
	|edge|{
		if edge.target().key() == &6 {
			Traverse::Finish
		} else {
			Traverse::Include
		}
	}).unwrap();

	// As the traversal returns the whole traversed node tree
	// as an ordered list of edges, we take the last edge
	// containing the sink node and backtrack to the source
	// thus finding the shortest path.
	let path = backtrack_edges(&res);

	println!("Breadth First Search\n");
	for edge in path.iter() {
		println!("{}", edge.upgrade().unwrap());
	}
}
```

```
Edge "4" -> "6" : "OPEN" : "_"
Edge "5" -> "4" : "OPEN" : "_"
Edge "3" -> "5" : "OPEN" : "_"
Edge "1" -> "3" : "OPEN" : "_"
```

Graphs are very useful for modeling a variety of problems so
I wanted to make the underlying API as generic as possible.
Without going into too much detail about the API in general,
I have implemented a parallelized version of the breadth first
traversal utilizing the Rayon library and it's parallel iterators.
Before going to the algorithm here are some design decisions that
support this implementation:

-	Each node and edge have a lock associated with them. This
	lock is atomic and will block any thread from visiting a
	traversed edge or node. All locks are released after the
	traversal is completed.
-	Node's adjacency lists are behind an RwLock. This assures
	that we can read them in any thread without blocking,
	only write access blocks other threads.

# Parallel Breadth First Traversal

For the parallel bft we want to minimize thread blocking and thus
we want to minimize writing. We use a frontier strategy where we
get the first frontier from the source node's adjacent edges and
then process the rest, frontier by frontier, inside a parallelized
loop. Then we collect segments from each traversed node and combine
them into a new frontier. We use a terminate signal, which is an
AtomicBool wrapped in an Arc, to signal all thread's that the search
has been completed.

Here's the parallel bft method with annotations:

```rust
pub fn parallel_directed_breadth_traversal<K, N, E, F>(
    source: &ArcNode<K, N, E>,
    explorer: F,
) -> Option<Vec<WeakEdge<K, N, E>>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&ArcEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
	// Our results as well as our collected edges. We only
	// ever append to this structure.
    let mut frontiers: Vec<WeakEdge<K, N, E>>;

	// We keep track of the bounds of the current frontier so
	// we can slice it from the frontiers.
    let mut bounds: (usize, usize) = (0, 0);

	// Terminate signal shared by all threads.
    let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

	// Close the source node and initialize frontier. Exit if sink was
	// found ie. `filter_adjacent` returns Continue::No(segment). The
	// `filter_adjacent` function returns a vector containing all the
	// edges included by the `explorer` closure.
    source.close();
    match source.filter_adjacent(&explorer) {
        Continue::No(segment) => {

			// Important that we open the locks of the traversed elements
			// before returning. We don't want to open all the locks in the
			// whole graph which would be slow, only the ones we've closed.
            open_locks(&segment);
            return Some(segment);
        }
        Continue::Yes(segment) => {
            frontiers = segment;
        }
    }

	// Loop over frontiers.
    loop {

		// Update bounds an check if lower bound has reached upper bound
		// which means there are no more edges to traverse and we can terminate.
		// Otherwise slice the current frontier from frontiers.
        bounds.1 = frontiers.len();
        if bounds.0 == bounds.1 {
            break;
        }
        let current_frontier = &frontiers[bounds.0..bounds.1];
        bounds.0 = bounds.1;

		// This is the parallel loop. We carefully choose this part of the
		// algorithm to parallelize making sure there's no non-atomic
		// operations inside. A segment reprepsents the resulting edges
		// from one node. We collect those segments into a vector.
        let frontier_segments: Vec<_> = current_frontier
            .into_par_iter()
            .map(|edge| {
				match terminate.load(Ordering::Relaxed) {
					true => { None }
					false => {
						let node = edge.upgrade().unwrap().target();
                    	match node.filter_adjacent(&explorer) {
                    	    Continue::No(segment) => {
                    	        terminate.store(true, Ordering::Relaxed);
                    	        Some(segment)
                    	    }
                    	    Continue::Yes(segment) => Some(segment),
                    	}
					}
				}
            })
            .while_some()
            .collect();

		// We append the new segments into the frontiers even if we need to
		// terminate. Otherwise we couldn't open all the right locks later.
        for mut segment in frontier_segments {
            frontiers.append(&mut segment);
        }
        if terminate.load(Ordering::Relaxed) == true {
            break;
        }
    }
    open_locks(&frontiers);

	// If a terminating condition was met ie. Finish was returned from the `explorer`
	// closure we return a Some value, otherwise we just return None.
    if terminate.load(Ordering::Relaxed) == true {
        Some(frontiers)
    } else {
        None
    }
}
```

# Results

Here are some benchmarking tests I have conducted using the parallelized bft and
comparing it to a non-parallelized implementation which is otherwise equal, but
the looping of the current frontier is sequential.

These tests were conducted on a 4-core / 8-thread CPU AMD Ryzen 3100

Sample size: 100

Test graphs are generated randomly.

## Test Case 1: Naked Breadth First Search

In a naked search we just search for a sink and don't conduct any extra processing.

```
graph node count = 1000000
graph edge count = 9493671
graph average degree = 9.493671
sizeof graph = 499.74684 Mb

time:		[lower average upper]
sequential:	[140.12 ms 161.74 ms 183.62 ms]
parallel:	[61.172 ms 71.585 ms 82.589 ms]
```

The parallel algorithm is about 2x more efficient than the sequential.

## Test Case 2: Even Workload

In this test we sleep for 1 millisecond on each edge. This test will show the
performance difference on an constant workload.

```
graph node count = 100
graph edge count = 275
graph average degree = 2.75
sizeof graph = 0.023 Mb

time:		[lower average upper]
sequential:	[40.886 ms 45.184 ms 49.503 ms]
parallel:	[9.7412 ms 10.709 ms 11.667 ms]
```

Here we see a 4x efficiency for the parallel algorithm corresponding to the CPU
core count.

## Test Case 3: Uneven Workload

In this test case we check if a number hidden in the node is a prime or not.
This represents an uneven workload since a different number can take a different
amount of processing.

```
graph node count = 10000
graph edge count = 45079
graph average degree = 4.5079
sizeof graph = 3.00316 Mb

time:		[lower average upper]
sequential:	[39.385 ms 45.872 ms 52.380 ms]
parallel:	[0.082  ms 0.088  ms 0.095 ms]
```

Here we see a huge gain of around 500x for the parallel algorithm. It seems like a
lot, but I haven't been able to falsify it at least.

# Conclusion

I think it was an interesting problem to tackle and so far my solution seems to
be quite efficient. There are a variety of use cases for such data-structure and
the interesting parallelism it provides. Data and methods can be associated with
the edges and nodes and executed according to user logic during traversal.

The repository contains test, documentation and more examples.