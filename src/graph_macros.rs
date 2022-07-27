//! Graph types for directed and undirected graphs.
// use crate::graph::*;
// use crate::ungraph::*;

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating a node.
#[macro_export]
macro_rules! dinode {

	// graph::Node<K, _>
	( $key:expr ) => {
        {
			use gdsl::digraph::node::DiNode;

            DiNode::new($key, Empty)
        }
    };

	// graph::Node<K, N>
    ( $key:expr, $param:expr ) => {
        {
			use gdsl::digraph::node::DiNode;

            DiNode::new($key, $param)
        }
    };

}

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating a node.
#[macro_export]
macro_rules! unnode {

	// graph::Node<K, _>
	( $key:expr ) => {
        {
			use gdsl::ungraph::node::UnNode;

            UnNode::new($key, Empty)
        }
    };

	// graph::Node<K, N>
    ( $key:expr, $param:expr ) => {
        {
			use gdsl::ungraph::node::UnNode;

            UnNode::new($key, $param)
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
			use gdsl::digraph::graph::*;
			use gdsl::digraph::node::DiNode;

            DiNode::connect($s, $t, Empty)
        }
    };

    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use gdsl::digraph::graph::*;
			use gdsl::digraph::node::DiNode;

            DiNode::connect($s, $t, $params)
        }
    };

	( $s:expr, $t:expr ) => {
        {
			use gdsl::ungraph::graph::*;
			use gdsl::ungraph::node::UnNode;

            UnNode::connect($s, $t, Empty)
        }
    };

    ( $s:expr, $t:expr, $params:expr ) => {
        {
			use gdsl::ungraph::graph::*;
			use gdsl::ungraph::node::UnNode;

            UnNode::connect($s, $t, $params)
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
			use gdsl::digraph::graph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, Empty, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = dinode!($NODE);
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
			use gdsl::digraph::graph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, $N, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = dinode!($NODE, $NPARAM);
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
			use gdsl::digraph::graph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, Empty, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = dinode!($NODE);
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
			use gdsl::digraph::graph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = DiGraph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = dinode!($NODE, $NPARAM);
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

	// ==== UNGRAPH ===========================================================

	// UnGraph<K, _, _>
	( ($K:ty) : $(($NODE:expr) : $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use gdsl::ungraph::graph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = UnGraph::<$K, Empty, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = unnode!($NODE);
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
				connect!(&s, &t);
			}
			g
		}
	};

	// UnGraph<K, N, _>
	( ($K:ty, $N:ty) : $(($NODE:expr, $NPARAM:expr) : $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use gdsl::ungraph::graph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = UnGraph::<$K, $N, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = unnode!($NODE, $NPARAM);
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
				connect!(&s, &t);
			}
			g
		}
	};

	// UnGraph<K, _, E>
	( ($K:ty) : [$E:ty] $(($NODE:expr) : $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use gdsl::ungraph::graph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = UnGraph::<$K, Empty, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = unnode!($NODE);
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
				connect!(&s, &t, param);
			}
			g
		}
	};

	// UnGraph<K, N, E>
	( ($K:ty, $N:ty) : [$E:ty] $(($NODE:expr, $NPARAM:expr) : $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use gdsl::ungraph::graph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = UnGraph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = unnode!($NODE, $NPARAM);
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
				connect!(&s, &t, param);
			}
			g
		}
	};
}
