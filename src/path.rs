use crate::node::*;
use crate::edge::*;

pub struct Path<N: GraphNode> {
	pub edges: Vec<N::Edge>,
}

impl<N: GraphNode> std::ops::Index<usize> for Path<N> {
	type Output = N::Edge;
	fn index(&self, index: usize) -> &Self::Output {
		self.edges.get(index).unwrap()
	}
}

impl <N: GraphNode> Path<N> {
	pub fn new() -> Path<N> { Path { edges: Vec::new() } }
	pub fn node_count(&self) -> usize { self.edges.len() + 1 }
	pub fn edge_count(&self) -> usize { self.edges.len() }

	pub fn from_edge_tree(edge_tree: Vec<N::Edge>) -> Path<N> {
		let mut path: Path<N> = Path::new();
		let w = edge_tree.get(edge_tree.len() - 1).unwrap();
		path.edges.push(w.clone());
		let mut i = 0;
		for edge in edge_tree.iter().rev() {
			let source = path.edges[i].source();
			if edge.target() == source {
				path.edges.push(edge.clone());
				i += 1;
			}
		}
		path.edges.reverse();
		path
	}

	pub fn walk<F>(&self, mut f: F)
		where F: FnMut(&N, &N, <<N as GraphNode>::Edge as GraphEdge<N>>::Params)
	{
		for edges in self.edges.iter() {
			f(edges.source(), edges.target(), edges.load());
		}
	}
}
