//! # Macros
use crate::node::GraphNode;
use std::collections::BTreeMap;

#[macro_export]
macro_rules! node {
	( $key:expr ) => {
        {
			use crate::dinode::*;
			use crate::enums::*;
            Node::new($key, Empty)
        }
    };
    ( $key:expr, $param:expr ) => {
        {
			use crate::dinode::*;
            Node::new($key, $param)
        }
    };
}

#[macro_export]
macro_rules! connect {
	( $s:expr => $t:expr ) => {
        {
			use crate::dinode::*;
			use crate::enums::*;
            Node::connect($s, $t, Empty)
        }
    };
    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use crate::dinode::*;
			use crate::node::GraphNode;
            Node::connect($s, $t, $params)
        }
    };
}

pub fn nodes_exist<K, N>(map: BTreeMap<K, N>, s: K, t: K) -> bool
where
	N: GraphNode + std::fmt::Display,
	K: std::fmt::Debug + std::fmt::Display + PartialEq + Ord,
{
	if !map.contains_key(&s) && !map.contains_key(&t) {
		panic!("Check your macro invocation: {} and {} are not in the graph", s, t);
	} else if !map.contains_key(&s) {
		panic!("Check your macro invocation: {} is not in the graph", s);
	} else if !map.contains_key(&t) {
		panic!("Check your macro invocation: {} is not in the graph", t);
	} else {
		true
	}
}

#[macro_export]
macro_rules! graph {

	// (Key)
	( ($K:ty) $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use std::collections::BTreeMap;
			use crate::dinode::*;
			use crate::enums::*;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K)>::new();
			let mut map = BTreeMap::<$K, Node<$K, Empty, Empty>>::new();
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
			use std::collections::BTreeMap;
			use crate::dinode::*;
			use crate::enums::*;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K)>::new();
			let mut map = BTreeMap::<$K, Node<$K, $N, Empty>>::new();
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
			use std::collections::BTreeMap;
			use crate::dinode::*;
			use crate::enums::*;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = BTreeMap::<$K, Node<$K, Empty, $E>>::new();
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
			use std::collections::BTreeMap;
			use crate::dinode::*;
			use crate::macros::nodes_exist;

			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = BTreeMap::<$K, Node<$K, $N, $E>>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE, $NPARAM);
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
}
