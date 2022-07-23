//! Graph types for directed and undirected async graphs.
pub mod digraph;
pub mod bigraph;

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating an async_node.
#[macro_export]
macro_rules! async_dinode {

	// digraph::AsyncDiNode<K>
	( $key:expr ) => {
        {
			use crate::graph_async::digraph::*;

            AsyncDiNode::new($key, Empty)
        }
    };

	// digraph::AsyncDiNode<K, V>
    ( $key:expr, $param:expr ) => {
        {
			use crate::graph_async::digraph::*;

            AsyncDiNode::new($key, $param)
        }
    };

}

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating an async_binode.
#[macro_export]
macro_rules! async_binode {

	// AsyncBiNode<K, _>
	( $key:expr ) => {
        {
			use crate::graph_async::bigraph::*;

            AsyncBiNode::new($key, Empty)
        }
    };

	// AsyncBiNode<K, V>
    ( $key:expr, $param:expr ) => {
        {
			use crate::graph_async::bigraph::*;

            AsyncBiNode::new($key, $param)
        }
    };

}

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for connecting two async nodes.
#[macro_export]
macro_rules! async_connect {

	( $s:expr => $t:expr ) => {
        {
			use crate::graph_async::digraph::*;

            AsyncDiNode::connect($s, $t, Empty)
        }
    };

	( $s:expr, $t:expr ) => {
        {
			use crate::graph_async::bigraph::*;

            AsyncBiNode::connect($s, $t, Empty)
        }
    };

    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use crate::graph_async::digraph::*;

            AsyncDiNode::connect($s, $t, $params)
        }
    };

	( $s:expr, $t:expr, $params:expr ) => {
        {
			use crate::graph_async::bigraph::*;

            AsyncBiNode::connect($s, $t, $params)
        }
    };

}

///////////////////////////////////////////////////////////////////////////////
///
/// Macro for creating either directional or bi-directional (undirected) async
/// graphs.
#[macro_export]
macro_rules! async_graph {

	//  AsyncDiGraph<K, _, _>
	( ($K:ty) $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use crate::graph_async::digraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = AsyncDiGraph::<$K, Empty, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = async_dinode!($NODE);
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

	// AsyncBiGraph<K, _, _>
	( ($K:ty) $(($NODE:expr) : $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use crate::graph_async::bigraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = AsyncBiGraph::<$K, Empty, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = async_binode!($NODE);
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

	// AsyncDiGraph<K, N, _>
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) => $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use crate::graph_async::digraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = AsyncDiGraph::<$K, $N, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = async_dinode!($NODE, $NPARAM);
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

	// AsyncBiGraph<K, N, _>
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) : $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use crate::graph_async::bigraph::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = AsyncBiGraph::<$K, $N, Empty>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = async_binode!($NODE, $NPARAM);
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

	// AsyncDiGraph<K, _, E>
	( ($K:ty) => [$E:ty] $(($NODE:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::graph_async::digraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = AsyncDiGraph::<$K, Empty, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = async_dinode!($NODE);
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

	// AsyncBiGraph<K, _, E>
	( ($K:ty) : [$E:ty] $(($NODE:expr) : $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::graph_async::bigraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = AsyncBiGraph::<$K, Empty, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = async_binode!($NODE);
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

	// AsyncDiGraph<K, N, E>
	( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::graph_async::digraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = AsyncDiGraph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = async_dinode!($NODE, $NPARAM);
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

	// AsyncBiGraph<K, N, E>
	( ($K:ty, $N:ty) : [$E:ty] $(($NODE:expr, $NPARAM:expr) : $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use crate::graph_async::bigraph::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = AsyncBiGraph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = async_binode!($NODE, $NPARAM);
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
