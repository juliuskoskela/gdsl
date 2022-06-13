use std::collections::{HashMap};
use std::fmt::Display;
use std::hash::Hash;
use crate::graph::*;

// pub enum Attr {
// 	Color(u32),
// 	Size(u32),
// 	Comment(String),
// 	FontColor(u32),
// 	FontName(String),
// 	FontSize(u32),
// 	Label(String),
// 	Weight(u32),
// }

pub trait DotNodeAttr {
	fn to_attr(str: HashMap<String, String>) -> Self;
	fn from_attr(self) -> HashMap<String, String>;
}

pub fn to_dot_directed<K, N, E>(graph: GraphMap<K, N, E>)
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + Hash + PartialEq + Display
{
	println!("digraph G {{");
	for (_, node) in graph {
		let attr = "labels";
		println!("\t{} [{}];", node.id(), attr);
		for edge in node.outbound().read().iter() {
			println!("\t{} -> {} [label=\"{}\"];", edge.source().id(), edge.target().id(), "edge attr");
		}
	}
	println!("}}");
}

pub fn to_dot_undirected<K, N, E>(graph: GraphMap<K, N, E>, )
where
	E: Clone + Sync + Send,
	N: Clone + Sync + Send,
	K: Clone + Sync + Send + Hash + PartialEq + Display
{
	println!("graph G {{");
	for (_, node) in graph {
		let attr = "labels";
		println!("\t{} [label=\"{}\"];", node.id(), attr);
		for edge in node.inbound().read().iter() {
			println!("\t{} -- {} [label=\"{}\"];", edge.target().id(), edge.source().id(), "edge attr");
		}
		for edge in node.outbound().read().iter() {
			println!("\t{} -- {} [label=\"{}\"];", edge.source().id(), edge.target().id(), "edge attr");
		}
	}
	println!("}}");
}
