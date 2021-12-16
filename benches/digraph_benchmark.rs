use criterion::Throughput;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fastgraph::core::Empty;
use fastgraph::{collections::*, core::*};
use rand::Rng;

type TestDigraph = Digraph<usize, Empty, Empty>;

fn rand_range(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn create_digraph(size: usize, degree: usize) -> TestDigraph {
    let mut g = TestDigraph::new();
    for i in 0..size {
        g.add_node(i, Empty);
    }
    for i in 0..size {
        let new_degree = rand_range(0, degree * 2);
        for _ in 0..new_degree {
            g.add_edge(i, rand_range(0, size), Empty);
        }
    }
    g
}

// ============================================================================

fn bench_bfs_target(c: &mut Criterion) {
    static B: usize = 1000;

    let mut group = c.benchmark_group("Breadth First Traversal Target");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B, 32 * B, 64 * B, 128 * B]
        .iter()
        .enumerate()
    {
        let g = create_digraph(*size, 16);
        let t = g.get_node(666).unwrap();
        group.throughput(Throughput::Bytes((g.size_of() * 10) as u64));
        group.bench_with_input(BenchmarkId::new("Sequential", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.breadth_first(i, |e| {
                        if t == e.target() {
                            Traverse::Finish
                        } else {
                            Traverse::Include
                        }
                    });
                }
            })
        });
        group.bench_with_input(BenchmarkId::new("Parallel", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.par_breadth_first(i, |e| {
                        if t == e.target() {
                            Traverse::Finish
                        } else {
                            Traverse::Include
                        }
                    });
                }
            })
        });
    }
    group.finish();
}

// ============================================================================

fn bench_bfs_no_worload(c: &mut Criterion) {
    static B: usize = 1000;

    let mut group = c.benchmark_group("Breadth First Traversal No Workload");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B, 32 * B, 64 * B, 128 * B]
        .iter()
        .enumerate()
    {
        let g = create_digraph(*size, 16);
        group.throughput(Throughput::Bytes((g.size_of() * 10) as u64));
        group.bench_with_input(BenchmarkId::new("Sequential", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.breadth_first(i, |_| Traverse::Include);
                }
            })
        });
        group.bench_with_input(BenchmarkId::new("Parallel", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.par_breadth_first(i, |_| Traverse::Include);
                }
            })
        });
    }
    group.finish();
}

// ============================================================================

fn bench_bfs_even_workload(c: &mut Criterion) {
    static B: usize = 10;

    let mut group = c.benchmark_group("Breadth First Traversal Even Workload");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B, 32 * B, 64 * B, 128 * B]
        .iter()
        .enumerate()
    {
        let g = create_digraph(*size, 16);
        group.throughput(Throughput::Bytes((g.size_of() * 10) as u64));
        group.bench_with_input(BenchmarkId::new("Sequential", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.breadth_first(i, |_| {
                        std::thread::sleep(std::time::Duration::from_nanos(10));
                        Traverse::Include
                    });
                }
            })
        });
        group.bench_with_input(BenchmarkId::new("Parallel", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.par_breadth_first(i, |_| {
                        std::thread::sleep(std::time::Duration::from_nanos(10));
                        Traverse::Include
                    });
                }
            })
        });
    }
    group.finish();
}

// ============================================================================

fn bench_bfs_uneven_workload(c: &mut Criterion) {
    static B: usize = 10;

    let mut group = c.benchmark_group("Breadth First Traversal Uneven Workload");
    for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B, 32 * B, 64 * B, 128 * B]
        .iter()
        .enumerate()
    {
        let g = create_digraph(*size, 16);
        group.throughput(Throughput::Bytes((g.size_of() * 10) as u64));
        group.bench_with_input(BenchmarkId::new("Sequential", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.breadth_first(i, |_| {
                        std::thread::sleep(std::time::Duration::from_nanos(
                            rand_range(0, 10) as u64
                        ));
                        Traverse::Include
                    });
                }
            })
        });
        group.bench_with_input(BenchmarkId::new("Parallel", size), &i, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    g.par_breadth_first(i, |_| {
                        std::thread::sleep(std::time::Duration::from_nanos(
                            rand_range(0, 10) as u64
                        ));
                        Traverse::Include
                    });
                }
            })
        });
    }
    group.finish();
}

// ============================================================================

criterion_group!(
    benches,
    bench_bfs_target,
    bench_bfs_no_worload,
    bench_bfs_even_workload,
    bench_bfs_uneven_workload
);
criterion_main!(benches);
