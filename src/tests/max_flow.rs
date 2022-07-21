
#[test]
fn max_flow() {
	use crate::node::*;
	use crate::graph::*;
	use crate::*;
	use std::rc::{Weak, Rc};
	use std::cell::Cell;

	#[derive(Clone)]
	pub struct Flow(Rc<Cell<(u64, u64)>>, Weak<Cell<(u64, u64)>>);

	impl Flow {
		pub fn new(max: u64, cur: u64) -> Self {
			Flow(Rc::new(Cell::new((max, cur))), Weak::new())
		}

		pub fn connect(s: &Node<usize, Empty, Flow>, t: &Node<usize, Empty, Flow>, amount: u64) {
			let mut fflow = Flow::new(amount, 0);
			let mut rflow = Flow::new(amount, amount);
			fflow.1 = Rc::downgrade(&rflow.0);
			rflow.1 = Rc::downgrade(&fflow.0);
			connect!(s => t, fflow);
			connect!(t => s, rflow);
		}

		pub fn update(&self, amount: u64) {
			let f = self.0.get();
			let r = self.1.upgrade().unwrap().get();
			self.0.set((f.0, f.1 + amount));
			self.1.upgrade().unwrap().set((r.0, r.1 - amount));
		}

		pub fn max(&self) -> u64 {
			self.0.get().0
		}

		pub fn cur(&self) -> u64 {
			self.0.get().1
		}
	}

	impl FmtDot for Flow {
		fn fmt_dot(&self) -> String {
			format!("{}/{}", self.cur(), self.max())
		}
	}

	let mut graph = crate::graph::Graph::new();

	graph.insert(node!(0));
	graph.insert(node!(1));
	graph.insert(node!(2));
	graph.insert(node!(3));
	graph.insert(node!(4));
	graph.insert(node!(5));

	Flow::connect(&graph[0], &graph[1], 16);
	Flow::connect(&graph[0], &graph[2], 13);
	Flow::connect(&graph[1], &graph[2], 10);
	Flow::connect(&graph[1], &graph[3], 12);
	Flow::connect(&graph[2], &graph[1], 4);
	Flow::connect(&graph[2], &graph[4], 14);
	Flow::connect(&graph[3], &graph[2], 9);
	Flow::connect(&graph[3], &graph[5], 20);
	Flow::connect(&graph[4], &graph[3], 7);
	Flow::connect(&graph[4], &graph[5], 4);

	// Maximum flow algorithm
	let mut max_flow: u64 = 0;

	// 1. We loop breadth-first until there is no more paths to explore
	while let Some(bfs) = graph[0].search().bfs_map(&graph[5], &|edge| {

		// 2. We exclude saturated edges from the search and terminate
		// if we reach the target
		edge.cur() < edge.max()
	}) {
		let path = bfs.edge_path();
		let mut augmenting_flow = std::u64::MAX;

		// 3. We find the minimum augmenting flow along the path
		for e in &path {
			if e.max() - e.cur() < augmenting_flow {
				augmenting_flow = e.max() - e.cur();
			}
		}

		// 4. We augment the flow along the path
		for e in &path {
			e.update(augmenting_flow);
		}

		// 5. We update the maximum flow
		max_flow += augmenting_flow;
	}

	// For this graph we expect the maximum flow from 0 -> 5 to be 23
	assert!(max_flow == 23);
}
