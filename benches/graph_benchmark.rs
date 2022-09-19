use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
mod test_graphs;
use test_graphs::*;
mod page_rank;
use page_rank::*;

// ============================================================================

fn digraph_creation(c: &mut Criterion) {
	let b = 1000;

	let mut group = c.benchmark_group("digraph creation");
	for (i, size) in [b].iter().enumerate() {
		group.throughput(Throughput::Elements(*size as u64));

		group.bench_with_input(BenchmarkId::new("graph", size), &i, |b, _| {
			b.iter(|| {
				black_box(create_digraph_simple_1(*size, size / 10));
			})
		});

		group.bench_with_input(BenchmarkId::new("vec", size), &i, |b, _| {
			b.iter(|| {
				black_box(create_graph_vec_distance_1(*size));
			})
		});
	}
	group.finish();
}

fn digraph_dfs(c: &mut Criterion) {
	let b = 1000;

	let mut group = c.benchmark_group("digraph dfs");
	for (i, size) in [b, 2 * b, 4 * b].iter().enumerate() {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_digraph_simple_1(*size, size / 10);
		let sg = create_sync_digraph_simple_1(*size, size / 10);

		group.bench_with_input(BenchmarkId::new("find", size), &i, |b, _| {
			b.iter(|| {
				let s = &g[rand::random::<usize>() % g.len()];
				let t = &g[rand::random::<usize>() % g.len()];
				black_box(s.dfs().target(t.key()).search());
			})
		});

		group.bench_with_input(BenchmarkId::new("path", size), &i, |b, _| {
			b.iter(|| {
				let s = &g[rand::random::<usize>() % g.len()];
				let t = &g[rand::random::<usize>() % g.len()];
				black_box(s.dfs().target(t.key()).search_path());
			})
		});

		group.bench_with_input(BenchmarkId::new("cycle", size), &i, |b, _| {
			b.iter(|| {
				let s = &g[rand::random::<usize>() % g.len()];
				black_box(s.dfs().search_cycle());
			})
		});

		group.bench_with_input(BenchmarkId::new("sync find", size), &i, |b, _| {
			b.iter(|| {
				let s = &sg[rand::random::<usize>() % g.len()];
				let t = &sg[rand::random::<usize>() % g.len()];
				black_box(s.dfs().target(t.key()).search());
			})
		});

		group.bench_with_input(BenchmarkId::new("sync path", size), &i, |b, _| {
			b.iter(|| {
				let s = &sg[rand::random::<usize>() % g.len()];
				let t = &sg[rand::random::<usize>() % g.len()];
				black_box(s.dfs().target(t.key()).search_path());
			})
		});

		group.bench_with_input(BenchmarkId::new("sync cycle", size), &i, |b, _| {
			b.iter(|| {
				let s = &sg[rand::random::<usize>() % g.len()];
				black_box(s.dfs().search_cycle());
			})
		});
	}

	group.finish();
}

fn digraph_bfs(c: &mut Criterion) {
	let b = 1000;

	let mut group = c.benchmark_group("digraph bfs");
	for (i, size) in [b, 2 * b, 4 * b].iter().enumerate() {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_digraph_simple_1(*size, size / 10);

		group.bench_with_input(BenchmarkId::new("find", size), &i, |b, _| {
			b.iter(|| {
				let s = &g[rand::random::<usize>() % g.len()];
				let t = &g[rand::random::<usize>() % g.len()];
				black_box(s.bfs().target(t.key()).search());
			})
		});

		group.bench_with_input(BenchmarkId::new("path", size), &i, |b, _| {
			b.iter(|| {
				let s = &g[rand::random::<usize>() % g.len()];
				let t = &g[rand::random::<usize>() % g.len()];
				black_box(s.bfs().target(t.key()).search_path());
			})
		});

		group.bench_with_input(BenchmarkId::new("cycle", size), &i, |b, _| {
			b.iter(|| {
				let s = &g[rand::random::<usize>() % g.len()];
				black_box(s.bfs().search_cycle());
			})
		});
	}

	group.finish();
}

fn digraph_scc(c: &mut Criterion) {
	let b = 100;

	let mut group = c.benchmark_group("digraph scc");
	for (i, size) in [b, 2 * b, 4 * b, 8 * b, 16 * b].iter().enumerate() {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_digraph_simple_1(*size, size / 10);

		group.bench_with_input(BenchmarkId::new("find scc's", size), &i, |b, _| {
			b.iter(|| {
				black_box(g.scc());
			})
		});
	}

	group.finish();
}

fn digraph_serde(c: &mut Criterion) {
	use gdsl::digraph::*;
	let b = 1000;

	let mut group = c.benchmark_group("digraph serde");
	for (i, size) in [b, 2 * b, 4 * b].iter().enumerate() {
		group.throughput(Throughput::Elements(*size as u64));

		let g = create_digraph_simple_1(*size, size / 10);
		let json = serde_json::to_vec(&g).unwrap();
		let cbor = serde_cbor::to_vec(&g).unwrap();

		group.bench_with_input(BenchmarkId::new("serialize JSON", size), &i, |b, _| {
			b.iter(|| {
				black_box(serde_json::to_vec(&g).unwrap());
			})
		});

		group.bench_with_input(BenchmarkId::new("deserialize JSON", size), &i, |b, _| {
			b.iter(|| {
				black_box(serde_json::from_slice::<Graph<usize, (), ()>>(&json).unwrap());
			})
		});

		group.bench_with_input(BenchmarkId::new("serialize CBOR", size), &i, |b, _| {
			b.iter(|| {
				black_box(serde_cbor::to_vec(&g).unwrap());
			})
		});

		group.bench_with_input(BenchmarkId::new("deserialize CBOR", size), &i, |b, _| {
			b.iter(|| {
				black_box(serde_cbor::from_slice::<Graph<usize, (), ()>>(&cbor).unwrap());
			})
		});
	}

	group.finish();
}

fn digraph_page_rank(c: &mut Criterion) {
	let b = 10_000;
	let α = 0.85;
	let ε = 0.0001;

	let mut group = c.benchmark_group("digraph PageRank");
	for (i, size) in [b, 2 * b, 4 * b].iter().enumerate() {
		group.throughput(Throughput::Elements(*size as u64));

		let g = create_page_rank_dataset(*size, 10);

		group.bench_with_input(BenchmarkId::new("PageRank", size), &i, |b, _| {
			b.iter(|| {
				black_box(page_rank(&g, α, ε));
			})
		});
	}

	group.finish();
}

criterion_group!(
	benches,
	digraph_creation,
	digraph_dfs,
	digraph_bfs,
	digraph_scc,
	digraph_serde,
	digraph_page_rank
);
criterion_main!(benches);
