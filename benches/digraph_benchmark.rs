use criterion::Throughput;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dug::digraph::*;
use dug::*;
use rand::*;
use std::cell::Cell;
use min_max_heap::MinMaxHeap;

fn rand_range(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn create_dijkstra_digraph(size: usize, degree: usize) -> DiGraph<usize, Cell<u64>, u64> {
    let mut g = DiGraph::new();
    for i in 0..size {
        g.insert(dinode!(i, Cell::new(u64::MAX)));
    }
    for i in 0..size {
        let new_degree = rand_range(0, degree * 2);
        for _ in 0..new_degree {
            connect!(&g[i] => &g[rand_range(0, size)], rand_range(1, 100) as u64);
        }
    }
    g
}

// ============================================================================

fn bench_search(c: &mut Criterion) {
    static B: usize = 1000;

    let mut group = c.benchmark_group("Dijkstra");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B]
        .iter()
        .enumerate()
    {

		group.throughput(Throughput::Elements(*size as u64));
		let g = create_dijkstra_digraph(*size, 100);

        group.bench_with_input(BenchmarkId::new("Depth First Search", size), &i, |b, _| {
			b.iter(|| {
				g[0].search().dfs(&g[rand_range(0, *size)]);
            })
        });

		group.bench_with_input(BenchmarkId::new("Breadth First Search", size), &i, |b, _| {
			b.iter(|| {
				g[0].search().bfs(&g[rand_range(0, *size)]);
            })
        });
    }
    group.finish();
}

// ============================================================================

fn bench_dijkstra(c: &mut Criterion) {
    static B: usize = 1000;

    let mut group = c.benchmark_group("Dijkstra");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B]
        .iter()
        .enumerate()
    {

		group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("Sync Naive", size), &i, |b, _| {
			b.iter(|| {
				let g = create_dijkstra_digraph(*size, 100);
				let mut heap = MinMaxHeap::new();
				let mut visited = std::collections::HashSet::new();
				let source = &g[rand_range(0, g.len())];
				let target = &g[rand_range(0, g.len())];

				source.replace(0);
				heap.push(source.clone());

				'search: while let Some(s) = heap.pop_min() {
					for (delta, t) in &s {
						if !visited.contains(t.key()) {
							let (s_dist, t_dist) = (s.get(), t.get());
							if t_dist > s_dist + delta {
								visited.insert(t.key().clone());
								t.replace(s_dist + delta);
								if &s == target { break 'search }
								heap.push(t.clone());
							}
						}
					}
				}
            })
        });

		group.bench_with_input(BenchmarkId::new("Sync PFS", size), &i, |b, _| {
			b.iter(|| {
				let g = create_dijkstra_digraph(*size, 100);
				let source = &g[rand_range(0, g.len())];
				let target = &g[rand_range(0, g.len())];

				source.replace(0);

				source.search().pfs_min_map(&target, &|s, t, delta| {
					let (s_dist, t_dist) = (s.get(), t.get());
					match t_dist > s_dist + delta {
						true => {
							t.set(s_dist + delta);
							true
						},
						false => false,
					}
				});
            })
        });

    }
    group.finish();
}

criterion_group!(
    benches,
	bench_search,
    bench_dijkstra,
);
criterion_main!(benches);
