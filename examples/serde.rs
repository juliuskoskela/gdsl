// Graph structures implement serialization and deserialization using serde.
// This allows you to serialize and deserialize graphs to and from
// JSON and CBOR as well as other formats. The following example shows how
// to serialize and deserialize a graph to JSON and CBOR and illustrate's
// the difference in the final size of the serialized data.

use gdsl::{*, digraph::*};

fn main() {
	let graph = digraph![
        (&str) => [i32]
        ("A") => [("B", 42), ("C", 42)]
        ("B") => [("C", 42)]
        ("C") => [("D", 42)]
        ("D") => []
    ];

	let cbor = serde_cbor::to_vec(&graph).unwrap();
	let json = serde_json::to_vec(&graph).unwrap();

	// CBOR's binary representation result's in a smaller size than JSON's
	assert!(cbor.len() == 47);
	assert!(json.len() == 101);

	let graph_cbor: Graph<&str, (), i32> = serde_cbor::from_slice(&cbor).unwrap();
	let graph_json: Graph<&str, (), i32> = serde_json::from_slice(&json).unwrap();

	// Since the `Graph` container is a HashMap with a non-deterministic
	// order, we need to sort the nodes before comparing them.
	let mut graph_cbor_vec = graph_cbor.to_vec();
	let mut graph_json_vec = graph_json.to_vec();
	graph_cbor_vec.sort_by(|a, b| a.key().cmp(b.key()));
	graph_json_vec.sort_by(|a, b| a.key().cmp(b.key()));

	for (a, b) in graph_cbor_vec.iter().zip(graph_json_vec.iter()) {
		assert!(a == b);
		for ((u, v, e), (uu, vv, ee)) in a.iter_out().zip(b.iter_out()) {
			assert!(u == uu);
			assert!(v == vv);
			assert!(e == ee);
		}
	}
}