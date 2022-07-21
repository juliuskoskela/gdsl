use crate::graph::Graph;
use std::hash::Hash;

pub fn nodes_exist<K, N, E>(graph: &Graph<K, N, E>, s: K, t: K) -> bool
where
	K: std::fmt::Debug + std::fmt::Display + Hash + Eq + Clone + PartialEq,
	N: std::fmt::Debug + Clone,
	E: Clone,
{
	if !graph.contains(&s) && !graph.contains(&t) {
		panic!("Check your macro invocation: {} and {} are not in the graph", s, t);
	} else if !graph.contains(&s) {
		panic!("Check your macro invocation: {} is not in the graph", s);
	} else if !graph.contains(&t) {
		panic!("Check your macro invocation: {} is not in the graph", t);
	} else {
		true
	}
}

#[macro_export]
macro_rules! node {
	( $key:expr ) => {
        {
			use crate::*;
			use crate::node::*;
            Node::new($key, Empty)
        }
    };
    ( $key:expr, $param:expr ) => {
        {
			use crate::node::*;
            Node::new($key, $param)
        }
    };
}

#[macro_export]
macro_rules! connect {
	( $s:expr => $t:expr ) => {
        {
			use crate::node::*;
			use crate::*;
            Node::connect($s, $t, Empty)
        }
    };
    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use crate::node::*;
            Node::connect($s, $t, $params)
        }
    };
}

#[macro_export]
macro_rules! graph {

	// (Key)
	( ($K:ty) $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use crate::graph::Graph;
			use crate::*;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut graph = Graph::<$K, Empty, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE);
				graph.insert(n);
			)*
			for (s, t) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t);
				}
			}
			graph
		}
	};

	// (Key, Node)
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) => $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use crate::graph::Graph;
			use crate::*;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut graph = Graph::<$K, $N, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				graph.insert(n);
			)*
			for (s, t) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t);
				}
			}
			graph
		}
	};

	// (Key) => [Edge]
	( ($K:ty) => [$E:ty] $(($NODE:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::graph::Graph;
			use crate::*;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut graph = Graph::<$K, Empty, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE);
				graph.insert(n);
			)*
			for (s, t, param) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t, param);
				}
			}
			graph
		}
	};

	// (Key, Node) -> [Edge]
	( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::*;
			use crate::graph::Graph;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut graph = Graph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				graph.insert(n);
			)*
			for (s, t, param) in edges {
				if nodes_exist(&graph, s, t) {
					let s = graph.get(&s).unwrap();
					let t = graph.get(&t).unwrap();
					connect!(&s => &t, param);
				}
			}
			graph
		}
	};
}
