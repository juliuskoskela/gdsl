//! # Dot Formatting
//!
//! This module provides the `FmtDot` trait, which is used to represent a DiGraph in the DOT language.
use crate::*;
use crate::graph::digraph::*;

pub trait FmtDot {
	fn attributes(&self) -> Vec<(String, String)>;
}

impl FmtDot for Empty {
	fn attributes(&self) -> Vec<(String, String)> {
		Vec::new()
	}
}

pub fn fmt_dot<K, N, E>(nodes: Vec<DiNode<K, N, E>>) -> String
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
		for attr in node.attributes() {
			node_string.push_str(&format!(" [ {}=\"{}\" ]", attr.0, attr.1));
		}
		s.push_str(&format!("    {}\n", node_string));
	}
	for node in &nodes {
		for edge in node {
			let mut edge_string = String::new();
			edge_string.push_str(&format!("{} -> {}", node.key(), edge.target().key()));
			for attr in edge.attributes() {
				edge_string.push_str(&format!(" [ {}=\"{}\" ]", attr.0, attr.1));
			}
			s.push_str(&format!("    {}\n", edge_string));
		}
	}
	s.push_str("}");
	s
}
