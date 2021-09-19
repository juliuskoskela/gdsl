// pub mod digraph_vertex;
pub mod digraph;
pub mod node;
pub mod edge;
pub mod edge_list;
pub mod global;

#[cfg(test)]
mod tests {
	use crate::digraph::*;
	use rand::{thread_rng, Rng};
	use rand::distributions::Alphanumeric;

	fn rand_string(len: usize) -> String {
		thread_rng()
        	.sample_iter(&Alphanumeric)
        	.take(6)
        	.map(char::from)
        	.collect()
	}

	fn rand_keys(count: usize, keysize: usize) -> Vec<String> {
		let mut i = 0;
		let mut keys = vec![];
		while i < count {
			keys.push(rand_string(keysize));
			i += 1;
		}
		keys
	}

    #[test]
    fn basic() {
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
				for edge in edge_list {
					println!("{}", edge.target());
				}
			}
			None => println!("Target not found!")
		}
	}

	#[test]
	fn stress() {
		type MyGraph = Digraph<String, usize, usize>;

		let mut g = MyGraph::new();
		let keys = rand_keys(100, 6);
		for key in keys.iter() {
			g.insert(key.clone(), 0);
		}

		// println!("node count = {}", g.node_count());
	}
}
