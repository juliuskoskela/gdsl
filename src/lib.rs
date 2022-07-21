pub mod graph;
pub mod node;
pub mod edge;
pub mod macros;
pub mod tests;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Empty;

impl std::fmt::Display for Empty {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "_")
    }
}
