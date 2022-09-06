use crate::sync_ungraph::node::*;

pub type FilterMap<'a, K, N, E> = &'a dyn Fn(&Node<K, N, E>, &Node<K, N, E>, &E) -> bool;
pub type Filter<'a, K, N, E> = &'a dyn Fn(&Node<K, N, E>, &Node<K, N, E>, &E) -> bool;
pub type Map<'a, K, N, E> = &'a dyn Fn(&Node<K, N, E>, &Node<K, N, E>, &E);

#[derive(Clone)]
pub enum Method<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	NullMethod,
	FilterMap(FilterMap<'a, K, N, E>),
	Filter(Filter<'a, K, N, E>),
	Map(Map<'a, K, N, E>),
}

impl<'a, K, N, E> Method<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn exec(&self, u: &Node<K, N, E>, v: &Node<K, N, E>, e: &E) -> bool {
		match self {
			Method::NullMethod => true,
			Method::Map(f) => {f(u, v, e); true},
			Method::Filter(f) => f(u, v, e),
			Method::FilterMap(f) => f(u, v, e),
		}
	}
}

pub enum Ordering {
	Pre,
	Post,
}
