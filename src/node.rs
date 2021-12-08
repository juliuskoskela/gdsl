use crate::adjacent::*;
use crate::edge::*;
use crate::path::*;
use crate::global::*;

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Weak, RwLockReadGuard, RwLockWriteGuard
    },
};

/// Node

#[derive(Debug)]
pub struct Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    key: K,
    data: Mutex<N>,
    outbound: RefAdjacent<K, N, E>,
    inbound: RefCell<Path<K, N, E>>,
    lock: Arc<AtomicBool>,
}

/// Node: Traits

unsafe impl<K, N, E> Sync for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
}

impl<K, N, E> Clone for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn clone(&self) -> Self {
        Node {
            key: self.key.clone(),
            data: Mutex::new(self.data.lock().unwrap().clone()),
            outbound: RefAdjacent::new(Adjacent::new()),
            inbound: RefCell::new(Path::new()),
            lock: Arc::new(AtomicBool::new(OPEN)),
        }
    }
}

impl<K, N, E> PartialEq for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn eq(&self, other: &Self) -> bool {
        if self.key == other.key {
            return true;
        }
        false
    }
}

impl<K, N, E> Display for Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Node {}", self.display_string())
    }
}

/// Node: Implementations

impl<K, N, E> Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    pub fn new(key: K, data: N) -> Node<K, N, E> {
        Self {
            key,
            data: Mutex::new(data),
            outbound: RefAdjacent::new(Adjacent::new()),
            inbound: RefCell::new(Path::new()),
            lock: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn load(&self) -> N {
        self.data.lock().unwrap().clone()
    }

    pub fn store(&self, data: N) {
        *self.data.lock().unwrap() = data;
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn try_lock(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    pub fn get_lock(&self) -> Weak<AtomicBool> {
        Arc::downgrade(&self.lock)
    }

    pub fn close(&self) {
        self.lock.store(CLOSED, Ordering::Relaxed)
    }

    pub fn open(&self) {
        self.lock.store(OPEN, Ordering::Relaxed)
    }

    pub fn outbound(&self) -> RwLockReadGuard<Adjacent<K, N, E>> {
        let lock = self.outbound.read();
		match lock {
			Ok(guard) => { guard }
			Err(error) => { panic!("RwLock error {}", error) }
		}
    }

    pub fn outbound_mut(&self) -> RwLockWriteGuard<Adjacent<K, N, E>> {
        let lock = self.outbound.write();
		match lock {
			Ok(guard) => { guard }
			Err(error) => { panic!("RwLock error {}", error) }
		}
    }

    pub fn inbound(&self) -> Ref<Path<K, N, E>> {
        self.inbound.borrow()
    }
    pub fn inbound_mut(&self) -> RefMut<Path<K, N, E>> {
        self.inbound.borrow_mut()
    }

	pub fn degree(&self) -> usize {
		self.outbound().len()
	}

	pub fn is_leaf(&self) -> bool {
		self.outbound().len() == 0
	}

    pub fn display_string(&self) -> String {
        let lock_state = if self.try_lock() { "CLOSED" } else { "OPEN" };
        let header = format!(
            "\"{}\" : \"{}\" : \"{}\"",
            self.key,
            lock_state,
            self.data.lock().unwrap()
        );
        header
    }
}

/// Node: Procedural Implementations

#[inline]
fn overlaps<K, N, E>(source: &RefNode<K, N, E>, target: &RefNode<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    source.outbound().find(source, target).is_some()
}

#[inline]
pub fn connect<K, N, E>(source: &RefNode<K, N, E>, target: &RefNode<K, N, E>, data: E) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    if !overlaps(source, target) {
        let new_edge = RefEdge::new(Edge::new(source, target, data));
        target.inbound_mut().add_weak(&Arc::downgrade(&new_edge));
        source.outbound_mut().add(new_edge);
        return true;
    }
    false
}

pub fn disconnect<K, N, E>(source: &RefNode<K, N, E>, target: &RefNode<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    let sr = source.outbound_mut().del(target);
    let tr = target.inbound_mut().del(source);
    sr && tr
}

fn depth_traversal_directed_recursion<K, N, E, F>(
    source: &RefNode<K, N, E>,
    results: &mut Path<K, N, E>,
    locks: &mut Vec<Weak<AtomicBool>>,
    f: F,
) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    source.close();
    locks.push(source.get_lock());
    for edge in source.outbound().iter() {
        if edge.try_lock() == OPEN && edge.target().try_lock() == OPEN {
            edge.target().close();
            edge.close();
            locks.push(edge.get_lock());
            let traverse = f(edge);
            match traverse {
                crate::node::Traverse::Include => {
                    results.add(&edge);
                    locks.push(edge.target().get_lock());
                }
				crate::node::Traverse::Finish => {
                    results.add(&edge);
                    locks.push(edge.target().get_lock());
                    return true;
                }
                crate::node::Traverse::Skip => {
					edge.target().open();
				}
            }
            return depth_traversal_directed_recursion(&edge.target(), results, locks, f);
        }
    }
    false
}

pub fn depth_traversal_directed<K, N, E, F>(
    source: &RefNode<K, N, E>,
    f: F,
) -> Option<Path<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    let mut result = Path::new();
    let mut locks = Vec::new();
    let res = depth_traversal_directed_recursion(source, &mut result, &mut locks, f);
    for weak in locks {
        let arc = weak.upgrade().unwrap();
        arc.store(OPEN, Ordering::Relaxed);
    }
    match res {
        true => Some(result),
        false => None,
    }
}

#[inline]
fn breadth_traversal_node<K, N, E, F>(
    source: &RefNode<K, N, E>,
    queue: &mut VecDeque<RefNode<K, N, E>>,
    result: &mut Path<K, N, E>,
    locks: &mut Vec<Weak<AtomicBool>>,
    f: &F,
) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    for edge in source.outbound().iter() {
        if edge.try_lock() == OPEN && edge.target().try_lock() == OPEN {
			edge.target().close();
            edge.close();
            locks.push(edge.get_lock());
            let traverse = f(edge);
            match traverse {
                crate::node::Traverse::Include => {
                    queue.push_back(edge.target());
                    result.add(&edge);
                    locks.push(edge.target().get_lock());
                }
				crate::node::Traverse::Finish => {
                    queue.push_back(edge.target());
                    result.add(&edge);
                    locks.push(edge.target().get_lock());
                    return true;
                }
                crate::node::Traverse::Skip => {
					edge.target().open();
				}
            }
        }
    }
    false
}

pub fn breadth_traversal_directed<K, N, E, F>(
    source: &RefNode<K, N, E>,
    f: F,
) -> Option<Path<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
	F: Fn (&RefEdge<K, N, E>) -> Traverse,
{
    let mut result = Path::new();
    let mut locks = Vec::new();
    let mut queue = VecDeque::new();
    source.close();
    locks.push(source.get_lock());
    if breadth_traversal_node(source, &mut queue, &mut result, &mut locks, &f) {
        for weak in locks {
            let lock = weak.upgrade().unwrap();
            lock.store(OPEN, Ordering::Relaxed);
        }
        return Some(result);
    }
    while let Some(node) = queue.pop_front() {
        if breadth_traversal_node(&node, &mut queue, &mut result, &mut locks, &f) {
            for weak in locks {
                let lock = weak.upgrade().unwrap();
                lock.store(OPEN, Ordering::Relaxed);
            }
            return Some(result);
        }
    }
    for weak in locks {
        let lock = weak.upgrade().unwrap();
        lock.store(OPEN, Ordering::Relaxed);
    }
    None
}

// fn open_locks<K, N, E>(result: &Frontier<K, N, E>)
// where
//     K: Hash + Eq + Clone + Debug + Display + Sync + Send,
//     N: Clone + Debug + Display + Sync + Send,
//     E: Clone + Debug + Display + Sync + Send,
// {
// 	for weak in result.iter() {
// 		let alive = weak.upgrade();
// 		match alive {
// 			Some(edge) => {
// 				edge.open();
// 				edge.target().open();
// 				edge.source().open();
// 			}
// 			None => { panic!("Weak reference not alive!") }
// 		}
// 	}
// }

// type Frontier<K, N, E> = Vec<WeakEdge<K, N, E>>;

// fn process_node<K, N, E, F>(
//     current_node: &RefNode<K, N, E>,
//     user_closure: F,
// 	terminate: &Arc<AtomicBool>
// ) -> Frontier<K, N, E>
// where
//     K: Hash + Eq + Clone + Debug + Display + Sync + Send,
//     N: Clone + Debug + Display + Sync + Send,
//     E: Clone + Debug + Display + Sync + Send,
// 	F: Fn (&RefEdge<K, N, E>) -> Traverse + Sync + Send,
// {
// 	let segment: Frontier<K, N, E>;
// 	let adjacent_edges = current_node.outbound();

// 	// We construct a parallel iterator from a filter map operation on
// 	// the node's adjacent edges.
// 	let parallel_iterator= adjacent_edges.par_iter().filter_map(
// 		|edge| {

// 			// Try if edge is traversable and check for finishing condition. The loop
// 			// in the thread won't finish, but the rest of the nodes won't be included in
// 			// the segment.
// 			if edge.try_lock() == OPEN
// 			&& edge.target().try_lock() == OPEN
// 			&& terminate.load(Ordering::Relaxed) == false {

// 				// Close target edge and node. This operation is atomic.
// 				edge.target().close();
// 				edge.close();

// 				// Get the traversal state by executing the user closure on the edge.
// 				// The User closure will return a Traverse enum with 3 states Include,
// 				// Finish and Skip.
// 				let traversal_state = user_closure(edge);
// 				match traversal_state {
// 					crate::node::Traverse::Include => {

// 						// In the include case we include the edge in the segment.
// 						Some(Arc::downgrade(edge))
// 					}
// 					crate::node::Traverse::Finish => {

// 						// In the finish case we include the edge in the segment and
// 						// set the finish boolean to true to signal we have found the
// 						// sink node and the whole algorithm can finish.
// 						terminate.store(true, Ordering::Relaxed);
// 						Some(Arc::downgrade(edge))
// 					}
// 					crate::node::Traverse::Skip => {

// 						// We skip the edge for a user defined reason.
// 						edge.open();
// 						edge.target().open();
// 						None
// 					}
// 				}

// 			// When an edge can't be traversed we return a none.
// 			} else {
// 				None
// 			}
// 		}
// 	);

// 	// We collect the parallel_iterator into a segment.
// 	segment = parallel_iterator.collect();

// 	return segment;
// }

// fn advance<K, N, E, F>(
// 	mut frontiers: Frontier<K, N, E>,
//     user_closure: F,
// 	terminate: &Arc<AtomicBool>
// ) -> Option<Frontier<K, N, E>>
// where
//     K: Hash + Eq + Clone + Debug + Display + Sync + Send,
//     N: Clone + Debug + Display + Sync + Send,
//     E: Clone + Debug + Display + Sync + Send,
// 	F: Fn (&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
// {
// 	let mut bounds: (usize, usize) = (0, 0);
// 	loop {

// 		// Base case
// 		if terminate.load(Ordering::Relaxed) == true{
// 			break ;
// 		}

// 		// We update the upper bound and check if the lower bound has caught the upper bound
// 		bounds.1 = frontiers.len();
// 		if bounds.0 >= bounds.1 {
// 			break ;
// 		}

// 		// A slice representing the current frontier
// 		let current_frontier = &frontiers[bounds.0..bounds.1];

// 		// Update the lower bound
// 		bounds.0 = bounds.1;

// 		// Iterator over Edges in the current frontier
// 		let iterator = current_frontier.into_par_iter().map(
// 			|edge| {
// 				if terminate.load(Ordering::Relaxed) == true {
// 					None
// 				} else {
// 					Some(process_node(&edge.upgrade().unwrap().target(), user_closure, &terminate))
// 				}
// 			}
// 		).while_some();

// 		// Collect the segments from the iterator
// 		let frontier_segments: Vec<_> = iterator.collect();

// 		// Append the segments to the frontiers
// 		for mut segment in frontier_segments {
// 			frontiers.append(&mut segment);
// 		}
// 	}

// 	// If target was found, returns the frontiers, else None
// 	open_locks(&frontiers);
// 	if terminate.load(Ordering::Relaxed) == true {
// 		Some(frontiers)
// 	} else {
// 		None
// 	}
// }

// pub fn parallel_directed_breadth_first_traversal<K, N, E, F>(
//     source: &RefNode<K, N, E>,
//     user_closure: F,
// ) -> Option<Path<K, N, E>>
// where
//     K: Hash + Eq + Clone + Debug + Display + Sync + Send,
//     N: Clone + Debug + Display + Sync + Send,
//     E: Clone + Debug + Display + Sync + Send,
// 	F: Fn (&RefEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
// {
// 	source.close();

// 	// A termination signal shared by all threads for if the target is found
// 	let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

// 	// Initialize frontier with source node
// 	let current_frontier = process_node(source, user_closure, &terminate);

// 	// In case target was found from the source node, terminate, otehrwise continue to advance
// 	if terminate.load(Ordering::Relaxed) == true {
// 		open_locks(&current_frontier);
// 		let result = Path::from(current_frontier);
// 		Some(result)
// 	} else {
// 		let network = advance(current_frontier, user_closure, &terminate);
// 		match network {
// 			Some(paths) => {
// 				let result = Path::from(paths);
// 				Some(result)
// 			}
// 			None => { None }
// 		}
// 	}
// }
