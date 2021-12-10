//=============================================================================
// CORE ABSTRACTIONS
//=============================================================================

//! # Graph Library Core
//!
//! This is the core part of the graph library. It contains a node and an
//! edge abstraction as well as traveral algorithms. Used to build
//! different graphs.
//!
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};
use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak,
    },
};

//=============================================================================

/// Constant that represent if a node or an edge is open.
///
pub const OPEN: bool = false;

/// Constant that represent if a node or an edge is closed.
///
pub const CLOSED: bool = true;

/// The Traverse enum is used when exploring edges in a graph. It's
/// states signify if an edge should be included in the search, skipped
/// or if the search should stop because a terminating condition has
/// been met such as finding a sink node.
///
pub enum Traverse {
    Include,
    Skip,
    Finish,
}

/// The Continue enum signifies if a loop should be stopped inclusive
/// of the last item or if the loop should be continued.
///
pub enum Continue<T> {
    Yes(T),
    No(T),
}

/// Represents an empty parameter for either a node or an edge.
///
#[derive(Clone, Debug)]
pub struct Empty;

impl std::fmt::Display for Empty {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "_")
    }
}

//=============================================================================
// EDGE IMPLEMENTATION
//=============================================================================

/// Edges are handled through a atomically reference counted
/// smart pointer.
///
pub type ArcEdge<K, N, E> = Arc<Edge<K, N, E>>;

/// Sometimes we need to handle edges through a weak reference.
///
pub type WeakEdge<K, N, E> = Weak<Edge<K, N, E>>;

//=============================================================================

/// Edge representing a connection between two nodes. Relevant data can be
/// stored in the edge atomically. Edge's target and source node's are
/// weak references and can't outlive the nodes they represent.
///
#[derive(Debug)]
pub struct Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    source: WeakNode<K, N, E>,
    target: WeakNode<K, N, E>,
    data: Mutex<E>,
    lock: Arc<AtomicBool>,
}

//=============================================================================

impl<K, N, E> Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    /// Creates a new edge.
    pub fn new(source: &ArcNode<K, N, E>, target: &ArcNode<K, N, E>, data: E) -> Edge<K, N, E> {
        Edge {
            source: Arc::downgrade(source),
            target: Arc::downgrade(target),
            data: Mutex::new(data),
            lock: Arc::new(AtomicBool::new(OPEN)),
        }
    }

    /// Edge's source node.
    #[inline(always)]
    pub fn source(&self) -> ArcNode<K, N, E> {
        self.source.upgrade().unwrap()
    }

    /// Edge's target node.
    #[inline(always)]
    pub fn target(&self) -> ArcNode<K, N, E> {
        self.target.upgrade().unwrap()
    }

    /// Load data from the edge.
    #[inline(always)]
    pub fn load(&self) -> E {
        self.data.lock().unwrap().clone()
    }

    /// Store data into the edge.
    #[inline(always)]
    pub fn store(&self, data: E) {
        let mut x = self.data.lock().expect("Error locking mutex!");
        *x = data;
    }

    #[inline(always)]
    fn try_lock(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    #[inline(always)]
    fn close(&self) {
        self.lock.store(CLOSED, Ordering::Relaxed)
    }

    #[inline(always)]
    fn open(&self) {
        self.lock.store(OPEN, Ordering::Relaxed)
    }

    #[inline(always)]
    fn get_lock(&self) -> Weak<AtomicBool> {
        Arc::downgrade(&self.lock)
    }

    fn display_string(&self) -> String {
        let lock_state = if self.try_lock() == false {
            "OPEN"
        } else {
            "CLOSED"
        };
        format!(
            "-> \"{}\" : \"{}\" : \"{}\"",
            self.target().key(),
            lock_state,
            self.data.lock().unwrap()
        )
    }
}

//=============================================================================

unsafe impl<K, N, E> Sync for Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
}

impl<K, N, E> Clone for Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn clone(&self) -> Self {
        Edge {
            source: self.source.clone(),
            target: self.target.clone(),
            data: Mutex::new(self.data.lock().unwrap().clone()),
            lock: Arc::new(AtomicBool::new(self.lock.load(Ordering::Relaxed))),
        }
    }
}

impl<K, N, E> Display for Edge<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "Edge \"{}\" {}",
            self.source().key(),
            self.display_string()
        )
    }
}

//=============================================================================

pub fn backtrack_edges<K, N, E>(edges: &Vec<WeakEdge<K, N, E>>) -> Vec<WeakEdge<K, N, E>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    let mut res = Vec::new();
    if edges.len() == 0 {
        return res;
    }
    let w = edges.get(edges.len() - 1).unwrap();
    res.push(w.clone());
    let mut i = 0;
    for edge in edges.iter().rev() {
        let source = res[i].upgrade().unwrap().source();
        if edge.upgrade().unwrap().target() == source {
            res.push(edge.clone());
            i += 1;
        }
    }
    res
}

fn open_locks<K, N, E>(result: &Vec<WeakEdge<K, N, E>>)
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

//=============================================================================
// NODE IMPLEMENTATION
//=============================================================================

//=============================================================================
// TYPES

/// Nodes are handled through a atomically reference counted
/// smart pointer.
///
pub type ArcNode<K, N, E> = Arc<Node<K, N, E>>;

/// Sometimes we need to handle nodes through a weak reference.
///
pub type WeakNode<K, N, E> = Weak<Node<K, N, E>>;

type Outbound<K, N, E> = RwLock<Vec<ArcEdge<K, N, E>>>;
type Inbound<K, N, E> = RwLock<Vec<WeakEdge<K, N, E>>>;

//=============================================================================
// STRUCT

/// Represents a node in the graph. Data can be stored in and loaded from the
/// node in a thread safe manner.
///
#[derive(Debug)]
pub struct Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    key: K,
    data: Mutex<N>,
    outbound: Outbound<K, N, E>,
    inbound: Inbound<K, N, E>,
    lock: Arc<AtomicBool>,
}

//=============================================================================
// STRUCT IMPLEMENTATIONS

impl<K, N, E> Node<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	//=============================================================================
	// PUBLIC

    /// Create a new node.
    ///
    #[inline(always)]
    pub fn new(key: K, data: N) -> Node<K, N, E> {
        Self {
            key,
            data: Mutex::new(data),
            outbound: Outbound::new(Vec::new()),
            inbound: Inbound::new(Vec::new()),
            lock: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Load data from the node.
    ///
    #[inline(always)]
    pub fn load(&self) -> N {
        self.data.lock().unwrap().clone()
    }

    /// Store data to the node.
    ///
    #[inline(always)]
    pub fn store(&self, data: N) {
        *self.data.lock().unwrap() = data;
    }

    /// Get node key.
    ///
    #[inline(always)]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Get node degree ie. amount of outbound edges.
    ///
    #[inline(always)]
    pub fn degree(&self) -> usize {
        self.outbound().len()
    }

    /// Check if node is a leaf node ie. has no outbound edges.
    ///
    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        self.outbound().len() == 0
    }

    /// Find an outbound node and return the corresponding edge if found.
    ///
    #[inline(always)]
    pub fn find_outbound(&self, target: &ArcNode<K, N, E>) -> Option<ArcEdge<K, N, E>> {
        for edge in self.outbound().iter() {
            if edge.target() == *target {
                return Some(edge.clone());
            }
        }
        None
    }

    /// Find an inbound node and return the corresponding edge if found.
    ///
    #[inline(always)]
    pub fn find_inbound(&self, source: &ArcNode<K, N, E>) -> Option<WeakEdge<K, N, E>> {
        for edge in self.inbound().iter() {
            if edge.upgrade().unwrap().source() == *source {
                return Some(edge.clone());
            }
        }
        None
    }

    /// Get read access to outbound edges of the node.
    ///
    #[inline(always)]
    pub fn outbound(&self) -> RwLockReadGuard<Vec<Arc<Edge<K, N, E>>>> {
        let lock = self.outbound.read();
        match lock {
            Ok(guard) => guard,
            Err(error) => {
                panic!("RwLock error {}", error)
            }
        }
    }

    /// Get read and write access to the outbound edges of the node. Will block other threads.
    ///
    #[inline(always)]
    pub fn outbound_mut(&self) -> RwLockWriteGuard<Vec<Arc<Edge<K, N, E>>>> {
        let lock = self.outbound.write();
        match lock {
            Ok(guard) => guard,
            Err(error) => {
                panic!("RwLock error {}", error)
            }
        }
    }

    /// Get read access to inbound edges of the node.
    ///
    #[inline(always)]
    pub fn inbound(&self) -> RwLockReadGuard<Vec<Weak<Edge<K, N, E>>>> {
        let lock = self.inbound.read();
        match lock {
            Ok(guard) => guard,
            Err(error) => {
                panic!("RwLock error {}", error)
            }
        }
    }

    /// Get read and write access to the outbound edges of the node. Will block other threads.
    ///
    #[inline(always)]
    pub fn inbound_mut(&self) -> RwLockWriteGuard<Vec<Weak<Edge<K, N, E>>>> {
        let lock = self.inbound.write();
        match lock {
            Ok(guard) => guard,
            Err(error) => {
                panic!("RwLock error {}", error)
            }
        }
    }

	//=============================================================================
	// PRIVATE

    #[inline(always)]
    fn try_lock(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    #[inline(always)]
    fn get_lock(&self) -> Weak<AtomicBool> {
        Arc::downgrade(&self.lock)
    }

    #[inline(always)]
    fn close(&self) {
        self.lock.store(CLOSED, Ordering::Relaxed)
    }

    #[inline(always)]
    fn open(&self) {
        self.lock.store(OPEN, Ordering::Relaxed)
    }

    #[inline(always)]
    fn filter_adjacent(
        &self,
        user_closure: &dyn Fn(&ArcEdge<K, N, E>) -> Traverse,
    ) -> Continue<Vec<WeakEdge<K, N, E>>>
    where
        K: Hash + Eq + Clone + Debug + Display + Sync + Send,
        N: Clone + Debug + Display + Sync + Send,
        E: Clone + Debug + Display + Sync + Send,
    {
        let mut segment: Vec<WeakEdge<K, N, E>> = Vec::new();
        let adjacent_edges = self.outbound();
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

	#[inline(always)]
    fn par_filter_adjacent<F>(
        &self,
        user_closure: &F,
    ) -> Continue<Vec<WeakEdge<K, N, E>>>
    where
        K: Hash + Eq + Clone + Debug + Display + Sync + Send,
        N: Clone + Debug + Display + Sync + Send,
        E: Clone + Debug + Display + Sync + Send,
		F: Fn(&ArcEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
    {
		let found = AtomicBool::new(false);
        let adjacent_edges = self.outbound();
		let segment: Vec<WeakEdge<K, N, E>> = adjacent_edges
			.par_iter()
			.filter_map(
				|edge| {
					if edge.try_lock() == OPEN
					&& edge.target().try_lock() == OPEN
					&& found.load(Ordering::Relaxed) == false {
						edge.target().close();
						edge.close();
						let traversal_state = user_closure(edge);
						match traversal_state {
							Traverse::Include => {
								return Some(Arc::downgrade(edge));
							}
							Traverse::Finish => {
								found.store(true, Ordering::Relaxed);
								return Some(Arc::downgrade(edge));
							}
							Traverse::Skip => {
								edge.open();
								edge.target().open();
							}
						}
					}
					None
				}
			)
			.collect();
		match found.load(Ordering::Relaxed) {
			true => { return Continue::No(segment); }
			false => { return Continue::Yes(segment); }
		}
    }

    fn display_string(&self) -> String {
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

//=============================================================================
// TRAIT IMPLEMENTATIONS

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
            outbound: Outbound::new(Vec::new()),
            inbound: Inbound::new(Vec::new()),
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

//=============================================================================
// FUNCTION IMPLEMENTATIONS

#[inline]
fn overlaps<K, N, E>(source: &ArcNode<K, N, E>, target: &ArcNode<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    for edge in source.outbound().iter() {
        if edge.target() == *target {
            return true;
        }
    }
    false
}

/// Connect two nodes if no previous connection exists.
pub fn connect<K, N, E>(source: &ArcNode<K, N, E>, target: &ArcNode<K, N, E>, data: E) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    if !overlaps(source, target) {
        let new_edge = ArcEdge::new(Edge::new(source, target, data));
        source.outbound_mut().push(new_edge.clone());
        target.inbound_mut().push(Arc::downgrade(&new_edge));
        return true;
    }
    false
}

/// Disconnect two nodes from each other if they share an edge.
pub fn disconnect<K, N, E>(source: &ArcNode<K, N, E>, target: &ArcNode<K, N, E>) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    let mut idx: (usize, usize) = (0, 0);
    let mut flag = false;
    for (i, edge) in source.outbound().iter().enumerate() {
        if edge.target() == *target {
            idx.0 = i;
            flag = true;
        }
    }
    for (i, edge) in target.inbound().iter().enumerate() {
        if edge.upgrade().unwrap().source() == *source {
            idx.1 = i;
        }
    }
    if flag == true {
        source.outbound_mut().remove(idx.0);
        source.inbound_mut().remove(idx.0);
    }
    flag
}

//=============================================================================
// TRAVERSAL ALGORITHMS
//=============================================================================

/// # Breadth Traversal
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
/// Function will return an `Option<Vec<WeakEdge<K, N, E>>>` where a Some value
/// indicates that the traversal was successful ie. a Finish condition was
/// reached. And WeakEdges is a collection of all the traversed edges.
/// The last edge will contain the result that triggered the Finish condition.
/// To get the shortest path for example, we'd backtrack the WeakEdges starting
/// from the last edge which would contain our sink node.
///
/// # Examples
///
/// ```
/// use graph::core::*;
///
/// let n1 = ArcNode::new(Node::<u32, Empty, Empty>::new(1, Empty));
/// let n2 = ArcNode::new(Node::<u32, Empty, Empty>::new(2, Empty));
/// let n3 = ArcNode::new(Node::<u32, Empty, Empty>::new(3, Empty));
///
/// connect(&n1, &n2, Empty);
/// connect(&n2, &n3, Empty);
/// connect(&n1, &n3, Empty);
///
/// let edges = directed_breadth_traversal(&n1,
/// 	| edge | {
/// 		if n3 == edge.target() {
///				Traverse::Finish
///			} else {
///				Traverse::Include
///			}
/// 	})
/// 	.unwrap();
///
/// let shortest_path = backtrack_edges(&edges);
///
/// assert!(shortest_path.len() == 1);
/// ```
///
pub fn directed_breadth_traversal<K, N, E, F>(
    source: &ArcNode<K, N, E>,
    explorer: F,
) -> Option<Vec<WeakEdge<K, N, E>>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&ArcEdge<K, N, E>) -> Traverse + Sync + Send + Copy,
{
    let mut frontiers: Vec<WeakEdge<K, N, E>>;
    let mut bounds: (usize, usize) = (0, 0);
    source.close();
    let initial = source.filter_adjacent(&explorer);
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
            let haystack = node.filter_adjacent(&explorer);
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

//=============================================================================

/// # Parallel Breadth Traversal
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
/// Function will return an `Option<Vec<WeakEdge<K, N, E>>>` where a Some value
/// indicates that the traversal was successful ie. a Finish condition was
/// reached. And WeakEdges is a collection of all the traversed edges.
/// The last edge will contain the result that triggered the Finish condition.
/// To get the shortest path for example, we'd backtrack the WeakEdges starting
/// from the last edge which would contain our sink node.
///
/// # Examples
///
/// ```
/// use graph::core::*;
///
/// let n1 = ArcNode::new(Node::<u32, Empty, Empty>::new(1, Empty));
/// let n2 = ArcNode::new(Node::<u32, Empty, Empty>::new(2, Empty));
/// let n3 = ArcNode::new(Node::<u32, Empty, Empty>::new(3, Empty));
///
/// connect(&n1, &n2, Empty);
/// connect(&n2, &n3, Empty);
/// connect(&n1, &n3, Empty);
///
/// let edges = parallel_directed_breadth_traversal(&n1,
/// 	| edge | {
/// 		if n3 == edge.target() {
///				Traverse::Finish
///			} else {
///				Traverse::Include
///			}
/// 	})
/// 	.unwrap();
///
/// let shortest_path = backtrack_edges(&edges);
///
/// assert!(shortest_path.len() == 1);
/// ```
///
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
    let mut frontiers: Vec<WeakEdge<K, N, E>>;
    let mut bounds: (usize, usize) = (0, 0);
    let terminate: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    source.close();
    match source.par_filter_adjacent(&explorer) {
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
				match terminate.load(Ordering::Relaxed) {
					true => { None }
					false => {
						let node = edge.upgrade().unwrap().target();
                    	match node.par_filter_adjacent(&explorer) {
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

//=============================================================================

/// # Depth First Traversal
///
/// Conduct a depth first traversal starting from the source node.
/// User provides an `explorer` closure which determines how nodes and edges
/// are to be interpreted. The closure will return a Traverse enum which has
/// 3 states Include, Skip and Finish. These states determine if we are to
/// "go through" the edge and thus include it in our search. Include will
/// include the edge and continue the search, Skip will indicate that the edge
/// is not to be traversed and Finish will include the edge and finish the
/// algorithm.
///
/// Function will return an `Option<Vec<WeakEdge<K, N, E>>>` where a Some value
/// indicates that the traversal was successful ie. a Finish condition was
/// reached. And WeakEdges is a collection of all the traversed edges.
/// The last edge will contain the result that triggered the Finish condition.
/// To get the shortest path for example, we'd backtrack the WeakEdges starting
/// from the last edge which would contain our sink node.
/// # Examples
///
/// ```
/// use graph::node::*;
/// use graph::global::*;
/// use graph::node::Traverse::*;
/// use graph::traverse::Continue;
/// use graph::traverse::*;
///
/// let n1 = ArcNode::new(Node::<u32, Empty, Empty>::new(1, Empty));
/// let n2 = ArcNode::new(Node::<u32, Empty, Empty>::new(2, Empty));
/// let n3 = ArcNode::new(Node::<u32, Empty, Empty>::new(3, Empty));
///
/// connect(&n1, &n2, Empty);
/// connect(&n2, &n3, Empty);
/// connect(&n1, &n3, Empty);
///
/// let edges = directed_depth_traversal(&n1,
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
/// assert!(shortest_path.len() == 2);
/// ```
///
fn directed_depth_traversal_recursion<K, N, E, F>(
    source: &ArcNode<K, N, E>,
    results: &mut Vec<WeakEdge<K, N, E>>,
    locks: &mut Vec<Weak<AtomicBool>>,
    explorer: F,
) -> bool
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&ArcEdge<K, N, E>) -> Traverse,
{
    source.close();
    locks.push(source.get_lock());
    for edge in source.outbound().iter() {
        if edge.try_lock() == OPEN && edge.target().try_lock() == OPEN {
            edge.target().close();
            edge.close();
            locks.push(edge.get_lock());
            let traverse = explorer(edge);
            match traverse {
                Traverse::Include => {
                    results.push(Arc::downgrade(edge));
                    locks.push(edge.target().get_lock());
                }
                Traverse::Finish => {
                    results.push(Arc::downgrade(edge));
                    locks.push(edge.target().get_lock());
                    return true;
                }
                Traverse::Skip => {
                    edge.target().open();
                }
            }
            return directed_depth_traversal_recursion(&edge.target(), results, locks, explorer);
        }
    }
    false
}

pub fn directed_depth_traversal<K, N, E, F>(
    source: &ArcNode<K, N, E>,
    explorer: F,
) -> Option<Vec<WeakEdge<K, N, E>>>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
    F: Fn(&ArcEdge<K, N, E>) -> Traverse,
{
    let mut result = Vec::new();
    let mut locks = Vec::new();
    let res = directed_depth_traversal_recursion(source, &mut result, &mut locks, explorer);
    for weak in locks {
        let arc = weak.upgrade().unwrap();
        arc.store(OPEN, Ordering::Relaxed);
    }
    match res {
        true => Some(result),
        false => None,
    }
}

//=============================================================================
