#![allow(unused)]
use criterion::Throughput;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gdsl::digraph::*;
use rand::*;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::HashSet;
// use min_max_heap::MinMaxHeap;
type Node = DiNode<usize, Empty, Empty>;
type Graph = DiGraph<usize, Empty, Empty>;

fn create_scc_graph(size: usize, degree: usize) -> Graph {
    let mut g = Graph::new();
    for i in 0..size {
        g.insert(dinode!(i));
    }
    for i in 0..size {
        let new_degree = rand_range(0, degree * 2);
        for _ in 0..new_degree {
            connect!(&g[i] => &g[rand_range(0, size)]);
        }
    }
    g
}

// fn ordering(graph: &Graph) -> Vec<Node> {
// 	let mut visited = HashSet::new();
// 	let mut ordering = Vec::new();
// 	for (_, root) in graph.iter() {
// 		if !visited.contains(root.key()) {
// 			let partial_ord = root.dfs()
// 				.postorder_fmap(None, &mut |_, v, _| {
// 					!visited.contains(v.key())
// 			});
// 			for node in &partial_ord {
// 				visited.insert(node.key().clone());
// 				ordering.push(node.clone());
// 			}
// 		}
// 	}
// 	ordering
// }

// fn kojarasu(graph: &Graph) -> Vec<Vec<Node>> {
// 	let mut invariant = HashSet::new();
// 	let mut components = Vec::new();
// 	let mut ordering = ordering(graph);
// 	while let Some(node) = ordering.pop() {
// 		if invariant.contains(node.key()) {
// 			continue;
// 		}
// 		let cycle = node.dfs()
// 			.transpose()
// 			.cycle(&mut |_, v, _| {
// 				!invariant.contains(v.key())
// 		});
// 		match cycle {
// 			Some(cycle) => {
// 				for node in &cycle {
// 					invariant.insert(node.key().clone());
// 				}
// 				components.push(cycle);
// 			},
// 			None => {
// 				invariant.insert(node.key().clone());
// 				components.push(vec![node.clone()]);
// 			},
// 		}
// 	}
// 	components
// }

// fn partition_cycles(postorder: &mut Vec<Node>, visited: &HashSet<usize>) -> Vec<Vec<Node>> {
// 	let mut invariant = HashSet::new();
// 	let mut components = Vec::new();
// 	while let Some(node) = postorder.pop() {
// 		if invariant.contains(node.key()) {
// 			continue;
// 		}
// 		let cycle = node.dfs()
// 			.transpose()
// 			.cycle(&mut |_, v, _| {
// 				!invariant.contains(v.key()) && visited.contains(v.key())
// 		});
// 		match cycle {
// 			Some(cycle) => {
// 				for node in &cycle {
// 					invariant.insert(node.key().clone());
// 				}
// 				components.push(cycle);
// 			},
// 			None => {
// 				invariant.insert(node.key().clone());
// 				components.push(vec![node.clone()]);
// 			},
// 		}
// 	}
// 	components
// }

// fn scc(graph: &Graph) -> Vec<Vec<Node>> {
// 	let mut visited = HashSet::new();
// 	let mut result = vec![];
// 	for (_, root) in graph.iter() {
// 		if !visited.contains(root.key()) {
// 			let mut postorder = root.dfs()
// 				.postorder_fmap(None, &mut |_, v, _| {
// 					!visited.contains(v.key())
// 			});
// 			for node in &postorder {
// 				visited.insert(node.key().clone());
// 			}
// 			let components = partition_cycles(&mut postorder, &visited);
// 			for component in components {
// 				result.push(component);
// 			}
// 		}
// 	}
// 	result
// }

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

fn create_dijkstra_digraph_against_petgraph(size: usize) -> Vec<DiNode<usize, Cell<usize>, usize>> {
    let mut g = Vec::new();
    for i in 0..size {
        g.push(dinode!(i, Cell::new(usize::MAX)));
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

        group.bench_with_input(BenchmarkId::new("Find", size), &i, |b, _| {
			b.iter(|| {
				let random = thread_rng().next_u64() as usize % g.len();
				g[0].dfs().target(&random).find();
            })
        });
    }
    group.finish();
}

// ============================================================================

// fn bench_scc(c: &mut Criterion) {
//     static B: usize = 100;

//     let mut group = c.benchmark_group("SCC");
//     for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B]
//         .iter()
//         .enumerate()
//     {

// 		group.throughput(Throughput::Elements(*size as u64));
// 		let g = create_scc_graph(*size, 3);

//         group.bench_with_input(BenchmarkId::new("Tarjan's", size), &i, |b, _| {
// 			b.iter(|| {
// 				scc(&g);
//             })
//         });

// 		group.bench_with_input(BenchmarkId::new("Kojarasu", size), &i, |b, _| {
// 			b.iter(|| {
// 				kojarasu(&g);
//             })
//         });
//     }
//     group.finish();
// }

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

				source.pfs().target(&rand_range(0, g.len())).filter_map(&|s, t, delta| {
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

				source.pfs().filter_map(&|s, t, delta| {
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
	// bench_search,
	// bench_scc,
    bench_dijkstra,
);
criterion_main!(benches);
