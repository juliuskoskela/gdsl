use crate::global::*;
use crate::traverse::Traverse;
use crate::path::*;

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

type Frontier<K, N, E> = Vec<WeakEdge<K, N, E>>;

fn open_locks<K, N, E>(result: &Frontier<K, N, E>)
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

#[inline(always)]
fn process_node<K, N, E, F>(
    current_node: &RefNode<K, N, E>,
    user_closure: F,
    terminate: &Arc<AtomicBool>,
) -> Frontier<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&RefEdge<K, N, E>) -> Traverse + Sync + Send,
{
    let segment: Frontier<K, N, E>;
    let adjacent_edges = current_node.outbound();
    let iterator = adjacent_edges.iter().filter_map(|edge| {
        if edge.try_lock() == OPEN
            && edge.target().try_lock() == OPEN
            && terminate.load(Ordering::Relaxed) == false
        {
            edge.target().close();
            edge.close();
            let traversal_state = user_closure(edge);
            match traversal_state {
                Traverse::Include => Some(Arc::downgrade(edge)),
                Traverse::Finish => {
                    terminate.store(true, Ordering::Relaxed);
                    Some(Arc::downgrade(edge))
                }
                Traverse::Skip => {
                    edge.open();
                    edge.target().open();
                    None
                }
            }
        } else {
            None
        }
    });
    segment = iterator.collect();
    return segment;
}

#[inline(never)]
fn process_frontiers<K, N, E, F>(
    mut frontiers: Frontier<K, N, E>,
    user_closure: F,
    terminate: &Arc<AtomicBool>,
) -> Option<Frontier<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
    let mut bounds: (usize, usize) = (0, 0);
    loop {
        if terminate.load(Ordering::Relaxed) == true {
            break;
        }
        bounds.1 = frontiers.len();
        if bounds.0 >= bounds.1 {
            break;
        }
        let current_frontier = &frontiers[bounds.0..bounds.1];
        bounds.0 = bounds.1;
        let mut frontier_segments = Vec::new();
        for edge in current_frontier {
            if terminate.load(Ordering::Relaxed) == true {
                break;
            } else {
                frontier_segments.push(process_node(
                    &edge.upgrade().unwrap().target(),
                    user_closure,
                    &terminate,
                ));
            }
        }
        for mut segment in frontier_segments {
            frontiers.append(&mut segment);
        }
    }
    open_locks(&frontiers);
    if terminate.load(Ordering::Relaxed) == true {
        Some(frontiers)
    } else {
        None
    }
}

#[inline(never)]
pub fn _directed_breadth_first_traversal<K, N, E, F>(
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
    let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let current_frontier = process_node(source, user_closure, &terminate);
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
            None => None,
        }
    }
}
