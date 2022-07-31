//! Graph types for directed and undirected graphs.
// use crate::graph::*;
// use crate::ungraph::*;

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating a node.
#[macro_export]
macro_rules! node {

	// graph::Node<K, _>
	( $key:expr ) => {
        {
			use gdsl::digraph::DiNode;

            DiNode::new($key, ())
        }
    };

	// graph::Node<K, N>
    ( $key:expr, $param:expr ) => {
        {
			use gdsl::digraph::DiNode;

            DiNode::new($key, $param)
        }
    };

}

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for connecting two nodes.
#[macro_export]
macro_rules! connect {

	( $s:expr => $t:expr ) => {
        {
			use gdsl::digraph::*;

            DiNode::connect($s, $t, ())
        }
    };

    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use gdsl::digraph::*;

            DiNode::connect($s, $t, $params)
        }
    };
}

/// Macro for creating either directional or bi-directional (undirected) graphs.
#[macro_export]
macro_rules! graph {

	// ==== DIGRAPH ===========================================================

	// DiGraph<K, _, _>
	( ($K:ty) => $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use gdsl::digraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, (), ()>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE);
				g.insert(n);
			)*
			for (s, t) in edges {
				if !g.contains(&s) || !g.contains(&t) {
					if !g.contains(&s) {
						panic!("Check your macro invocation: \"{}\" is not in the graph", s);
					} else {
						panic!("Check your macro invocation: \"{}\" is not in the graph", t);
					}
				}
				let s = g.get(&s).unwrap();
				let t = g.get(&t).unwrap();
				connect!(&s => &t);
			}
			g
		}
	};

	// DiGraph<K, N, _>
	( ($K:ty, $N:ty) => $(($NODE:expr, $NPARAM:expr) => $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use gdsl::digraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, $N, ()>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				g.insert(n);
			)*
			for (s, t) in edges {
				if !g.contains(&s) || !g.contains(&t) {
					if !g.contains(&s) {
						panic!("Check your macro invocation: \"{}\" is not in the graph", s);
					} else {
						panic!("Check your macro invocation: \"{}\" is not in the graph", t);
					}
				}
				let s = g.get(&s).unwrap();
				let t = g.get(&t).unwrap();
				connect!(&s => &t);
			}
			g
		}
	};

	// DiGraph<K, _, E>
	( ($K:ty) => [$E:ty] $(($NODE:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use gdsl::digraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, (), $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE);
				g.insert(n);
			)*
			for (s, t, param) in edges {
				if !g.contains(&s) || !g.contains(&t) {
					if !g.contains(&s) {
						panic!("Check your macro invocation: \"{}\" is not in the graph", s);
					} else {
						panic!("Check your macro invocation: \"{}\" is not in the graph", t);
					}
				}
				let s = g.get(&s).unwrap();
				let t = g.get(&t).unwrap();
				connect!(&s => &t, param);
			}
			g
		}
	};

	// DiGraph<K, N, E>
	( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use gdsl::digraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = node!($NODE, $NPARAM);
				g.insert(n);
			)*
			for (s, t, param) in edges {
				if !g.contains(&s) || !g.contains(&t) {
					if !g.contains(&s) {
						panic!("Check your macro invocation: \"{}\" is not in the graph", s);
					} else {
						panic!("Check your macro invocation: \"{}\" is not in the graph", t);
					}
				}
				let s = g.get(&s).unwrap();
				let t = g.get(&t).unwrap();
				connect!(&s => &t, param);
			}
			g
		}
	};
}
