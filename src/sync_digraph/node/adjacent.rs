use super::*;

pub struct Adjacent<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display,
    N: Clone,
    E: Clone,
{
    outbound: Vec<(Node<K, N, E>, E)>,
    inbound: Vec<(Node<K, N, E>, E)>,
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

    pub fn get_outbound(&self, idx: usize) -> Option<(Node<K, N, E>, E)> {
        match self.outbound.get(idx) {
            Some(edge) => Some((edge.0.clone(), edge.1.clone())),
            None => None,
        }
    }

    pub fn get_inbound(&self, idx: usize) -> Option<(Node<K, N, E>, E)> {
        match self.inbound.get(idx) {
            Some(edge) => Some((edge.0.clone(), edge.1.clone())),
            None => None,
        }
    }

    pub fn find_outbound(&self, node: &K) -> Option<(Node<K, N, E>, E)> {
        for edge in self.outbound.iter() {
            if edge.0.key() == node {
                return Some((edge.0.clone(), edge.1.clone()));
            }
        }
        None
    }

    pub fn find_inbound(&self, node: &K) -> Option<(Node<K, N, E>, E)> {
        for edge in self.inbound.iter() {
            if edge.0.key() == node {
                return Some((edge.0.clone(), edge.1.clone()));
            }
        }
        None
    }

    pub fn len_outbound(&self) -> usize {
        self.outbound.len()
    }

    pub fn len_inbound(&self) -> usize {
        self.inbound.len()
    }

    pub fn push_inbound(&mut self, edge: (Node<K, N, E>, E)) {
        self.inbound.push(edge);
    }

    pub fn push_outbound(&mut self, edge: (Node<K, N, E>, E)) {
        self.outbound.push(edge);
    }

    pub fn remove_inbound(&mut self, source: &K) -> Result<E, ()> {
        let idx = self.inbound.iter().position(|edge| edge.0.key() == source);
        match idx {
            Some(idx) => {
                let edge = self.inbound.remove(idx);
                Ok(edge.1.clone())
            }
            None => Err(()),
        }
    }

    pub fn remove_outbound(&mut self, target: &K) -> Result<E, ()> {
        let idx = self.outbound.iter().position(|edge| edge.0.key() == target);
        match idx {
            Some(idx) => {
                let edge = self.outbound.remove(idx);
                Ok(edge.1.clone())
            }
            None => Err(()),
        }
    }

    pub fn clear_inbound(&mut self) {
        self.inbound.clear();
    }

    pub fn clear_outbound(&mut self) {
        self.outbound.clear();
    }

	pub fn sizeof(&self) -> usize {
		self.inbound.len() + self.outbound.len()
			* (std::mem::size_of::<Node<K, N, E>>()
			+ std::mem::size_of::<E>())
			+ std::mem::size_of::<Self>()
	}
}