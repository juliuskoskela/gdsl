use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;

use crate::path::*;
use crate::global::*;

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc
    },
};

type Frontier<K, N, E> = Vec<WeakEdge<K, N, E>>;

// A helper function to open all closed nodes and edges after the algorithm has
// finished.
fn open_locks<K, N, E>(result: &Frontier<K, N, E>)
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	for weak in result.iter() {
		let alive = weak.upgrade();
		match alive {
			Some(edge) => {
				edge.open();
				edge.target().open();
				edge.source().open();
			}
			None => { panic!("Weak reference not alive!") }
		}
	}
}

fn process_node<K, N, E, F>(
    current_node: &RefNode<K, N, E>,
    user_closure: F,
	terminate: &Arc<AtomicBool>
) -> Frontier<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse + Sync + Send,
{
	let segment: Frontier<K, N, E>;
	let adjacent_edges = current_node.outbound();

	// We construct a parallel iterator from a filter map operation on
	// the node's adjacent edges.
	let parallel_iterator= adjacent_edges.par_iter().filter_map(

		// Closure for the filtering operation.
		|edge| {

			// Try if edge is traversable and check for finishing condition. The loop
			// in the thread won't finish, but the rest of the nodes won't be included in
			// the segment.
			if edge.try_lock() == OPEN
				&& edge.target().try_lock() == OPEN
				&& terminate.load(Ordering::Relaxed) == false {

				// Close target edge and node. This operation is atomic.
				edge.target().close();
				edge.close();

				// Get the traversal state by executing the user closure on the edge.
				// The User closure will return a Traverse enum with 3 states Include,
				// Finish and Skip.
				let traversal_state = user_closure(edge);
				match traversal_state {
					crate::global::Traverse::Include => {

						// In the include case we include the edge in the segment.
						Some(Arc::downgrade(edge))
					}
					crate::global::Traverse::Finish => {

						// In the finish case we include the edge in the segment and
						// set the finish boolean to true to signal we have found the
						// sink node and the whole algorithm can finish.
						terminate.store(true, Ordering::Relaxed);
						Some(Arc::downgrade(edge))
					}
					crate::global::Traverse::Skip => {

						// We skip the edge for a user defined reason.
						edge.open();
						edge.target().open();
						None
					}
				}

			// When an edge can't be traversed we return a none.
			} else {
				None
			}
		}
	);

	// We collect the parallel_iterator into a segment and return it.
	segment = parallel_iterator.collect();
	return segment;
}

fn process_frontiers<K, N, E, F>(
	mut frontiers: Frontier<K, N, E>,
    user_closure: F,
	terminate: &Arc<AtomicBool>
) -> Option<Frontier<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
	let mut bounds: (usize, usize) = (0, 0);
	loop {

		// If any thread has found the sink we terminate the loop.
		if terminate.load(Ordering::Relaxed) == true {
			break ;
		}

		// We update the upper bound and check if the lower bound has caught the upper bound.
		bounds.1 = frontiers.len();
		if bounds.0 >= bounds.1 {
			break ;
		}

		// A slice representing the current frontier
		let current_frontier = &frontiers[bounds.0..bounds.1];

		// Update the lower bound
		bounds.0 = bounds.1;

		// Iterator over Edges in the current frontier. If any thread has found the sink, we return
		// None which will terminate the loop.
		let iterator = current_frontier.into_par_iter().map(
			|edge| {
				if terminate.load(Ordering::Relaxed) == true {
					None
				} else {
					Some(process_node(&edge.upgrade().unwrap().target(), user_closure, &terminate))
				}
			}
		).while_some();

		// Collect the segments from the iterator
		let frontier_segments: Vec<_> = iterator.collect();

		// Append the segments to the frontiers
		for mut segment in frontier_segments {
			frontiers.append(&mut segment);
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

pub fn parallel_directed_breadth_first_traversal<K, N, E, F>(
    source: &RefNode<K, N, E>,
    user_closure: F,
) -> Option<Path<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
	source.close();

	// A termination signal shared by all threads for if the target is found
	let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

	// Initialize frontier with source node
	let current_frontier = process_node(source, user_closure, &terminate);

	// In case target was found from the source node, terminate, otehrwise continue to process_frontiers
	if terminate.load(Ordering::Relaxed) == true {
		open_locks(&current_frontier);
		let result = Path::from(current_frontier);
		Some(result)
	} else {
		let network = process_frontiers(current_frontier, user_closure, &terminate);
		match network {
			Some(paths) => {
				let result = Path::from(paths);
				Some(result)
			}
			None => { None }
		}
	}
}
