use crate::ungraph::node::*;

pub type FilterMap<'a, K, N, E> = &'a mut dyn FnMut(&Edge<K, N, E>) -> bool;
pub type Filter<'a, K, N, E> = &'a mut dyn FnMut(&Edge<K, N, E>) -> bool;
pub type Map<'a, K, N, E> = &'a mut dyn FnMut(&Edge<K, N, E>);
pub type ForEach<'a, K, N, E> = &'a mut dyn FnMut(&Edge<K, N, E>);

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
	ForEach(ForEach<'a, K, N, E>),
}

impl<'a, K, N, E> Method<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	pub fn exec(&mut self, e: &Edge<K, N, E>) -> bool {
		match self {
			Method::NullMethod => true,
			Method::Map(f) => {f(e); true},
			Method::Filter(f) => f(e),
			Method::FilterMap(f) => f(e),
			Method::ForEach(f) => {f(e); true},
		}
	}
}

pub enum Ordering {
	Pre,
	Post,
}
