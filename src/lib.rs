pub mod graph;
pub mod graph_node;
pub mod graph_edge;

#[cfg(test)]
mod test {
    use crate::graph::Graph;

	#[test]
    fn graph_create() {
        let graph = Graph::<String, f64, f64>::new();
		println!("{:?}", graph);
    }
	#[test]
	fn graph_add_nodes() {
		let mut graph = Graph::<usize, f64, f64>::new();
		graph.add_node(&0x0, 1.0);
		graph.add_node(&0x1, 2.0);
		graph.add_node(&0x2, 3.0);
		assert_eq!(graph.get_node(&0x0).unwrap().arg, 1.0);
		assert_eq!(graph.get_node(&0x1).unwrap().arg, 2.0);
		assert_eq!(graph.get_node(&0x2).unwrap().arg, 3.0);
		println!("{:?}", graph);
	}
	#[test]
	fn graph_add_edges() {
		let mut graph = Graph::<usize, f64, f64>::new();
		graph.add_node(&0x0, 1.0);
		graph.add_node(&0x1, 2.0);
		graph.add_node(&0x2, 3.0);
		graph.add_node(&0x3, 4.0);
		graph.add_node(&0x4, 5.0);
		graph.connect(&0x0, &0x1, 10.0);
		graph.connect(&0x1, &0x2, 20.0);
		graph.connect(&0x1, &0x4, 30.0);
		graph.connect(&0x2, &0x3, 40.0);
		println!("{:?}", graph);
	}

	#[test]
	fn graph_update_node() {
		let mut graph = Graph::<u64, u64, u64>::new();
		graph.add_node(&0x0, 1);
		let mut node = graph.get_node(&0x0).unwrap();
		node.arg = 2;
		graph.update_node(node);
		assert_eq!(graph.get_node(&0x0).unwrap().arg, 2);
	}

	#[test]
	fn graph_bfs() {
		let mut graph = Graph::<usize, f64, f64>::new();
		graph.add_node(&0x0, 1.0);
		graph.add_node(&0x1, 2.0);
		graph.add_node(&0x2, 3.0);
		graph.add_node(&0x3, 4.0);
		graph.add_node(&0x4, 5.0);
		graph.connect(&0x0, &0x1, 10.0);
		graph.connect(&0x1, &0x2, 20.0);
		graph.connect(&0x1, &0x4, 30.0);
		graph.connect(&0x2, &0x3, 40.0);
		let edges = graph.bfs(&0x0, &0x3).unwrap();
		let expect = vec![1, 2, 4, 3];
		let mut i : usize;
		i = 0;
		for e in edges.iter() {
			assert_eq!(expect[i], e.v);
			i += 1;
		}
	}

	#[test]
	fn graph_struct_arg() {
		#[derive(Debug)]
		struct Node {
			a: i64,
			b: f64,
			c: String,
		}
		impl Clone for Node {
			fn clone(&self) -> Self {
				Node { a: self.a, b: self.b, c: self.c.clone() }
			}
		}
		#[derive(Debug)]
		struct Edge {
			a: i64,
			b: f64,
			c: String,
		}
		impl Clone for Edge {
			fn clone(&self) -> Self {
				Edge { a: self.a, b: self.b, c: self.c.clone() }
			}
		}
		let mut graph = Graph::<String, Node, Edge>::new();
		graph.add_node(&"n1".to_string(), Node {a: 1, b: 1.0, c: "arg1".to_string()});
		graph.add_node(&"n1".to_string(), Node {a: 1, b: 1.0, c: "arg2".to_string()});
		graph.add_node(&"n1".to_string(), Node {a: 1, b: 1.0, c: "arg3".to_string()});
	}
}
