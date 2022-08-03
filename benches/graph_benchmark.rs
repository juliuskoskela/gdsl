#![allow(unused)]
use criterion::Throughput;
use criterion::{criterion_group, criterion_main, black_box, BenchmarkId, Criterion};
use gdsl::*;
use rand::*;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::HashSet;

mod test_graphs;
use test_graphs::*;

// use gdsl::digraph::{
// 	Node,
// 	Graph
// };


// ============================================================================

fn bench_graph_creation(c: &mut Criterion) {
    let b = 10000;

	let mut group = c.benchmark_group("Group 1");
    for (i, size) in [b, 2 * b, 4 * b, 8 * b, 16 * b]
        .iter()
        .enumerate()
    {
		group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("create distance graph", size), &i, |b, _| {
			b.iter(|| {
				black_box(create_graph_simple_1(*size, size / 10));
            })
        });

		group.bench_with_input(BenchmarkId::new("create distance graph as vec", size), &i, |b, _| {
			b.iter(|| {
				black_box(create_graph_vec_distance_1(*size));
            })
        });
    }
    group.finish();
}

fn bench_ordering(c: &mut Criterion) {
    let b = 10000;

	let mut group = c.benchmark_group("Group 1");
    for (i, size) in [b, 2 * b, 4 * b, 8 * b, 16 * b]
        .iter()
        .enumerate()
    {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_graph_simple_1(*size, size / 10);
        group.bench_with_input(BenchmarkId::new("create distance graph", size), &i, |b, _| {
			b.iter(|| {
				//
            })
        });

		group.bench_with_input(BenchmarkId::new("create distance graph as vec", size), &i, |b, _| {
			b.iter(|| {
				//
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
	bench_graph_creation,
);
criterion_main!(benches);
