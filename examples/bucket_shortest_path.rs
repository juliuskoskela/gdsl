// foreach v ∈ V do tent(v) := ∞
//
// relax(s, 0);									(* Insert source node with distance 0 *)
// while ¬isEmpty(B) do							(* A phase: Some queued nodes left (a) *)
//   i := min{j >= 0: B[j] != ∅}				(* Smallest nonempty bucket (b) *)
//   R := ∅										(* No nodes deleted for bucket B[i] yet *)
//   while B[i] != = ∅ do						(* New phase (c) *)
//     Req := findRequests(B[i], light)			(* Create requests for light edges (d) *)
//     R := R ∪ B[i]							(* Remember deleted nodes (e) *)
//     B[i] := ∅								(* Current bucket empty *)
//     relaxRequests(Req)						(* Do relaxations, nodes may (re)enter B[i] (f) *)
// Req := findRequests(R, heavy)				(* Create requests for heavy edges (g) *)
// relaxRequests(Req)							(* Relaxations will not refill B[i] (h) *)
//
// Function findRequests(V′, kind : {light, heavy}) : set of Request
//   return {(w, tent(v) + c(v, w)): v ∈ V′ ∧ (v, w) ∈ Ekind)}
//
// Procedure relaxRequests(Req)
//   foreach (w, x) ∈ Req do relax(w, x)
//
// Procedure relax(w, x)						(* Insert or move w in B if x < tent(w) *)
//   if x < tent(w) then
//     B[{tent(w)/∆}] := B[{tent(w)/∆}] \ {w}	(* If in, remove from old bucket *)
//     B[{x /∆}] := B[{x /∆}] ∪{w}				(* Insert into new bucket *)
//     tent(w) := x

use gdsl::digraph::*;
use gdsl::*;
use std::cell::Cell;
use ahash::HashMap;

type N = Node<char, Cell<u64>, u64>;
type E = Edge<char, Cell<u64>, u64>;

const DELTA: usize = 3;

enum EdgeKind {
	Light,
	Heavy,
}

fn relax_requests(requests: Vec<E>, buckets: &mut Vec<HashMap<char, N>>) {
	for (u, v, e) in requests {
		relax(v, u.get() + e, buckets);
	}
}

fn find_requests(bucket: &HashMap<char, N>, kind: EdgeKind) -> Vec<E> {
	let mut requests = Vec::new();

	for (_, node) in bucket.iter() {
		for (u, v, e) in node {
			match kind {
				EdgeKind::Light => {
					if e <= DELTA as u64 {
						requests.push((u, v, e));
					}
				}
				EdgeKind::Heavy => {
					if e > DELTA as u64 {
						requests.push((u, v, e));
					}
				}
			}
		}
	}
	requests
}

fn relax(n: N, cur_dist: u64, buckets: &mut Vec<HashMap<char, N>>) {
    let tent_dist = n.get();
    if cur_dist < tent_dist {
        let old_bucket = tent_dist / DELTA as u64;
        let new_bucket = cur_dist / DELTA as u64;
        if old_bucket != new_bucket {
			if old_bucket < buckets.len() as u64 {
            	buckets[old_bucket as usize].remove(n.key());
			}
            buckets[new_bucket as usize].insert(*n.key(), n.clone());
        }
        n.set(cur_dist);
    }
}

fn find_min_bucket(buckets: &Vec<HashMap<char, N>>) -> Option<usize> {
	let mut min_bucket = None;
	for (i, bucket) in buckets.iter().enumerate() {
		if bucket.len() > 0 {
			min_bucket = Some(i);
			break;
		}
	}
	min_bucket
}

fn seq_delta_stepping(s: &N) {
	let mut buckets = vec![HashMap::default(); 1000];

	buckets[0].insert(*s.key(), s.clone());

	while let Some(i) = find_min_bucket(&buckets) {
		let mut deleted_nodes = HashMap::default();
		while !buckets[i].is_empty() {
			let requests = find_requests(&buckets[i], EdgeKind::Light);
			for node in buckets[i].values() {
				deleted_nodes.insert(*node.key(), node.clone());
			}
			buckets[i].clear();
			relax_requests(requests, &mut buckets);
		}
		let requests = find_requests(&deleted_nodes, EdgeKind::Heavy);
		relax_requests(requests, &mut buckets);
	}
}

fn main () {
	let g = digraph![
		(char, Cell<u64>) => [u64]
		('A', Cell::new(u64::MAX)) => [ ('B', 4), ('H', 8) ]
		('B', Cell::new(u64::MAX)) => [ ('A', 4), ('H', 11), ('C', 8) ]
		('C', Cell::new(u64::MAX)) => [ ('B', 8), ('C', 2), ('F', 4), ('D', 7) ]
		('D', Cell::new(u64::MAX)) => [ ('C', 7), ('F', 14), ('E', 9) ]
		('E', Cell::new(u64::MAX)) => [ ('D', 9), ('F', 10) ]
		('F', Cell::new(u64::MAX)) => [ ('G', 2), ('C', 4), ('D', 14), ('E', 10) ]
		('G', Cell::new(u64::MAX)) => [ ('H', 1), ('I', 6), ('F', 2) ]
		('H', Cell::new(u64::MAX)) => [ ('A', 8), ('B', 11), ('I', 7), ('G', 1) ]
		('I', Cell::new(u64::MAX)) => [ ('H', 7), ('C', 2), ('G', 6) ]
	];

	g['A'].set(0);

	seq_delta_stepping(&g['A']);

	println!("dist A -> E = {}", g['E'].get());
	// assert!(g['E'].take() == 21);
}