pub mod digraph;
pub mod node;
pub mod edge;
pub mod edge_list;
pub mod global;

#[cfg(test)]
mod tests {
	use crate::digraph::*;

	type TestGraph<'a> = Digraph<&'a str, usize, usize>;

    #[test]
    fn digraph_test_shortest_path() {

		let mut g = TestGraph::new();
		g.insert("N1", 1);
		g.insert("N2", 0);
		g.insert("N3", 0);
		g.insert("N4", 3);
		g.insert("N5", 2);
		g.insert("N6", 1);
		g.connect(&"N1", &"N2", 16);
		g.connect(&"N1", &"N3", 58);
		g.connect(&"N1", &"N4", 23);
		g.connect(&"N2", &"N3", 12);
		g.connect(&"N2", &"N4", 83);
		g.connect(&"N3", &"N4", 19);
		g.connect(&"N3", &"N2", 38);
		g.connect(&"N3", &"N1", 27);
		g.connect(&"N4", &"N5", 22);
		g.connect(&"N5", &"N3", 25);
		g.connect(&"N5", &"N6", 58);
		g.connect(&"N6", &"N2", 67);

		let res = g.bfs(&"N1", &"N6");
		match res {
			Some(edge_list) => {
				let path = edge_list.backtrack();
				assert!(*path.list[0].target().key() == "N6");
				assert!(*path.list[1].target().key() == "N5");
				assert!(*path.list[2].target().key() == "N4");
			}
			None => assert!(0 == 1)
		}
	}
}
