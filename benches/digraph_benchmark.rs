use criterion::{criterion_group, criterion_main, Criterion};
use graph::digraph::*;
use rand::{thread_rng, Rng};
// use rand::distributions::Alphanumeric;

const KEY_COUNT: usize = 10000;
const NODE_DEGREE: usize = 300;

type IntKeysGraph = Digraph<usize, usize, usize>;

// fn rand_string(len: usize) -> String {
// 	thread_rng()
// 		.sample_iter(&Alphanumeric)
// 		.take(len)
// 		.map(char::from)
// 		.collect()
// }

// fn rand_keys(count: usize, keysize: usize) -> Vec<String> {
// 	let mut i = 0;
// 	let mut keys = vec![];
// 	while i < count {
// 		keys.push(rand_string(keysize));
// 		i += 1;
// 	}
// 	keys
// }

fn rand_range(start: usize, end: usize) -> usize {
	let mut rng = rand::thread_rng();
	rng.gen_range(start..end)
}

fn digraph_bfs(g: &IntKeysGraph) {
	g.bfs(&rand_range(0, KEY_COUNT), &rand_range(0, KEY_COUNT));
}


fn digraph_breadth_first_search(c: &mut Criterion) {

	let mut g = IntKeysGraph::new();
	for i in 0..KEY_COUNT {
		g.insert(i, 0);
	}
	for i in 0..KEY_COUNT {
		for _ in 0..NODE_DEGREE {
			g.connect(&i, &rand_range(0, KEY_COUNT), 0);
		}
	}
    c.bench_function("breadth first search", |b| b.iter(|| digraph_bfs(&g)));
}

fn digraph_sp(g: &IntKeysGraph) {
	let res = g.bfs(&rand_range(0, KEY_COUNT), &rand_range(0, KEY_COUNT));
	match res {
		Some(edges) => {edges.backtrack();}
		None => {return;}
	}
}

fn digraph_find_shortest_path(c: &mut Criterion) {

	let mut g = IntKeysGraph::new();
	for i in 0..KEY_COUNT {
		g.insert(i, 0);
	}
	for i in 0..KEY_COUNT {
		for _ in 0..NODE_DEGREE {
			g.connect(&i, &rand_range(0, KEY_COUNT), 0);
		}
	}
    c.bench_function("find shortest path", |b| b.iter(|| digraph_sp(&g)));
}

criterion_group!(benches, digraph_find_shortest_path, digraph_breadth_first_search);
criterion_main!(benches);
