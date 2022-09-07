//! Graph Macros

/// Macro for creating a node.
#[macro_export]
macro_rules! sync_ungraph_node {

	// graph::Node<K, _>
	( $key:expr ) => {
        {
			use gdsl::sync_ungraph::*;

            Node::new($key, ())
        }
    };

	// graph::Node<K, N>
    ( $key:expr, $param:expr ) => {
        {
			use gdsl::sync_ungraph::*;

            Node::new($key, $param)
        }
    };

}

/// Macro for connecting two nodes.
#[macro_export]
macro_rules! sync_ungraph_connect {

	( $s:expr => $t:expr ) => {
        {
			use gdsl::sync_ungraph::*;

            Node::connect($s, $t, ())
        }
    };

    ( $s:expr => $t:expr, $params:expr ) => {
        {
			use gdsl::sync_ungraph::*;

            Node::connect($s, $t, $params)
        }
    };
}

/// Macro for creating a graph.
#[macro_export]
macro_rules! sync_ungraph {

	()
	=> {
		{
			use gdsl::sync_ungraph::*;

			Graph::<usize, (), ()>::new()
		}
	};

	// Graph<K, _, _>
	( ($K:ty) $(($NODE:expr) => $( [ $( $EDGE:expr),*] )? )* )
	=> {
		{
			use gdsl::sync_ungraph::*;
			use gdsl::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = Graph::<$K, (), ()>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = sync_ungraph_node!($NODE);
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
				sync_ungraph_connect!(&s => &t);
			}
			g
		}
	};

	// Graph<K, N, _>
	( ($K:ty, $N:ty) $(($NODE:expr, $NPARAM:expr) => $( [$(  $EDGE:expr) ,*] )? )* )
	=> {
		{
			use gdsl::sync_ungraph::*;
			use gdsl::*;

			let mut edges = Vec::<($K, $K)>::new();
			edges.clear();
			let mut g = Graph::<$K, $N, ()>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE));
					)*
				)?
				let n = sync_ungraph_node!($NODE, $NPARAM);
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
				sync_ungraph_connect!(&s => &t);
			}
			g
		}
	};

	// Graph<K, _, E>
	( ($K:ty) => [$E:ty] $(($NODE:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use gdsl::sync_ungraph::*;
			use gdsl::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = Graph::<$K, (), $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = sync_ungraph_node!($NODE);
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
				sync_ungraph_connect!(&s => &t, param);
			}
			g
		}
	};

	// Graph<K, N, E>
	( ($K:ty, $N:ty) => [$E:ty] $(($NODE:expr, $NPARAM:expr) => $( [$( ( $EDGE:expr, $EPARAM:expr) ),*] )? )* )
	=> {
		{
			use gdsl::sync_ungraph::*;
			use gdsl::*;

			let mut edges = Vec::<($K, $K, $E)>::new();
			edges.clear();
			let mut g = Graph::<$K, $N, $E>::new();
			$(
				$(
					$(
						edges.push(($NODE, $EDGE, $EPARAM));
					)*
				)?
				let n = sync_ungraph_node!($NODE, $NPARAM);
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
				sync_ungraph_connect!(&s => &t, param);
			}
			g
		}
	};
}
