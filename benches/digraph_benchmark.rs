use criterion::{criterion_group, criterion_main, Criterion};
use graph::digraph::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

const KEY_SIZE: usize = 6;
const KEY_COUNT: usize = 10000;
const NODE_DEGREE: usize = 10;
const BFS_SAMPLES: usize = 10;

type BenchGraph = Digraph<String, usize, usize>;

fn rand_string(len: usize) -> String {
	thread_rng()
		.sample_iter(&Alphanumeric)
		.take(len)
		.map(char::from)
		.collect()
}

fn rand_keys(count: usize, keysize: usize) -> Vec<String> {
	let mut i = 0;
	let mut keys = vec![];
	while i < count {
		keys.push(rand_string(keysize));
		i += 1;
	}
	keys
}

fn rand_range(start: usize, end: usize) -> usize {
	let mut rng = rand::thread_rng();
	rng.gen_range(start..end)
}

fn rand_bfs(g: &BenchGraph, keys: &Vec<String>) {
	g.bfs(&keys[rand_range(0, keys.len() - 1)], &keys[rand_range(0, keys.len() - 1)]);
}

fn criterion_benchmark(c: &mut Criterion) {

	let mut g = BenchGraph::new();
	let keys = rand_keys(KEY_COUNT, KEY_SIZE);
	for key in keys.iter() {
		g.insert(key.clone(), 0);
	}
	for key in keys.iter() {
		let mut i = 0;
		while i < NODE_DEGREE {
			g.connect(key, &keys[rand_range(0, keys.len() - 1)], 0);
			i += 1;
		}
	}
    c.bench_function("breadth first search", |b| b.iter(|| rand_bfs(&g, &keys)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
