#![allow(non_snake_case)]
use gdsl::digraph::*;
use gdsl::{digraph_connect as connect, digraph_node as node};
use std::cell::Cell;
use std::rc::Rc;

type N = Node<usize, Cell<f64>, Rc<Cell<f64>>>;
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
	let mut nodes = vec![0.0; g.len()];
    while Δ > ε {
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
		nodes.iter_mut().for_each(|x| *x = 0.0);
    }
}

pub fn create_page_rank_dataset(size: usize, avg_dgr: usize, min_dgr: usize, max_dgr: usize) -> G {
    let mut g = Vec::with_capacity(size);
	for i in 0..size {
		g.push(node!(i, Cell::new(0.0)));
	}
	for u in g.iter() {
		// Generate a random numer from a distribution over [min_dgr, max_dgr] with
		// an average of avg_dgr.
		let dgr = (min_dgr as f64 + (max_dgr - min_dgr) as f64 * (avg_dgr as f64 / max_dgr as f64).ln().exp())
			.round() as usize;
		for _ in 0..dgr {
			let v = g[rand::random::<usize>() % size].clone();
			connect!(u => &v, Rc::new(Cell::new(1.0)));
		}
	}
    g
}
