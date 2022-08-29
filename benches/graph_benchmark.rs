#![allow(unused)]
use criterion::Throughput;
use criterion::{criterion_group, criterion_main, black_box, BenchmarkId, Criterion};
use gdsl::*;
use rand::*;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::HashSet;
use gdsl::digraph::*;
use gdsl::async_digraph:: {
	Node as AsyncNode,
	Edge as AsyncEdge,
};

use gdsl::{
	digraph_node as node,
	digraph_connect as connect,
	async_digraph_connect as async_connect,
	async_digraph_node as async_node,
};

mod test_graphs;
mod bucket_shortest_path;
mod par_bucket_shortest_path;

use test_graphs::*;
use bucket_shortest_path::*;
use par_bucket_shortest_path::*;

// use gdsl::digraph::{
// 	Node,
// 	Graph
// };


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

type AN = AsyncNode<usize, Dist, u64>;

pub fn create_graph_vec_distance_async(size: usize, avg_dgr: usize) -> Vec<AN> {
	let mut g = Vec::new();

    for i in 0..size {
        g.push(async_node!(i, Dist::new(u64::MAX)));
    }

	for node in g.iter() {
		let cur_dgr = rand::random::<usize>() % avg_dgr + 1;
		for _ in 0..cur_dgr {
			async_connect!(&node => &g[rand::random::<usize>() % size], rand::random::<u64>() % 10 + 1);
		}
	}
	g
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;

type N = Node<usize, Cell<u64>, u64>;

fn relax_node(
	node: &N,
	u_dist: u64,
	heap: &mut BinaryHeap<Reverse<N>>,
	visited: &mut HashSet<usize>
) {
	for (_, v, e) in node {
		let new_dist = u_dist + e;
		let cur_dist = v.get();
		if new_dist < cur_dist {
			v.set(new_dist);
			if visited.insert(*v.key()) {
				heap.push(Reverse(v.clone()));
			}
		}
	}
}

fn dijkstra(s: &N) {
	let mut heap = BinaryHeap::new();
	let mut visited = HashSet::new();

	s.set(0);
	heap.push(Reverse(s.clone()));
	relax_node(s, 0, &mut heap, &mut visited);

	while let Some(u) = heap.pop() {
		let u = u.0;
		relax_node(&u, u.get(), &mut heap, &mut visited);
	}
}

fn digraph_dijkstra(c: &mut Criterion) {
    let b = 10000;

	let mut group = c.benchmark_group("digraph dijkstra");
    for (i, size) in [b, 2 * b, 4 * b]
        .iter()
        .enumerate()
    {
		group.throughput(Throughput::Elements(*size as u64));
		let g = create_graph_vec_distance_2(*size, 3);
		let gp = create_graph_vec_distance_async(*size, 3);

        group.bench_with_input(BenchmarkId::new("dijkstra", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				dijkstra(&g[0]);
            })
        });

		group.bench_with_input(BenchmarkId::new("dijkstra PFS", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				g[0].set(0);
				black_box(g[0].pfs().map(&|u, v, e| {
					let (u_dist, v_dist) = (u.get(), v.get());
					if v_dist > u_dist + e { v.set(u_dist + e); }
				}).search());
            })
        });

		group.bench_with_input(BenchmarkId::new("delta_stepping: D = 1", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				black_box(seq_dstep_sd(&g[0], 1))
            })
        });

		group.bench_with_input(BenchmarkId::new("delta_stepping: D = 3", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				black_box(seq_dstep_sd(&g[0], 3))
            })
        });

		group.bench_with_input(BenchmarkId::new("delta_stepping: D = 6", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				black_box(seq_dstep_sd(&g[0], 6))
            })
        });

		group.bench_with_input(BenchmarkId::new("delta_stepping: D = 10", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				black_box(seq_dstep_sd(&g[0], 10))
            })
        });

		group.bench_with_input(BenchmarkId::new("delta_stepping: D = 20", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				black_box(seq_dstep_sd(&g[0], 20))
            })
        });

		group.bench_with_input(BenchmarkId::new("delta_stepping: D = 30", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				black_box(seq_dstep_sd(&g[0], 30))
            })
        });

		group.bench_with_input(BenchmarkId::new("delta_stepping: D = 50", size), &i, |b, _| {
			b.iter(|| {
				for node in g.iter() {
					node.set(u64::MAX);
				}
				black_box(seq_dstep_sd(&g[0], 50))
            })
        });

		// group.bench_with_input(BenchmarkId::new("delta_stepping_async: D = 1", size), &i, |b, _| {
		// 	b.iter(|| {
		// 		for node in gp.iter() {
		// 			node.set(u64::MAX);
		// 		}
		// 		black_box(par_seq_dstep_sd(&gp[0], 1))
        //     })
        // });

		// group.bench_with_input(BenchmarkId::new("delta_stepping_async: D = 3", size), &i, |b, _| {
		// 	b.iter(|| {
		// 		for node in gp.iter() {
		// 			node.set(u64::MAX);
		// 		}
		// 		black_box(par_seq_dstep_sd(&gp[0], 1))
        //     })
        // });

		// group.bench_with_input(BenchmarkId::new("delta_stepping_async: D = 10", size), &i, |b, _| {
		// 	b.iter(|| {
		// 		for node in gp.iter() {
		// 			node.set(u64::MAX);
		// 		}
		// 		black_box(par_seq_dstep_sd(&gp[0], 1))
        //     })
        // });

		// group.bench_with_input(BenchmarkId::new("delta_stepping_async: D = 100", size), &i, |b, _| {
		// 	b.iter(|| {
		// 		for node in gp.iter() {
		// 			node.set(u64::MAX);
		// 		}
		// 		black_box(par_seq_dstep_sd(&gp[0], 1))
        //     })
        // });
    }

    group.finish();
}


criterion_group!(
    benches,
	// digraph_creation,
	// digraph_dfs,
	// digraph_bfs,
	// digraph_scc,
	digraph_dijkstra,
);
criterion_main!(benches);
