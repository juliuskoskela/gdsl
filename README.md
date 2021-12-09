# Graph API

## API Structure

Node

Edge
--- Path

Graph
--- Digraph
--- Ungraph

```rust

pub fn parallel_directed_breadth_first_traversal<K, N, E, F>(
    source: &RefNode<K, N, E>,
    explorer: F,
) -> Option<EdgeList<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
    let mut frontiers: EdgeList<K, N, E>;
	let mut bounds: (usize, usize) = (0, 0);
	let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    // Close the source node and use it to initialize the first frontier.
	source.close();
	let initial = filter_adjacent(source, &explorer);
	match initial {
		Continue::No(segment) => {
			return Some(segment);
		},
		Continue::Yes(segment) => {
			frontiers = segment;
		}
	}

	// Loop over incoming frontiers until either all available nodes have been
	// traversed or the sink has been found.
    loop {

        // We update the upper bound and break if the lower bound has caught the upper bound
        // which would signify we have traversed all available nodes.
        bounds.1 = frontiers.len();
        if bounds.0 == bounds.1 {
            break;
        }

        // We slice the current frontier from the frontiers using the bounds.
        let current_frontier = &frontiers[bounds.0..bounds.1];

        // Update the lower bound.
        bounds.0 = bounds.1;

        // A parallel iterator over the edges in the current frontier.
        let frontier_segments: Vec<_> = current_frontier
            .into_par_iter()
            .map(|edge| {

				// If any thread reaches the sink and toggles the terminate to true,
				// any thread should stop so we return None to terminate the parallel loop.
                if terminate.load(Ordering::Relaxed) == true {
                    None
                } else {
                    let node = edge.upgrade().unwrap().target();

					// We filter the adjacent edges from the target node that can be traversed.
                    let haystack = filter_adjacent(&node, &explorer);
                    match haystack {

						// If the segment contains the sink we terminarte, otherwise we continue collecting segments.
                        Continue::No(segment) => {
                            terminate.store(true, Ordering::Relaxed);
                            Some(segment)
                        }
                        Continue::Yes(segment) => Some(segment),
                    }
                }
            })
            .while_some()
			.collect();

        // Append the segments to the frontiers.
        for mut segment in frontier_segments {
            frontiers.append(&mut segment);
        }

		// If any thread has found the sink we terminate the loop.
        if terminate.load(Ordering::Relaxed) == true {
            break;
        }
    }

    // If target was found, returns the frontiers, else None
    open_locks(&frontiers);
    if terminate.load(Ordering::Relaxed) == true {
        Some(frontiers)
    } else {
        None
    }
}

```