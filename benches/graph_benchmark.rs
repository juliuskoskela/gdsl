use criterion::Throughput;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gdsl::graph::*;
use gdsl::node::Node;
use gdsl::*;
use rand::*;
use std::cell::Cell;
use std::cmp::{max, min};
// use min_max_heap::MinMaxHeap;

fn rand_range(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn create_dijkstra_digraph(size: usize, degree: usize) -> Graph<usize, Cell<u64>, u64> {
    let mut g = Graph::new();
    for i in 0..size {
        g.insert(node!(i, Cell::new(u64::MAX)));
    }
    for i in 0..size {
        let new_degree = rand_range(0, degree * 2);
        for _ in 0..new_degree {
            connect!(&g[i] => &g[rand_range(0, size)], rand_range(1, 100) as u64);
        }
    }
    g
}

fn create_dijkstra_digraph_against_petgraph(size: usize) -> Vec<Node<usize, Cell<usize>, usize>> {
    let mut g = Vec::new();
    for i in 0..size {
        g.push(node!(i, Cell::new(usize::MAX)));
    }

	for (i, node) in g.iter().enumerate() {
		let neighbour_count = i % 8 + 3;
		let j_from = max(0, i as i32 - neighbour_count as i32 / 2) as usize;
		let j_to = min(size, j_from + neighbour_count);
		for j in j_from..j_to {
			connect!(&node => &g[j], (i + 3) % 10);
		}
	}
    g
}


// ============================================================================

fn bench_search(c: &mut Criterion) {
    static B: usize = 1000;

    let mut group = c.benchmark_group("Depth First Search");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B]
        .iter()
        .enumerate()
    {

		group.throughput(Throughput::Elements(*size as u64));
		let g = create_dijkstra_digraph(*size, 10);

        group.bench_with_input(BenchmarkId::new("unbound", size), &i, |b, _| {
			b.iter(|| {
				g[0].dfs().search(None);
            })
        });

		group.bench_with_input(BenchmarkId::new("unbound path", size), &i, |b, _| {
			b.iter(|| {
				g[0].dfs_path().search(None);
            })
        });
    }
    group.finish();
}

// ============================================================================

fn bench_dijkstra(c: &mut Criterion) {
    static B: usize = 10000;

    let mut group = c.benchmark_group("Dijkstra");
    for (i, size) in [B]
        .iter()
        .enumerate()
    {

		group.throughput(Throughput::Elements(*size as u64));

		let g = create_dijkstra_digraph(*size, 100);

		group.bench_with_input(BenchmarkId::new("With PFS", size), &i, |b, _| {
			b.iter(|| {
				let source = &g[rand_range(0, g.len())];
				let target = &g[rand_range(0, g.len())];

				source.replace(0);

				source.pfs_min().search_filter_map(Some(&target), &|s, t, delta| {
					let (s_dist, t_dist) = (s.get(), t.get());
					match t_dist > s_dist + delta {
						true => {
							t.set(s_dist + delta);
							true
						},
						false => false,
					}
				});
				g.iter().for_each(|(_, n)| n.set(u64::MAX));
            })
        });

		let g = create_dijkstra_digraph_against_petgraph(*size);
		group.bench_with_input(BenchmarkId::new("Petgraph Test", size), &i, |b, _| {
			b.iter(|| {
				let source = &g[0];

				source.replace(0);

				source.pfs_min().search_filter_map(None, &|s, t, delta| {
					let (s_dist, t_dist) = (s.get(), t.get());
					match t_dist > s_dist + delta {
						true => {
							t.set(s_dist + delta);
							true
						},
						false => false,
					}
				});

				for n in &g {
					n.set(usize::MAX);
				}
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
