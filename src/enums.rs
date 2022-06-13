use crate::node_trait::*;

pub const OPEN: bool = false;
pub const CLOSED: bool = true;

pub enum Sig {
	Continue,
	Terminate,
}

pub enum Coll {
	Include,
	Exclude,
}

pub enum Move<N: GraphNode> {
	Next,
	Prev,
	Jump(N),
}

#[derive(Clone, Debug)]
pub struct Empty;

impl std::fmt::Display for Empty {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "_")
    }
}
