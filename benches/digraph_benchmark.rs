use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph::digraph::*;
use rand::Rng;

const KEY_COUNT: usize = 10000;
const NODE_DEGREE: usize = 100;

type IntKeysGraph = Digraph<usize, usize, usize>;

fn rand_range(start: usize, end: usize) -> usize {
	let mut rng = rand::thread_rng();
	rng.gen_range(start..end)
}

fn create_graph() -> IntKeysGraph {
	let mut g = IntKeysGraph::new();
	for i in 0..KEY_COUNT {
		g.insert(i, 0);
	}
	for i in 0..KEY_COUNT {
		for _ in 0..NODE_DEGREE {
			g.connect(&i, &rand_range(0, KEY_COUNT), 0);
		}
	}
	g
}

fn digraph_bfs(g: &IntKeysGraph) {
	g.bfs(&rand_range(0, KEY_COUNT), &rand_range(0, KEY_COUNT));
}

fn digraph_breadth_first_search(c: &mut Criterion) {

	let g = create_graph();
    c.bench_function("breadth first search", |b| b.iter(|| black_box(digraph_bfs(&g))));
}

fn digraph_sp(g: &IntKeysGraph) {
	g.shortest_path(&rand_range(0, KEY_COUNT), &rand_range(0, KEY_COUNT));
}

fn digraph_find_shortest_path(c: &mut Criterion) {

	let g = create_graph();
    c.bench_function("find shortest path", |b| b.iter(|| black_box(digraph_sp(&g))));
}

criterion_group!(benches, digraph_breadth_first_search, digraph_find_shortest_path);
criterion_main!(benches);
