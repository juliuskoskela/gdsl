use crate::*;
use crate::graph::*;

pub trait FmtDot {
	fn fmt_dot(&self) -> Vec<(String, String)>;

	fn parse_attrs(&self) -> String {
		let mut attrs = String::new();
		for (key, value) in self.fmt_dot() {
			attrs.push_str(&format!(" [ {} = \"{}\" ]", key, value));
		}
		attrs
	}
}

impl FmtDot for Empty {
	fn fmt_dot(&self) -> Vec<(String, String)> {
		Vec::new()
	}
}

pub fn fmt_dot<K, N, E>(nodes: Vec<Node<K, N, E>>) -> String
where
	K: std::fmt::Display + std::fmt::Debug + std::hash::Hash + Eq + Clone,
	N: FmtDot + Clone,
	E: FmtDot + Clone,
{
	let mut s = String::new();
	s.push_str("digraph {\n");
	for node in &nodes {
		let mut node_string = String::new();
		node_string.push_str(&format!("{}", node.key()));
		for attr in node.fmt_dot() {
			node_string.push_str(&format!(" [ {}=\"{}\" ]", attr.0, attr.1));
		}
		s.push_str(&format!("    {}\n", node_string));
	}
	for node in &nodes {
		for edge in node {
			let mut edge_string = String::new();
			edge_string.push_str(&format!("{} -> {}", node.key(), edge.target().key()));
			for attr in edge.fmt_dot() {
				edge_string.push_str(&format!(" [ {}=\"{}\" ]", attr.0, attr.1));
			}
			s.push_str(&format!("    {}\n", edge_string));
		}
	}
	s.push_str("}");
	s
}