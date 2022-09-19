#![allow(non_snake_case)]
use gdsl::digraph::*;
use gdsl::{digraph_connect as connect, digraph_node as node};
use std::cell::Cell;
use std::sync::Arc;

type N = Node<usize, Cell<f64>, Arc<Cell<f64>>>;
type G = Vec<N>;

pub fn page_rank(g: &G, α: f64, ε: f64) {
    let inverse = 1.0 / g.len() as f64;
    for node in g {
        let sum = node.iter_out().map(|e| e.value().get()).sum::<f64>();
        for Edge(_, _, e) in node {
            e.set(e.get() / sum);
        }
        node.set(inverse);
    }
    let mut Δ = 1.0;
    while Δ > ε {
        let mut nodes = vec![0.0; g.len()];
        let mut leak = 0.0;
        for u in g.iter() {
            nodes[*u.key()] = u.get();
            if u.out_degree() == 0 {
                leak += u.get();
            }
            u.set(0.0);
        }
        leak *= α;
        for u in g {
            for Edge(_, v, e) in u {
                v.set(v.get() + α * nodes[*u.key()] * e.get());
            }
            u.set(u.get() + (1.0 - α) * inverse + leak * inverse);
        }

        Δ = g.iter().map(|u| (u.get() - nodes[*u.key()]).abs()).sum();
    }
}

pub fn create_page_rank_dataset(size: usize, degree: usize) -> G {
    let mut g = Vec::with_capacity(size);
    for i in 0..size {
        g.push(node!(i, Cell::new(0.0)));
    }
    for i in 0..size {
        for j in 0..degree {
            connect!(&g[i] => &g[j], Arc::new(Cell::new(1.0)));
        }
    }
    g
}
