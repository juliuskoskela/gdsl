use crate::graph::*;

#[macro_export]
macro_rules! node {
	( $key:expr ) => {
        {
            GraphNode::new($key, None)
        }
    };
    ( $key:expr, $param:expr ) => {
        {
            GraphNode::new($key, Some($param))
        }
    };
}

#[macro_export]
macro_rules! graph {

	// s => [e1, e2]
	( ($K:ty, $N:ty, $E:ty), $($NODE:expr $( => [ $( $EDGE:expr),*] )? )* )
	=> {
        {
			use std::collections::BTreeMap;
			let mut edges = Vec::<($K, $K)>::new();
            let mut map = BTreeMap::<$K, GraphNode<$K, $N, $E>>::new();
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
				if map.contains_key(&s) && map.contains_key(&t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t);
				}
			}
            map
        }
    };
}

#[macro_export]
macro_rules! graph_with_params {
// (s, T) => [(e1, T), (e2, T)]
( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use std::collections::BTreeMap;
			let mut edges = Vec::<($K, $K, $E)>::new();
			let mut map = BTreeMap::<$K, GraphNode<$K, $N, $E>>::new();
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
				if map.contains_key(&s) && map.contains_key(&t) {
					let s = map.get(&s).unwrap();
					let t = map.get(&t).unwrap();
					connect!(s => t, Some(param));
				}
			}
			map
		}
	};
}

#[macro_export]
macro_rules! connect {
	( $s:expr => $t:expr ) => {
        {
            Node::connect($s, $t, None)
        }
    };
    ( $s:expr => $t:expr, $params:expr ) => {
        {
            Node::connect($s, $t, $params)
        }
    };
}