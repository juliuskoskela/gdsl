use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph::digraph::*;
use graph::core::*;
use graph::examples::*;
use rand::Rng;
use lazy_static::lazy_static;
use std::sync::Arc;

const BIG_NODE_COUNT: usize = 100000;
const MEDIUM_NODE_COUNT: usize = 10000;
const SMALL_NODE_COUNT: usize = 100;
const FLOW_NODE_COUNT: usize = 1000;
const FLOW_NODE_DEGREE: usize = 20;

type IntKeysGraph = Digraph<usize, usize, Empty>;

fn create_graph_flow() -> FlowGraph {
	let mut g = FlowGraph::new();
	for i in 0..FLOW_NODE_COUNT {
		g.insert(i, Empty);
	}
	for i in 0..FLOW_NODE_COUNT {
		for _ in 0..FLOW_NODE_DEGREE {
			connect_flow(&mut g, &i, &rand_range(0, FLOW_NODE_COUNT), rand_range(0, FLOW_NODE_COUNT));
		}
	}
	g
}

fn create_graph(count: usize, average_degree: usize) -> IntKeysGraph {
	let mut g = IntKeysGraph::new();
	for i in 0..count {
		g.insert(i, rand_range(0, 10000000000));
	}
	for i in 0..count {
		let new_degree = rand_range(0, average_degree * 2);
		for _ in 0..new_degree {
			g.connect(&i, &rand_range(0, count), Empty);
		}
	}
	g
}

lazy_static! {
	static ref SMALL_GRAPH: IntKeysGraph = create_graph(SMALL_NODE_COUNT, 3);
	static ref MEDIUM_GRAPH: IntKeysGraph = create_graph(MEDIUM_NODE_COUNT, 5);
	static ref BIG_GRAPH: IntKeysGraph = create_graph(BIG_NODE_COUNT, 10);
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
			g.connect(&i, &rand_range(0, 100), Empty);
		}
	}
}

fn digraph_construction(c: &mut Criterion) {
    c.bench_function("graph construction", |b| b.iter(|| create_graph_speed()));
}

// ============================================================================

fn digraph_breadth_first_search_naked(c: &mut Criterion) {
	fn digraph_bfs() {
		let t = BIG_GRAPH.node(&rand_range(0, BIG_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Empty>> | {
			if *t == e.target() {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		};
		for _ in 0..10 {
			BIG_GRAPH.breadth_first(&rand_range(0, BIG_NODE_COUNT), closure);
		}
	}
	println!("graph node count = {}", BIG_GRAPH.node_count());
	println!("graph edge count = {}", BIG_GRAPH.edge_count());
	println!("graph average degree = {}", BIG_GRAPH.edge_count() as f64 / BIG_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", BIG_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("breadth first search naked", |b| b.iter(|| black_box(digraph_bfs())));
}

fn digraph_breadth_first_sleep_1ms(c: &mut Criterion) {
	fn digraph_bfs() {
		let t = SMALL_GRAPH.node(&rand_range(0, SMALL_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Empty>> | {
			std::thread::sleep(std::time::Duration::from_millis(1));
			if *t == e.target() {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		};
		SMALL_GRAPH.breadth_first(&rand_range(0, SMALL_NODE_COUNT), closure);
	}
	println!("graph node count = {}", SMALL_GRAPH.node_count());
	println!("graph edge count = {}", SMALL_GRAPH.edge_count());
	println!("graph average degree = {}", SMALL_GRAPH.edge_count() as f64 / SMALL_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", SMALL_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("breadth first sleep 1ms", |b| b.iter(|| black_box(digraph_bfs())));
}

fn digraph_breadth_first_count_prime(c: &mut Criterion) {
	fn digraph_bfs() {
		// let t = MEDIUM_GRAPH.node(&rand_range(0, MEDIUM_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Empty>> | {
			let n = e.target().load();
			if primes::is_prime(n as u64) == true {
				e.target().store(n);
			}
			Traverse::Include
			// if *t == e.target() {
			// 	Traverse::Finish
			// } else {
			// 	Traverse::Include
			// }
		};
		for _ in 0..10 {
			MEDIUM_GRAPH.breadth_first(&rand_range(0, MEDIUM_NODE_COUNT), closure);
		}
	}
	println!("graph node count = {}", MEDIUM_GRAPH.node_count());
	println!("graph edge count = {}", MEDIUM_GRAPH.edge_count());
	println!("graph average degree = {}", MEDIUM_GRAPH.edge_count() as f64 / MEDIUM_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", MEDIUM_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("breadth first count prime", |b| b.iter(|| black_box(digraph_bfs())));
}

// ============================================================================

fn digraph_par_breadth_first_search_naked(c: &mut Criterion) {
	fn digraph_bfs() {
		let t = BIG_GRAPH.node(&rand_range(0, BIG_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Empty>> | {
			if *t == e.target() {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		};
		BIG_GRAPH.par_breadth_first(&rand_range(0, BIG_NODE_COUNT), closure);
	}
	println!("graph node count = {}", BIG_GRAPH.node_count());
	println!("graph edge count = {}", BIG_GRAPH.edge_count());
	println!("graph average degree = {}", BIG_GRAPH.edge_count() as f64 / BIG_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", BIG_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("parallel breadth first search naked", |b| b.iter(|| black_box(digraph_bfs())));
}

fn digraph_par_breadth_first_sleep_1ms(c: &mut Criterion) {
	fn digraph_bfs() {
		let t = SMALL_GRAPH.node(&rand_range(0, SMALL_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Empty>> | {
			std::thread::sleep(std::time::Duration::from_millis(1));
			if *t == e.target() {
				Traverse::Finish
			} else {
				Traverse::Include
			}
		};
		SMALL_GRAPH.par_breadth_first(&rand_range(0, SMALL_NODE_COUNT), closure);
	}
	println!("graph node count = {}", SMALL_GRAPH.node_count());
	println!("graph edge count = {}", SMALL_GRAPH.edge_count());
	println!("graph average degree = {}", SMALL_GRAPH.edge_count() as f64 / SMALL_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", SMALL_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("parallel breadth first sleep 1ms", |b| b.iter(|| black_box(digraph_bfs())));
}

fn digraph_par_breadth_first_count_prime(c: &mut Criterion) {
	fn digraph_bfs() {
		// let t = MEDIUM_GRAPH.node(&rand_range(0, MEDIUM_NODE_COUNT)).unwrap();
		let closure = | e: &Arc<Edge<usize, usize, Empty>> | {
			let n = e.target().load();
			if primes::is_prime(n as u64) == true {
				e.target().store(n);
			}
			Traverse::Include
			// if *t == e.target() {
			// 	Traverse::Finish
			// } else {
			// 	Traverse::Include
			// }
		};
		for _ in 0..10 {
			MEDIUM_GRAPH.par_breadth_first(&rand_range(0, MEDIUM_NODE_COUNT), closure);
		}
	}
	println!("graph node count = {}", MEDIUM_GRAPH.node_count());
	println!("graph edge count = {}", MEDIUM_GRAPH.edge_count());
	println!("graph average degree = {}", MEDIUM_GRAPH.edge_count() as f64 / MEDIUM_GRAPH.node_count() as f64);
	println!("sizeof graph = {} Mb", MEDIUM_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("parallel breadth first count prime", |b| b.iter(|| black_box(digraph_bfs())));
}

// ============================================================================


fn digraph_find_shortest_path(c: &mut Criterion) {
	// println!("constructing graph of size = {} Mb", ((BIG_NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (BIG_NODE_COUNT * SIMPLE_NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_sp() {
		let t = BIG_GRAPH.node(&rand_range(0, BIG_NODE_COUNT)).unwrap();
		let res = BIG_GRAPH.breadth_first(&rand_range(0, BIG_NODE_COUNT), |e| if *t == e.target() { Traverse::Finish } else { Traverse::Include});
		match res {
			Some(r) => { backtrack_edges(&r); }
			None => {}
		}
	}
	println!("graph node count = {}", BIG_GRAPH.node_count());
	println!("graph edge count = {}", BIG_GRAPH.edge_count());
	println!("graph average degree = {}", BIG_GRAPH.edge_count() / BIG_GRAPH.node_count());
	println!("sizeof graph = {} Mb", BIG_GRAPH.bytesize() as f64 / 1000_000.0);
    c.bench_function("find shortest path", |b| b.iter(|| black_box(digraph_sp())));
}

fn digraph_par_find_shortest_path(c: &mut Criterion) {
	// println!("constructing graph of size = {} Mb", ((BIG_NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (BIG_NODE_COUNT * SIMPLE_NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_sp() {
		let t = BIG_GRAPH.node(&rand_range(0, BIG_NODE_COUNT)).unwrap();
		let res = BIG_GRAPH.par_breadth_first(&rand_range(0, BIG_NODE_COUNT), |e| if *t == e.target() { Traverse::Finish } else { Traverse::Include});
		match res {
			Some(r) => { backtrack_edges(&r); }
			None => {}
		}
	}
	println!("graph node count = {}", BIG_GRAPH.node_count());
	println!("graph edge count = {}", BIG_GRAPH.edge_count());
	println!("graph average degree = {}", BIG_GRAPH.edge_count() / BIG_GRAPH.node_count());
	println!("sizeof graph = {} Mb", BIG_GRAPH.bytesize() as f64 / 1000_000.0);
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

	digraph_breadth_first_search_naked,
	digraph_par_breadth_first_search_naked,
	digraph_breadth_first_sleep_1ms,
	digraph_par_breadth_first_sleep_1ms,
	digraph_breadth_first_count_prime,
	digraph_par_breadth_first_count_prime,
	digraph_max_flow,
	digraph_par_max_flow,
	digraph_find_shortest_path,
	digraph_par_find_shortest_path,
	digraph_construction
);

criterion_main!(benchmarks);
