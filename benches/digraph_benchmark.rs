use criterion::Throughput;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ggi::graph_async::digraph::*;
use ggi::graph::digraph::*;
use ggi::*;
use rand::*;
use std::cell::{RefCell, Cell};
use min_max_heap::MinMaxHeap;

fn rand_range(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn create_dijkstra_digraph(size: usize, degree: usize) -> DiGraph<usize, RefCell<u64>, u64> {
    let mut g = DiGraph::new();
    for i in 0..size {
        g.insert(dinode!(i, RefCell::new(u64::MAX)));
    }
    for i in 0..size {
        let new_degree = rand_range(0, degree * 2);
        for _ in 0..new_degree {
            connect!(&g[i] => &g[rand_range(0, size)], rand_range(1, 100) as u64);
        }
    }
    g
}

fn create_dijkstra_digraph_cell(size: usize, degree: usize) -> DiGraph<usize, Cell<u64>, u64> {
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

fn create_async_dijkstra_digraph(size: usize, degree: usize) -> AsyncDiGraph<usize, RefCell<u64>, u64> {
    let mut g = AsyncDiGraph::new();
    for i in 0..size {
        g.insert(async_dinode!(i, RefCell::new(u64::MAX)));
    }
    for i in 0..size {
        let new_degree = rand_range(0, degree * 2);
        for _ in 0..new_degree {
            async_connect!(&g[i] => &g[rand_range(0, size)], rand_range(1, 100) as u64);
        }
    }
    g
}

// ============================================================================

fn bench_dijkstra(c: &mut Criterion) {
    static B: usize = 1000;

    let mut group = c.benchmark_group("Dijkstra");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B]
        .iter()
        .enumerate()
    {
		let g = create_dijkstra_digraph(*size, 100);

		group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("Sync", size), &i, |b, _| {
			b.iter(|| {
				let mut dists = Vec::new();
				let mut heap = MinMaxHeap::new();
				let mut visited = DiGraph::new();
				let source = &g[rand_range(0, g.len())];
				let target = &g[rand_range(0, g.len())];

				source.replace(0);
				heap.push(source.clone());

				'search: while let Some(s) = heap.pop_min() {
					for edge in &s {

						let t = edge.target();
						let (s_dist, t_dist) = (*s.borrow(), *t.borrow());
						let e_delta = *edge;

						if visited.insert(t.clone()) {
							if t_dist > s_dist + e_delta {
								t.replace(s_dist + e_delta);
								dists.push(t.clone());
								if &s == target { break 'search }
								heap.push(t.clone());
							}
						}
					}
				}

				for d in dists {
					d.replace(u64::MAX);
				}
            })
        });

		let gc = create_dijkstra_digraph_cell(*size, 100);

		group.bench_with_input(BenchmarkId::new("Sync PFS", size), &i, |b, _| {
			b.iter(|| {
				let source = &gc[rand_range(0, g.len())];
				let target = &gc[rand_range(0, g.len())];

				source.replace(0);

				source.search().pfs_min_map(&target, &|s, t, delta| {
					match t.get() > s.get() + delta {
						true => {
							t.set(s.get() + delta);
							true
						},
						false => false,
					}
				});
            })
        });

		let ag = create_async_dijkstra_digraph(*size, 100);

		group.bench_with_input(BenchmarkId::new("Async", size), &i, |b, _| {
			b.iter(|| {
				let mut dists = Vec::new();
				let mut heap = MinMaxHeap::new();
				let mut visited = AsyncDiGraph::new();
				let source = &ag[rand_range(0, g.len())];
				let target = &ag[rand_range(0, g.len())];

				source.replace(0);
				heap.push(source.clone());

				'search: while let Some(s) = heap.pop_min() {
					for edge in &s {

						let t = edge.target();
						let (s_dist, t_dist) = (*s.borrow(), *t.borrow());
						let e_delta = *edge;

						if visited.insert(t.clone()) {
							if t_dist > s_dist + e_delta {
								t.replace(s_dist + e_delta);
								dists.push(t.clone());
								if &s == target { break 'search }
								heap.push(t.clone());
							}
						}
					}
				}

				for d in dists {
					d.replace(u64::MAX);
				}
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_dijkstra,
);
criterion_main!(benches);
