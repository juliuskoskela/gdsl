use crate::global::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub enum Traverse {
	Include,
	Skip,
	Finish,
}

fn filter_adjacent<K, N, E, F>(
    node: &RefNode<K, N, E>,
    user_closure: &F,
) -> Continue<EdgeList<K, N, E>>
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
                Traverse::Include => {
                    segment.push(Arc::downgrade(edge));
                }
                Traverse::Finish => {
                    segment.push(Arc::downgrade(edge));
                    return Continue::No(segment);
                }
                Traverse::Skip => {
                    edge.open();
                    edge.target().open();
                }
            }
        }
    }
    Continue::Yes(segment)
}

/// # Breadth First Traversal
///
/// Conduct a breadth first traversal starting from the source node.
/// User provides an `explorer` closure which determines how nodes and edges
/// are to be interpreted. The closure will return a Traverse enum which has
/// 3 states Include, Skip and Finish. These states determine if we are to
/// "go through" the edge and thus include it in our search. Include will
/// include the edge and continue the search, Skip will indicate that the edge
/// is not to be traversed and Finish will include the edge and finish the
/// algorithm.
///
/// Function will return an `Option<EdgeList<K, N, E>>` where a Some value
/// indicates that the traversal was successful ie. a Finish condition was
/// reached. And EdgeList is a collection of all the traversed edges.
/// The last edge will contain the result that triggered the Finish condition.
/// To get the shortest path for example, we'd backtrack the EdgeList starting
/// from the last edge which would contain our sink node.
///
/// # Examples
///
/// ```
/// use graph::node::*;
/// use graph::global::*;
/// use graph::global::Traverse::*;
/// use graph::bfs::*;
///
/// let n1 = RefNode::new(Node::<u32, Empty, Empty>::new(1, Empty));
/// let n2 = RefNode::new(Node::<u32, Empty, Empty>::new(2, Empty));
/// let n3 = RefNode::new(Node::<u32, Empty, Empty>::new(3, Empty));
///
/// connect(&n1, &n2, Empty);
/// connect(&n2, &n3, Empty);
/// connect(&n1, &n3, Empty);
///
/// let edges = directed_breadth_first_traversal(&n1,
/// 	| edge | {
/// 		if n3 == edge.target() {
///				Finish
///			} else {
///				Include
///			}
/// 	})
/// 	.unwrap();
///
/// let shortest_path = backtrack_edges(&edges);
///
/// assert!(shortest_path.len() == 1);
/// ```

pub fn directed_breadth_first_traversal<K, N, E, F>(
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
    source.close();
    let initial = filter_adjacent(source, &explorer);
    match initial {
        Continue::No(segment) => {
            open_locks(&segment);
            return Some(segment);
        }
        Continue::Yes(segment) => {
            frontiers = segment;
        }
    }
    loop {
        bounds.1 = frontiers.len();
        if bounds.0 == bounds.1 {
            break;
        }
        let current_frontier = &frontiers[bounds.0..bounds.1];
        bounds.0 = bounds.1;
        let mut new_segments = Vec::new();
        for edge in current_frontier.into_iter() {
            let node = edge.upgrade().unwrap().target();
            let haystack = filter_adjacent(&node, &explorer);
            match haystack {
                Continue::No(mut segment) => {
                    new_segments.append(&mut segment);
                    frontiers.append(&mut new_segments);
                    open_locks(&frontiers);
                    return Some(frontiers);
                }
                Continue::Yes(mut segment) => {
                    new_segments.append(&mut segment);
                }
            }
        }
        frontiers.append(&mut new_segments);
    }
    open_locks(&frontiers);
    None
}

/// # Parallel Breadth First Traversal
///
/// Conduct a parallel breadth first traversal starting from the source node.
/// User provides an `explorer` closure which determines how nodes and edges
/// are to be interpreted. The closure will return a Traverse enum which has
/// 3 states Include, Skip and Finish. These states determine if we are to
/// "go through" the edge and thus include it in our search. Include will
/// include the edge and continue the search, Skip will indicate that the edge
/// is not to be traversed and Finish will include the edge and finish the
/// algorithm.
///
/// Function will return an `Option<EdgeList<K, N, E>>` where a Some value
/// indicates that the traversal was successful ie. a Finish condition was
/// reached. And EdgeList is a collection of all the traversed edges.
/// The last edge will contain the result that triggered the Finish condition.
/// To get the shortest path for example, we'd backtrack the EdgeList starting
/// from the last edge which would contain our sink node.

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
    source.close();
    let initial = filter_adjacent(source, &explorer);
    match initial {
        Continue::No(segment) => {
            open_locks(&segment);
            return Some(segment);
        }
        Continue::Yes(segment) => {
            frontiers = segment;
        }
    }
    loop {
        bounds.1 = frontiers.len();
        if bounds.0 == bounds.1 {
            break;
        }
        let current_frontier = &frontiers[bounds.0..bounds.1];
        bounds.0 = bounds.1;
        let frontier_segments: Vec<_> = current_frontier
            .into_par_iter()
            .map(|edge| {
                if terminate.load(Ordering::Relaxed) == true {
                    None
                } else {
                    let node = edge.upgrade().unwrap().target();
                    let haystack = filter_adjacent(&node, &explorer);
                    match haystack {
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
        for mut segment in frontier_segments {
            frontiers.append(&mut segment);
        }
        if terminate.load(Ordering::Relaxed) == true {
            break;
        }
    }
    open_locks(&frontiers);
    if terminate.load(Ordering::Relaxed) == true {
        Some(frontiers)
    } else {
        None
    }
}
