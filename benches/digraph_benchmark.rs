// use criterion::Throughput;
// use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use fastgraph::dinode::Node;
// use fastgraph::node::*;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use fastgraph::*;
// use rand::*;
// use crate::enums::{Coll, Sig};

// type DistNode = Node<usize, u64, u64>;

// fn rand_range(start: usize, end: usize) -> usize {
//     let mut rng = rand::thread_rng();
//     rng.gen_range(start..end)
// }

// fn create_dijkstra_graph(size: usize, degree: usize) -> Vec<DistNode> {
//     let mut g: Vec<DistNode> = Vec::new();
//     for i in 0..size {
//         g.push(node!(i, u64::MAX));
//     }
//     for i in 0..size {
//         let new_degree = rand_range(0, degree * 2);
//         for _ in 0..new_degree {
//             connect!(&g[i] => &g[rand_range(0, size)], rand_range(1, 100) as u64);
//         }
//     }
//     g
// }

// // ============================================================================

// fn bench_dijkstra(c: &mut Criterion) {
//     static B: usize = 1000;

//     let mut group = c.benchmark_group("Dijkstra");
//     for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B]
//         .iter()
//         .enumerate()
//     {
//         let g = create_dijkstra_graph(*size, 100);
//         group.throughput(Throughput::Bytes((g.len() * std::mem::size_of::<DistNode>()) as u64));
//         group.bench_with_input(BenchmarkId::new("Min Heap", size), &i, |b, _| {
//             b.iter(|| {
//                 let mut min_heap = BinaryHeap::<Reverse<DistNode>>::new();
// 				let source = &g[rand_range(0, g.len() - 1)];
// 				let target = &g[rand_range(0, g.len() - 1)];
// 				source.store(0);
// 				min_heap.push(Reverse(source.clone()));
// 				while let Some(u) = min_heap.pop() {
// 					let (u_dist, u) = (u.0.load(), u.0);
// 					if &u == target { break; }
// 					u.adjacent(|v, e| {
// 						let (v_dist, e_len) = (v.load(), e);
// 						if v_dist > u_dist + e_len {
// 							v.store(u_dist + e_len);
// 							min_heap.push(Reverse(v.clone()));
// 						}
// 						true
// 					});
// 				}
//             })
//         });
//     }
//     group.finish();
// }

// // ============================================================================

// use fastgraph::edge::GraphEdge;
// fn bench_bfs(c: &mut Criterion) {
//     static B: usize = 1000;

//     let mut group = c.benchmark_group("BFS");
//     for (i, size) in [B, 2 * B, 4 * B, 8 * B, 16 * B]
//         .iter()
//         .enumerate()
//     {
//         let g = create_dijkstra_graph(*size, 100);
// 		let g2 = create_dijkstra_graph(*size, 100);
//         group.throughput(Throughput::Bytes((g.len() * std::mem::size_of::<DistNode>()) as u64));
//         group.bench_with_input(BenchmarkId::new("Parallel", size), &i, |b, _| {
//             b.iter(|| {
// 				let source = &g[rand_range(0, g.len() - 1)];
// 				let target = &g[rand_range(0, g.len() - 1)];
//                 let _shortest_path = source.bfs(|_, v, _| {
// 					if v == target {
// 						(Coll::Include, Sig::Terminate)
// 					} else {
// 						(Coll::Include, Sig::Continue)
// 					}
// 				});
//             })
//         });
// 		group.bench_with_input(BenchmarkId::new("Seq", size), &i, |b, _| {
//             b.iter(|| {
// 				let source = &g2[rand_range(0, g.len() - 1)];
// 				let target = &g2[rand_range(0, g.len() - 1)];
// 				let mut edge_tree = Vec::new();
// 				let first_edge = source.outbound().read();
// 				let first_edge = first_edge.first().unwrap();
// 				edge_tree.push(first_edge.clone());
// 				let (mut low, mut high) = (0, 0);
// 				loop {
// 					high = edge_tree.len();
// 					let adjacent = source.outbound().read();
// 					for edge in adjacent.iter() {
// 						match edge.target().try_close() {
// 							Ok(_) => {
// 								edge_tree.push(edge.clone());
// 								if edge.target() == target {
// 									break;
// 								}
// 							}
// 							Err(_) => {
// 								continue;
// 							}
// 						}
// 					}
// 					if low == high {
// 						break;
// 					}
// 					low = high;
// 				}
//             })
//         });
//     }
//     group.finish();
// }

// // ============================================================================

// criterion_group!(
//     benches,
//     bench_dijkstra,
// 	bench_bfs
// );
// criterion_main!(benches);

fn main() {}
