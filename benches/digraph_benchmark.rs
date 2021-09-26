use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph::digraph::*;
use graph::node::Node;
use graph::edge::Edge;
use graph::node::Traverse::Collect;
use rand::Rng;

const NODE_COUNT: usize = 100000;
const NODE_DEGREE: usize = 100;

type IntKeysGraph = Digraph<usize, usize, usize>;

fn rand_range(start: usize, end: usize) -> usize {
	let mut rng = rand::thread_rng();
	rng.gen_range(start..end)
}

fn create_graph() -> IntKeysGraph {
	let mut g = IntKeysGraph::new();
	for i in 0..NODE_COUNT {
		g.insert(i, 0);
	}
	for i in 0..NODE_COUNT {
		for _ in 0..NODE_DEGREE {
			g.connect(&i, &rand_range(0, NODE_COUNT), 0);
		}
	}
	g
}

fn create_graph_speed() {
	let mut g = IntKeysGraph::new();
	for i in 0..1000 {
		g.insert(i, 0);
	}
	for i in 0..1000 {
		for _ in 0..100 {
			g.connect(&i, &rand_range(0, 100), 0);
		}
	}
}

fn digraph_breadth_construction(c: &mut Criterion) {
    c.bench_function("graph construction", |b| b.iter(|| create_graph_speed()));
}

fn digraph_breadth_first_search(c: &mut Criterion) {
	println!("constructing graph of size = {} Mb", ((NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (NODE_COUNT * NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_bfs(g: &IntKeysGraph) {
		g.breadth_first(&rand_range(0, NODE_COUNT), &rand_range(0, NODE_COUNT),
	|_| { Collect });
	}
	let g = create_graph();
	println!("graph node count = {}", g.node_count());
	println!("graph edge count = {}", g.edge_count());
	println!("graph average degree = {}", g.edge_count() as f64 / g.node_count() as f64);
	println!("sizeof graph = {} Mb", g.bytesize() as f64 / 1000_000.0);
    c.bench_function("breadth first search", |b| b.iter(|| black_box(digraph_bfs(&g))));
}

fn digraph_find_shortest_path(c: &mut Criterion) {
	println!("constructing graph of size = {} Mb", ((NODE_COUNT * std::mem::size_of::<Node<usize, usize, usize>>()) + (NODE_COUNT * NODE_DEGREE * std::mem::size_of::<Edge<usize, usize, usize>>())) / 1000_000);
	fn digraph_sp(g: &IntKeysGraph) {
		let res = g.breadth_first(&rand_range(0, NODE_COUNT), &rand_range(0, NODE_COUNT), |_| { Collect });
		match res {
			Some(r) => { r.backtrack(); }
			None => {}
		}
	}
	let g = create_graph();
	println!("graph node count = {}", g.node_count());
	println!("graph edge count = {}", g.edge_count());
	println!("graph average degree = {}", g.edge_count() / g.node_count());
	println!("sizeof graph = {} Mb", g.bytesize() as f64 / 1000_000.0);
    c.bench_function("find shortest path", |b| b.iter(|| black_box(digraph_sp(&g))));
}

criterion_group!(benches, digraph_breadth_construction, digraph_breadth_first_search, digraph_find_shortest_path);
criterion_main!(benches);
