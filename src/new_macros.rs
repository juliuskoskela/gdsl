use crate::new_node::Graph;
use std::hash::Hash;

pub fn nodes_exist<K, N, E>(graph: &Graph<K, N, E>, s: K, t: K) -> bool
where
	K: std::fmt::Debug + std::fmt::Display + Hash + Eq + Clone + PartialEq,
	N: std::fmt::Debug + Clone,
	E: std::fmt::Debug + Clone,
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
			use crate::enums::*;
			use crate::new_node::Node;
            Node::new($key, Empty)
        }
    };
    ( $key:expr, $param:expr ) => {
        {
			use crate::new_node::Node;
            Node::new($key, $param)
        }
    };
}

#[macro_export]
macro_rules! connect {
	( $s:expr => $t:expr ) => {
        {
			use crate::new_node::Node;
			use crate::enums::*;
            Node::connect($s, $t, Empty)
        }
    };
    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use crate::new_node::Node;
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
			use crate::new_node::Graph;
			use crate::new_node::Node;
			use crate::enums::*;
			use crate::new_macros::nodes_exist;

			let mut edges = Vec::<($K, $K)>::new();
			let mut map = Graph::<$K, Node<$K, Empty, Empty>>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE);
				map.insert(n.id().clone(), n);
			)*
			for (s, t) in edges {
				if nodes_exist(map.clone(), s, t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t);
				}
			}
			map
		}
	};

	// (Key, Node)
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr) ),*] )? )* )
	=> {
		{
			use crate::new_node::Graph;
			use crate::new_node::Node;
			use crate::enums::*;
			use crate::new_macros::nodes_exist;

			let mut edges = Vec::<($K, $K)>::new();
			let mut map = Graph::<$K, Node<$K, $N, Empty>>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				map.insert(n.id().clone(), n);
			)*
			for (s, t) in edges {
				if nodes_exist(map.clone(), s, t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t);
				}
			}
			map
		}
	};

	// (Key) => [Edge]
	( ($K:ty) => [$E:ty] $(($NODE:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::new_node::Graph;
			use crate::new_node::Node;
			use crate::enums::*;
			use crate::new_macros::nodes_exist;

			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = Graph::<$K, Node<$K, Empty, $E>>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE);
				map.insert(n.id().clone(), n);
			)*
			for (s, t, param) in edges {
				if nodes_exist(map.clone(), s, t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t, param);
				}
			}
			map
		}
	};

	// (Key, Node) -> [Edge]
	( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::*;
			use crate::new_node::Graph;
			use crate::new_macros::nodes_exist;

			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = Graph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				map.insert(n);
			)*
			for (s, t, param) in edges {
				if nodes_exist(&map, s, t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(&s => &t, param);
				}
			}
			map
		}
	};
}
