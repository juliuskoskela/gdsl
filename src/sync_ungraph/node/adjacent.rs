use super::*;

#[derive(Clone)]
pub struct WeakNode<K = usize, N = (), E = ()>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    inner: Weak<(K, N, RwLock<Adjacent<K, N, E>>)>,
}

impl<K, N, E> WeakNode<K, N, E>
where
    K: Clone + Hash + Display + PartialEq + Eq,
    N: Clone,
    E: Clone,
{
    pub fn upgrade(&self) -> Option<Node<K, N, E>> {
        self.inner.upgrade().map(|inner| Node { inner })
    }

    pub fn downgrade(node: &Node<K, N, E>) -> Self {
        WeakNode {
            inner: Arc::downgrade(&node.inner),
        }
    }
}

pub struct Adjacent<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    outbound: Vec<(WeakNode<K, N, E>, E)>,
    inbound: Vec<(WeakNode<K, N, E>, E)>,
}

impl<K, N, E> Adjacent<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    pub fn new() -> RwLock<Self> {
        RwLock::new(Self {
            outbound: Vec::new(),
            inbound: Vec::new(),
        })
    }

    pub fn get_adjacent(&self, idx: usize) -> Option<(&WeakNode<K, N, E>, &E)> {
        match self.outbound.get(idx) {
            Some(edge) => Some((&edge.0, &edge.1)),
            None => self
                .inbound
                .get(idx - self.outbound.len())
                .map(|edge| (&edge.0, &edge.1)),
        }
    }

    pub fn find_outbound(&self, node: &K) -> Option<(&WeakNode<K, N, E>, &E)> {
        for edge in self.outbound.iter() {
            if edge.0.upgrade().unwrap().key() == node {
                return Some((&edge.0, &edge.1));
            }
        }
        None
    }

    pub fn find_inbound(&self, node: &K) -> Option<(&WeakNode<K, N, E>, &E)> {
        for edge in self.inbound.iter() {
            if edge.0.upgrade().unwrap().key() == node {
                return Some((&edge.0, &edge.1));
            }
        }
        None
    }

    pub fn find_adjacent(&self, node: &K) -> Option<(&WeakNode<K, N, E>, &E)> {
        match self.find_outbound(node) {
            Some(edge) => Some(edge),
            None => self.find_inbound(node),
        }
    }

    pub fn len_outbound(&self) -> usize {
        self.outbound.len()
    }

    pub fn len_inbound(&self) -> usize {
        self.inbound.len()
    }

    pub fn push_inbound(&mut self, edge: (Node<K, N, E>, E)) {
        self.inbound.push((WeakNode::downgrade(&edge.0), edge.1));
    }

    pub fn push_outbound(&mut self, edge: (Node<K, N, E>, E)) {
        self.outbound.push((WeakNode::downgrade(&edge.0), edge.1));
    }

    pub fn remove_inbound(&mut self, source: &K) -> Result<E, ()> {
        for (idx, edge) in self.inbound.iter().enumerate() {
            if edge.0.upgrade().unwrap().key() == source {
                return Ok(self.inbound.remove(idx).1);
            }
        }
        Err(())
    }

    pub fn remove_outbound(&mut self, target: &K) -> Result<E, ()> {
        for (idx, edge) in self.outbound.iter().enumerate() {
            if edge.0.upgrade().unwrap().key() == target {
                return Ok(self.outbound.remove(idx).1);
            }
        }
        Err(())
    }

    pub fn remove_undirected(&mut self, node: &K) -> Result<E, ()> {
        match self.remove_inbound(node) {
            Ok(edge) => Ok(edge),
            Err(_) => self.remove_outbound(node),
        }
    }

    pub fn clear_inbound(&mut self) {
        self.inbound.clear();
    }

    pub fn clear_outbound(&mut self) {
        self.outbound.clear();
    }

    pub fn sizeof(&self) -> usize {
        self.inbound.len()
            + self.outbound.len()
                * (std::mem::size_of::<Node<K, N, E>>() + std::mem::size_of::<E>())
            + std::mem::size_of::<Self>()
    }
}
