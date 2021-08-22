use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, Index};
use std::rc::{Rc, Weak};
use std::collections::HashMap;

type NodeRef<N, E> = Weak<GraphNode<N, E>>;
type NodeEdges<N, E> = HashMap<String, GraphEdge<N, E>>;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct GraphNode<N, E> {
    name: String,
	arg: N,
    ein: NodeEdges<N, E>,
    eout: NodeEdges<N, E>,
	valid: bool,
}

impl<N, E> GraphNode<N, E>
where N: Copy + Clone, E: Copy + Clone {
	fn new(name: &str, arg: N) -> GraphNode<N, E> {
		GraphNode {
			name: name.to_string(),
			arg,
			ein: NodeEdges::new(),
			eout: NodeEdges::new(),
			valid: true
		}
	}
	fn close(&mut self) {
		self.valid = false;
	}
	fn open(&mut self) {
		self.valid = true;
	}
	fn collect_valid_children(&mut self) -> Vec<NodeRef<N, E>> {
		let mut ret : Vec<NodeRef<N, E>> = Vec::new();
		for (_, edge) in self.eout.iter() {
			if edge.valid == true {
				ret.push(edge.v.clone());
			}
		}
		return ret
	}
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct GraphEdge<N, E> {
    u: NodeRef<N, E>,
	v: NodeRef<N, E>,
	arg: E,
	valid: bool,
}

impl<N, E> GraphEdge<N, E>
where N: Copy + Clone, E: Copy + Clone {
	fn new(u: NodeRef<N, E>, v: NodeRef<N, E>, arg: E) -> Self {
		GraphEdge {
			u: u.clone(),
			v: v.clone(),
			arg,
			valid: true
		}
	}
}

///////////////////////////////////////////////////////////////////////////////

type NodeAlloc<N, E> = Rc<RefCell<GraphNode<N, E>>>;
type NodePool<N, E> = RefCell<HashMap<String, NodeAlloc<N, E>>>;

struct Graph<N, E> {
	pool: NodePool<N, E>
}

impl<N, E> Graph<N, E>
where N: Copy + Clone, E: Copy + Clone {
	fn new() -> Self {
		Self {
			pool: NodePool::new(HashMap::new())
		}
	}
	fn connect(&mut self, edge: GraphEdge<N, E>) {
		let u_name = edge.u.upgrade().unwrap().name.clone();
		let u_ref = self.pool.borrow_mut();
		let u_cell = u_ref.get(&u_name).unwrap();
		let u_node_ref = u_cell.borrow();
		let u_node = u_node_ref.deref();
		u_node.eout.insert(u_name, edge);
		// u_node.borrow_mut().eout.insert(u_name, edge);
		// let u_node = self.pool.get().unwrap();
	}
	fn edge(&self, u: &str, v: &str, arg: E) {
		let u_ref = self.pool.borrow_mut();
		let v_ref = self.pool.borrow_mut();
		let u_node = u_ref.get(u).unwrap();
		let v_node = v_ref.get(v).unwrap();
		// let edge = GraphEdge::new(Rc::downgrade(u_node), Rc::downgrade(v_node), arg);
		// self.connect(edge);
	}
}

///////////////////////////////////////////////////////////////////////////////

fn main () {
	let n = GraphNode::<i32, i32>::new("new", 1);
	let r = Rc::new(RefCell::new(n));
	*r.borrow_mut().valid = true;
}
