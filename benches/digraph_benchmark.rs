use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph::digraph::*;
use graph::global::*;
use graph::edge::*;
use graph::examples::*;
use graph::global::Traverse::{Include, Finish};
use rand::Rng;
use lazy_static::lazy_static;
use std::sync::Arc;

const SIMPLE_NODE_COUNT: usize = 100000;
const SIMPLE_NODE_DEGREE: usize = 100;
const FLOW_NODE_COUNT: usize = 1000;
const FLOW_NODE_DEGREE: usize = 20;

type IntKeysGraph = Digraph<usize, usize, Null>;

fn create_graph_flow() -> FlowGraph {
	let mut g = FlowGraph::new();
	for i in 0..FLOW_NODE_COUNT {
		g.insert(i, Null);
	}
	for i in 0..FLOW_NODE_COUNT {
		for _ in 0..FLOW_NODE_DEGREE {
			connect_flow(&mut g, &i, &rand_range(0, FLOW_NODE_COUNT), rand_range(0, FLOW_NODE_COUNT));
		}
	}
	g
}

fn create_graph_simple() -> IntKeysGraph {
	let mut g = IntKeysGraph::new();
	for i in 0..SIMPLE_NODE_COUNT {
		g.insert(i, rand_range(0, 10000000000));
	}
	for i in 0..SIMPLE_NODE_COUNT {
		for _ in 0..SIMPLE_NODE_DEGREE {
			g.connect(&i, &rand_range(0, SIMPLE_NODE_COUNT), Null);
		}
	}
	g
}

lazy_static! {
	static ref SIMPLE_GRAPH: IntKeysGraph = create_graph_simple();
	static ref FLOW_GRAPH: FlowGraph = create_graph_flow();
}

fn rand_range(start: usize, end: usize) -> usize {
	let mut rng = rand::thread_rng();
	rng.gen_range(start..end)
}

fn create_graph_speed() {
	let mut g = IntKeysGraph::new();
	for i in 0..1000 {
		g.insert(i, rand_range(0, 10000000000));
	}
	for i in 0..1000 {
		for _ in 0..100 {
			g.connect(&i, &rand_range(0, 100), Null);
		}
	}
}

fn digraph_construction(c: &mut Criterion) {
    c.bench_function("graph construction", |b| b.iter(|| create_graph_speed()));
}

fn digraph_breadth_first_search(c: &mut Criterion) {
	// println!("constructing graph of size = {} Mb", ((SIMPLE_NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (SIMPLE_NODE_COUNT * SIMPLE_NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_bfs() {
		let t = SIMPLE_GRAPH.node(&rand_range(0, SIMPLE_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Null>> | {

			let mut n = e.target().load();

			if primes::is_prime(n as u64) == true {
				n = n / 2;
				e.target().store(n);
			}
			if *t == e.target() {
				Finish
			} else {
				Include
			}
		};
		SIMPLE_GRAPH.breadth_first(&rand_range(0, SIMPLE_NODE_COUNT), closure);
	}
	println!("graph node count = {}", SIMPLE_GRAPH.node_count());
	println!("graph edge count = {}", SIMPLE_GRAPH.edge_count());
	println!("graph average degree = {}", SIMPLE_GRAPH.edge_count() as f64 / SIMPLE_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", SIMPLE_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("breadth first search", |b| b.iter(|| black_box(digraph_bfs())));
}

fn digraph_par_breadth_first_search(c: &mut Criterion) {
	// println!("constructing graph of size = {} Mb", ((SIMPLE_NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (SIMPLE_NODE_COUNT * SIMPLE_NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_par_bfs() {
		let t = SIMPLE_GRAPH.node(&rand_range(0, SIMPLE_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Null>> | {

			let mut n = e.target().load();

			if primes::is_prime(n as u64) == true {
				n = n / 2;
				e.target().store(n);
			}
			if *t == e.target() {
				Finish
			} else {
				Include
			}
		};
		SIMPLE_GRAPH.par_breadth_first(&rand_range(0, SIMPLE_NODE_COUNT), closure);
	}
	println!("graph node count = {}", SIMPLE_GRAPH.node_count());
	println!("graph edge count = {}", SIMPLE_GRAPH.edge_count());
	println!("graph average degree = {}", SIMPLE_GRAPH.edge_count() as f64 / SIMPLE_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", SIMPLE_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("parallel breadth first search", |b| b.iter(|| black_box(digraph_par_bfs())));
}

fn digraph_find_shortest_path(c: &mut Criterion) {
	// println!("constructing graph of size = {} Mb", ((SIMPLE_NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (SIMPLE_NODE_COUNT * SIMPLE_NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_sp() {
		let t = SIMPLE_GRAPH.node(&rand_range(0, SIMPLE_NODE_COUNT)).unwrap();
		let res = SIMPLE_GRAPH.breadth_first(&rand_range(0, SIMPLE_NODE_COUNT), |e| if *t == e.target() { Finish } else { Include});
		match res {
			Some(r) => { r.backtrack(); }
			None => {}
		}
	}
	println!("graph node count = {}", SIMPLE_GRAPH.node_count());
	println!("graph edge count = {}", SIMPLE_GRAPH.edge_count());
	println!("graph average degree = {}", SIMPLE_GRAPH.edge_count() / SIMPLE_GRAPH.node_count());
	println!("sizeof graph = {} Mb", SIMPLE_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("find shortest path", |b| b.iter(|| black_box(digraph_sp())));
}

fn digraph_par_find_shortest_path(c: &mut Criterion) {
	// println!("constructing graph of size = {} Mb", ((SIMPLE_NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (SIMPLE_NODE_COUNT * SIMPLE_NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_sp() {
		let t = SIMPLE_GRAPH.node(&rand_range(0, SIMPLE_NODE_COUNT)).unwrap();
		let res = SIMPLE_GRAPH.par_breadth_first(&rand_range(0, SIMPLE_NODE_COUNT), |e| if *t == e.target() { Finish } else { Include});
		match res {
			Some(r) => { r.backtrack(); }
			None => {}
		}
	}
	println!("graph node count = {}", SIMPLE_GRAPH.node_count());
	println!("graph edge count = {}", SIMPLE_GRAPH.edge_count());
	println!("graph average degree = {}", SIMPLE_GRAPH.edge_count() / SIMPLE_GRAPH.node_count());
	println!("sizeof graph = {} Mb", SIMPLE_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("parallel find shortest path", |b| b.iter(|| black_box(digraph_sp())));
}

fn digraph_max_flow(c: &mut Criterion) {
	fn digraph_mf() {
		let g = create_graph_flow();
		maximum_flow_edmonds_karp(&g, rand_range(0, FLOW_NODE_COUNT), rand_range(0, FLOW_NODE_COUNT));
	}
    c.bench_function("maximum flow edmonds karp", |b| b.iter(|| black_box(digraph_mf())));
}

fn digraph_par_max_flow(c: &mut Criterion) {
	fn digraph_mf() {
		let g = create_graph_flow();
		parallel_maximum_flow_edmonds_karp(&g, rand_range(0, FLOW_NODE_COUNT), rand_range(0, FLOW_NODE_COUNT));
	}
    c.bench_function("parallel maximum flow edmonds karp", |b| b.iter(|| black_box(digraph_mf())));
}

criterion_group!(
	benchmarks,

	digraph_breadth_first_search,
	digraph_par_breadth_first_search,
	digraph_max_flow,
	digraph_par_max_flow,
	digraph_find_shortest_path,
	digraph_par_find_shortest_path,
	digraph_construction
);

criterion_main!(benchmarks);
