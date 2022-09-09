use crate::ungraph::node::*;

pub type FilterMap<'a, K, N, E> = &'a dyn Fn(&Edge<K, N,E>) -> bool;
pub type Filter<'a, K, N, E> = &'a dyn Fn(&Edge<K, N,E>) -> bool;
pub type Map<'a, K, N, E> = &'a dyn Fn(&Edge<K, N,E>);

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
	pub fn exec(&self, e: &Edge<K, N, E>) -> bool {
		match self {
			Method::NullMethod => true,
			Method::Map(f) => {f(e); true},
			Method::Filter(f) => f(e),
			Method::FilterMap(f) => f(e),
		}
	}
}

pub enum Ordering {
	Pre,
	Post,
}
