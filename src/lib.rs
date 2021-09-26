pub mod digraph;
pub mod node;
pub mod edge;
pub mod edge_list;
pub mod global;
pub mod example;

#[cfg(test)]
mod tests {
	use crate::{digraph::*, global::*, node::depth_traversal_directed, node::Traverse::*};
	use crate::node::Node;
	use crate::edge::Edge;

    #[test]
    fn digraph_test_breadth_first_search() {
		type MyGraph<'a> = Digraph<&'a str, usize, usize>;

		let mut g = MyGraph::new();
		g.insert("N1", 1);
		g.insert("N2", 0);
		g.insert("N3", 0);
		g.insert("N4", 3);
		g.insert("N5", 2);
		g.insert("N6", 1);
		g.connect(&"N1", &"N2", 16);
		g.connect(&"N2", &"N3", 12);
		g.connect(&"N3", &"N4", 19);
		g.connect(&"N4", &"N5", 22);
		g.connect(&"N5", &"N3", 25);
		g.connect(&"N3", &"N6", 38);
		g.connect(&"N1", &"N5", 23);
		g.connect(&"N2", &"N4", 83);
		g.connect(&"N6", &"N2", 67);
		g.connect(&"N3", &"N1", 27);
		g.connect(&"N1", &"N3", 58);
		g.disconnect(&"N4", &"N5");

		let res = g.bfs(&"N1", &"N6");
		match res {
			Some(edge_list) => {
				for edge in edge_list.iter() {
					println!("{}", edge);
				}
			}
			None => println!("Target not found!")
		}
	}

	#[test]
	fn digraph_test_get_leaves() {
		type MyGraph<'a> = Digraph<&'a str, usize, usize>;

		let mut g = MyGraph::new();
		g.insert("N1", 1);
		g.insert("N2", 0);
		g.insert("N3", 0);
		g.insert("N4", 3);
		g.insert("N5", 2);
		g.insert("N6", 1);
		g.connect(&"N1", &"N2", 16);
		g.connect(&"N2", &"N3", 12);
		g.connect(&"N3", &"N4", 19);
		g.connect(&"N4", &"N5", 22);
		g.connect(&"N5", &"N3", 25);
		g.connect(&"N3", &"N6", 38);
		g.connect(&"N1", &"N5", 23);
		g.connect(&"N2", &"N4", 83);
		g.connect(&"N3", &"N1", 27);
		g.connect(&"N1", &"N3", 58);

		let res = g.get_leaves();

		for n in res {
			println!("{}", n);
		}
	}

	#[test]
	fn digraph_test_depth_traversal() {

		type MyGraph<'a> = Digraph<&'a str, usize, usize>;

		let mut g = MyGraph::new();

		g.insert("N1", 1);
		g.insert("N2", 0);
		g.insert("N3", 0);
		g.insert("N4", 3);
		g.insert("N5", 2);
		g.insert("N6", 1);
		g.connect(&"N1", &"N2", 16);
		g.connect(&"N2", &"N3", 12);
		g.connect(&"N3", &"N4", 19);
		g.connect(&"N4", &"N5", 22);
		g.connect(&"N5", &"N3", 25);
		g.connect(&"N3", &"N6", 38);
		g.connect(&"N1", &"N5", 23);
		g.connect(&"N2", &"N4", 83);
		g.connect(&"N3", &"N1", 27);
		g.connect(&"N1", &"N3", 58);

		let e = g.edge(&"N1", &"N2");

		match e {
			Some(_) => { }
			None => { assert!(1 == 0) }
		}

		let res = g.depth_first(&"N1", &"N5",
			|e, t|
			{
				if e.target() == *t {
					return Finish ;
				}
				Collect
			}
		).unwrap();

		for e in res.iter() {
			println!("{}", e);
		}
	}

	#[test]
	fn digraph_test_breadth_traversal() {

		type MyGraph<'a> = Digraph<&'a str, usize, usize>;

		let mut g = MyGraph::new();

		g.insert("N1", 1);
		g.insert("N2", 0);
		g.insert("N3", 0);
		g.insert("N4", 3);
		g.insert("N5", 2);
		g.insert("N6", 1);
		g.connect(&"N1", &"N2", 16);
		g.connect(&"N2", &"N3", 12);
		g.connect(&"N3", &"N4", 19);
		g.connect(&"N4", &"N5", 22);
		g.connect(&"N5", &"N3", 25);
		g.connect(&"N3", &"N6", 38);
		g.connect(&"N1", &"N5", 23);
		g.connect(&"N2", &"N4", 83);
		g.connect(&"N3", &"N1", 27);
		g.connect(&"N1", &"N3", 58);

		let res = g.breadth_first(&"N1", &"N6",
			|_|
			{
				Collect
			}
		).unwrap();

		for e in res.iter() {
			println!("{}", e);
		}
	}

	#[test]
	fn edmonds_karp() {

		#[derive(Clone, Debug)]
		struct Void;

		impl std::fmt::Display for Void {
			fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(fmt, "_")
			}
		}

		#[derive(Clone, Debug)]
		struct Flow {
			pub max: i64,
			pub cur: i64,
			pub rev: Option<EdgeRef<usize, Void, Flow>>,
		}

		impl std::fmt::Display for Flow {
			fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(fmt, "{}/{}", self.cur, self.max)
			}
		}

		type FlowGraph<'a> = Digraph<usize, Void, Flow>;

		println!("sizeof node = {}", std::mem::size_of::<Node<usize, Void, Flow>>());
		println!("sizeof edge = {}", std::mem::size_of::<Edge<usize, Void, Flow>>());
		let size = (std::mem::size_of::<Node<usize, Void, Flow>>() * 100_000)
		+ (std::mem::size_of::<Edge<usize, Void, Flow>>() * 100_000 * 100);
		println!("Expected size for a graph with 100,000 nodes of degree 100 = {} Mb", size / 1000_000);
		let mut g = FlowGraph::new();

		g.insert(1, Void);
		g.insert(2, Void);
		g.insert(3, Void);
		g.insert(4, Void);
		g.insert(5, Void);
		g.insert(6, Void);

		fn connect_flow(g: &FlowGraph, u: &usize, v: &usize, flow: i64) {
			g.connect(u, v, Flow { max: flow, cur: 0, rev: None });
			g.connect(v, u, Flow { max: flow, cur: flow, rev: None });
			let uv = g.edge(u, v).unwrap();
			let vu = g.edge(v, u).unwrap();
			let mut uv_data = uv.load();
			let mut vu_data = vu.load();
			uv_data.rev = Some(vu.clone());
			vu_data.rev = Some(uv.clone());
			uv.store(uv_data);
			vu.store(vu_data);
		}

		connect_flow(&g, &1, &2, 16);
		connect_flow(&g, &1, &3, 13);
		connect_flow(&g, &2, &3, 10);
		connect_flow(&g, &2, &4, 12);
		connect_flow(&g, &3, &2, 4);
		connect_flow(&g, &3, &5, 14);
		connect_flow(&g, &4, &3, 9);
		connect_flow(&g, &4, &6, 20);
		connect_flow(&g, &5, &4, 7);
		connect_flow(&g, &5, &6, 4);

		let mut max_flow: i64 = 0;

		while let Some(b) = g.breadth_first(&1, &6,
			|e| {
				let flow = e.load();
				if flow.cur < flow.max { Collect } else { Skip }
		})
		{
			let mut aug_flow = std::i64::MAX;
			let path = b.backtrack().unwrap();
			for e in path.iter() {
				let flow = e.load();
				if flow.max - flow.cur < aug_flow {
					aug_flow = flow.max - flow.cur;
				}
			}
			for e in path.iter() {
				let mut flow = e.load();
				let r = flow.rev.as_ref().unwrap().clone();
				let mut rev_flow = r.load();
				flow.cur += aug_flow;
				rev_flow.cur -= aug_flow;
				e.store(flow);
				r.store(rev_flow);
			}

			max_flow += aug_flow;
		}

		assert!(max_flow == 23);
	}
}
