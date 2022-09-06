#![allow(unused)]
use criterion::Throughput;
use criterion::{criterion_group, criterion_main, black_box, BenchmarkId, Criterion};
use gdsl::*;
use rand::*;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::HashSet;
use gdsl::digraph::*;
use gdsl::sync_digraph:: {
	Node as AsyncNode,
	Edge as AsyncEdge,
};

use gdsl::{
	digraph_node as node,
	digraph_connect as connect,
	sync_digraph_connect as async_connect,
	sync_digraph_node as async_node,
};

mod test_graphs;
use test_graphs::*;

// ============================================================================

fn digraph_creation(c: &mut Criterion) {
    let b = 1000;

	let mut group = c.benchmark_group("digraph creation");
    for (i, size) in [b]
        .iter()
        .enumerate()
    {
		group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("graph", size), &i, |b, _| {
			b.iter(|| {
				black_box(create_graph_simple_1(*size, size / 10));
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
    for (i, size) in [b, 2 * b, 4 * b]
        .iter()
        .enumerate()
    {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_graph_simple_1(*size, size / 10);

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
    }

    group.finish();
}

fn digraph_bfs(c: &mut Criterion) {
    let b = 1000;

	let mut group = c.benchmark_group("digraph bfs");
    for (i, size) in [b, 2 * b, 4 * b]
        .iter()
        .enumerate()
    {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_graph_simple_1(*size, size / 10);

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
    for (i, size) in [b, 2 * b, 4 * b, 8 * b, 16 * b]
        .iter()
        .enumerate()
    {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_graph_simple_1(*size, size / 10);

        group.bench_with_input(BenchmarkId::new("find scc's", size), &i, |b, _| {
			b.iter(|| {
				black_box(g.scc());
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
);
criterion_main!(benches);
