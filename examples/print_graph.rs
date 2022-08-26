// use gdsl::digraph::*;
use gdsl::*;
use std::cell::Cell;

fn attr(field: &str, value: &str) -> (String, String) {
	(field.to_string(), value.to_string())
}

pub const THEME: [&str; 5] = [
	"#ffffff", // 0. background
	"#ffe5a9", // 1. medium
	"#423f3b", // 2. dark
	"#ff6666", // 3. accent
	"#525266", // 4. theme
];

fn main() {
	let g = digraph![
		(usize, Cell<bool>) => [f64]
		(0, Cell::new(false)) => [(1, 0.1), (2, 0.4), (3, 0.8)]
		(1, Cell::new(false)) => [(3, 0.16), (4, 0.32)]
		(2, Cell::new(false)) => [(4, 0.64)]
		(3, Cell::new(false)) => [(2, 0.12), (0, 0.24)]
		(4, Cell::new(false)) => []
	];

	g[0].bfs()
		.target(&4)
		.search_path()
		.unwrap()
		.iter_nodes()
		.for_each(|n| n.set(true));

	let dot_str = g.to_dot_with_attr(
		&|graph| {
			let graph_size_mb = graph.sizeof() as f64 / 1024.0 / 1024.0;
			Some(vec![
				attr("bgcolor", THEME[0]),
				attr("fontcolor", THEME[4]),
				attr("label", &format!("Shortest Path {:.4} Mb", graph_size_mb)),
			])
		},
		&|node| {
			let color = if node.get() { THEME[3] } else { THEME[4] };
			Some(vec![
				attr("color", color),
				attr("fontcolor", THEME[4]),
				attr("label", &format!("{}", node.key())),
			])
		},
		&|u, v, edge| {
			let color = if u.get() && v.get() { THEME[3] } else { THEME[4] };
			Some(vec![
				attr("fontcolor", THEME[4]),
				attr("weight", &edge.to_string()),
				attr("color", color),
			])
		}
	);

	println!("{}", dot_str);
}
