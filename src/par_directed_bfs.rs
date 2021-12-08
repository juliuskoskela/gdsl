use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::global::*;
use crate::path::*;

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

// A helper function to open all closed nodes and edges after the algorithm has
// finished.
fn open_locks<K, N, E>(result: &EdgeList<K, N, E>)
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    for weak in result.iter() {
        let edge = weak.upgrade().unwrap();
        edge.open();
        edge.target().open();
        edge.source().open();
    }
}

fn filter_adjacent<K, N, E, F>(
    node: &RefNode<K, N, E>,
    user_closure: &F,
) -> Return<EdgeList<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn(&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
	let mut segment: EdgeList<K, N, E> = Vec::new();
	let adjacent_edges = node.outbound();
	for edge in adjacent_edges.iter() {
		if edge.try_lock() == OPEN && edge.target().try_lock() == OPEN {
			edge.target().close();
			edge.close();
			let traversal_state = user_closure(edge);
			match traversal_state {
				crate::global::Traverse::Include => {
					segment.push(Arc::downgrade(edge));
				}
				crate::global::Traverse::Finish => {
					segment.push(Arc::downgrade(edge));
					return Return::Yes(segment);
				}
				crate::global::Traverse::Skip => {
					edge.open();
					edge.target().open();
				}
			}
		}
	}
	Return::No(segment)
}

fn process_frontiers<K, N, E, F>(
    mut frontiers: EdgeList<K, N, E>,
    user_closure: &F,
) -> Option<EdgeList<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
    let mut bounds: (usize, usize) = (0, 0);
	let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    loop {
        // If any thread has found the sink we terminate the loop.
        if terminate.load(Ordering::Relaxed) == true {
            break;
        }

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

        // Iterator over Edges in the current frontier. If any thread has found the sink, we return
        // None which will terminate the loop.
        let iterator = current_frontier
            .into_par_iter()
            .map(|edge| {
                if terminate.load(Ordering::Relaxed) == true {
                    None
                } else {
                    let node = edge.upgrade().unwrap().target();
                    let haystack = filter_adjacent(&node, user_closure);
                    match haystack {
                        Return::Yes(segment) => {
                            terminate.store(true, Ordering::Relaxed);
                            Some(segment)
                        }
                        Return::No(segment) => Some(segment),
                    }
                }
            })
            .while_some();

        // Collect the segments from the iterator.
        let frontier_segments: Vec<_> = iterator.collect();

        // Append the segments to the frontiers.
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
    F: Fn(&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
    source.close();

    // Initialize frontier with source node
    let haystack = filter_adjacent(source, &user_closure);

    // In case target was found from the source node, terminate, otehrwise continue to process_frontiers
	match haystack {
		Return::Yes(current_frontier) => {
			open_locks(&current_frontier);
        	let result = Path::from(current_frontier);
        	Some(result)
		}
		Return::No(current_frontier) => {
			let network = process_frontiers(current_frontier, &user_closure);
       		match network {
       		    Some(paths) => {
       		        let result = Path::from(paths);
       		        Some(result)
       		    }
       		    None => None,
       		}
		}
	}
}
