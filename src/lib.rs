// pub mod digraph_vertex;
pub mod digraph;
pub mod node;
pub mod edge;
pub mod edge_list;
pub mod global;

#[cfg(test)]
mod tests {
	use crate::digraph::*;

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
}
