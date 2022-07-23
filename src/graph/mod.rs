//! Graph types for directed and undirected graphs.
pub mod digraph;
pub mod bigraph;

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating a node.
#[macro_export]
macro_rules! dinode {

	// digraph::DiNode<K, _>
	( $key:expr ) => {
        {
			use ggi::graph::digraph::DiNode;

            DiNode::new($key, Empty)
        }
    };

	// digraph::DiNode<K, N>
    ( $key:expr, $param:expr ) => {
        {
			use ggi::graph::digraph::*;

            DiNode::new($key, $param)
        }
    };

}

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating a binode.
#[macro_export]
macro_rules! binode {

	// BiNode<K, _>
	( $key:expr ) => {
        {
			use ggi::graph::bigraph::*;

            BiNode::new($key, Empty)
        }
    };

	// BiNode<K, N>
    ( $key:expr, $param:expr ) => {
        {
			use ggi::graph::bigraph::*;

            BiNode::new($key, $param)
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
			use ggi::graph::digraph::*;

            DiNode::connect($s, $t, Empty)
        }
    };

	( $s:expr, $t:expr ) => {
        {
			use ggi::graph::bigraph::*;

            BiNode::connect($s, $t, Empty)
        }
    };

    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use ggi::graph::digraph::*;

            DiNode::connect($s, $t, $params)
        }
    };

	( $s:expr, $t:expr, $params:expr ) => {
        {
			use ggi::graph::bigraph::*;

            BiNode::connect($s, $t, $params)
        }
    };

}

/// Macro for creating either directional or bi-directional (undirected) graphs.
#[macro_export]
macro_rules! graph {

	//  DiGraph<K, _, _>
	( ($K:ty) $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use ggi::graph::digraph::*;

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

	// BiGraph<K, _, _>
	( ($K:ty) $(($NODE:expr) : $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use ggi::graph::bigraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = BiGraph::<$K, Empty, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = binode!($NODE);
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

	// DiGraph<K, N, _>
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) => $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use ggi::graph::digraph::*;

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

	// BiGraph<K, N, _>
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) : $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use ggi::graph::bigraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = BiGraph::<$K, $N, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = binode!($NODE, $NPARAM);
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

	// DiGraph<K, _, E>
	( ($K:ty) => [$E:ty] $(($NODE:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use ggi::graph::digraph::*;

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

	// BiGraph<K, _, E>
	( ($K:ty) : [$E:ty] $(($NODE:expr) : $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use ggi::graph::bigraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = BiGraph::<$K, Empty, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = binode!($NODE);
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

	// DiGraph<K, N, E>
	( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use ggi::graph::digraph::*;

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

	// BiGraph<K, N, E>
	( ($K:ty, $N:ty) : [$E:ty] $(($NODE:expr, $NPARAM:expr) : $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use ggi::graph::bigraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = BiGraph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = binode!($NODE, $NPARAM);
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
