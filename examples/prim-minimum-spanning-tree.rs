// Prim's algorithm
//
// Prim's algorithm (also known as Jarník's algorithm) is a greedy algorithm
// that finds a minimum spanning tree for a weighted undirected graph. This
// means it finds a subset of the edges that forms a tree that includes every
// vertex, where the total weight of all the edges in the tree is minimized.
// The algorithm operates by building this tree one vertex at a time,
// from an arbitrary starting vertex, at each step adding the cheapest possible
// connection from the tree to another vertex.
//
// https://en.wikipedia.org/wiki/Prim%27s_algorithm

use gdsl::ungraph::*;
use gdsl::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type N = Node<usize, (), u64>;
type E = Edge<usize, (), u64>;

// Standard library's BinaryHeap is a max-heap, so we need to reverse the
// ordering of the edge weights to get a min-heap using the Reverse wrapper.
type Heap = BinaryHeap<Reverse<E>>;

fn prim_minimum_spanning_tree(s: &N) -> Vec<E> {
    // We collect the resulting MST edges in to a vector.
    let mut mst: Vec<E> = vec![];

    // We use a HashSet to keep track of the nodes that are in the MST.
    let mut in_mst: HashSet<usize> = HashSet::new();

    // We use a BinaryHeap to keep track of all edges sorted by weight.
    let mut heap = Heap::new();

    in_mst.insert(*s.key());

    // Collect all edges reachable from `s` to a Min Heap.
    s.bfs()
        .for_each(&mut |edge| {
            heap.push(Reverse(edge.clone()));
        })
        .search();

    // When we pop from the min heap, we know that the edge is the cheapest
    // edge to add to the MST, but we need to make sure that the edge
    // connecting to a node that is not already in the MST, otherwise we
    // we store the edge and continue to the next iteration. When we find
    // an edge that connects to a node that is not in the MST, we add the
    // stored edges back to the heap.
    let mut tmp: Vec<E> = vec![];

    // While the heap is not empty, search for the next edge
    // that connects a node in the tree to a node not in the tree.
    while let Some(Reverse(edge)) = heap.pop() {
        let Edge(u, v, _) = edge.clone();

        // If the edge's source node `u` is in the MST...
        if in_mst.contains(u.key()) {
            // ...and the edge's destination node `v` is not in the MST,
            // then we add the edge to the MST and add all edges
            // in `tmp` back to the heap.
            if in_mst.contains(v.key()) == false {
                in_mst.insert(*v.key());
                mst.push(edge.clone());
                for tmp_edge in &tmp {
                    heap.push(Reverse(tmp_edge.clone()));
                }
            }
        } else {
            // The edge is the cheapest edge to add to the MST, but
            // it's source node `u` nor it's destination node `v` are
            // in the MST, so we store the edge and continue to the next
            // iteration.
            if in_mst.contains(v.key()) == false {
                tmp.push(edge);
            }
        }

        // If neither condition is met, then the edge's destination node
        // `v` is already in the MST, so we continue to the next iteration.
    }
    mst
}

fn main() {
    // Example g1 from Wikipedia
    let g1 = ungraph![
        (usize) => [u64]
        (0) => [ (1, 1), (3, 4), (4, 3)]
        (1) => [ (3, 4), (4, 2)]
        (2) => [ (4, 4), (5, 5)]
        (3) => [ (4, 4)]
        (4) => [ (5, 7)]
        (5) => []
    ];
    let forest = prim_minimum_spanning_tree(&g1[0]);
    let sum = forest.iter().fold(0, |acc, e| acc + e.2);
    assert!(sum == 16);

    // Example g2 from Figure 7.1 in https://jeffe.cs.illinois.edu/teaching/algorithms/book/07-mst.pdf
    let g2 = ungraph![
        (usize) => [u64]
        (0) => [ (1, 8), (2, 5)]
        (1) => [ (2, 10), (3, 2), (4, 18)]
        (2) => [ (3, 3), (5, 16)]
        (3) => [ (4, 12), (5, 30)]
        (4) => [ (6, 4)]
        (5) => [ (6, 26)]
        (6) => []
    ];
    let forest = prim_minimum_spanning_tree(&g2[0]);
    let sum = forest.iter().fold(0, |acc, e| acc + e.2);
    assert!(sum == 42);
}
