// use gdsl::digraph::*;
use gdsl::*;
use gdsl::digraph::*;

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
		(usize, &str) => [f64]
		(0, "blue") => [(1, 0.1), (2, 0.4), (3, 0.8)]
		(1, "red") => [(3, 0.16), (4, 0.32)]
		(2, "green") => [(4, 0.64)]
		(3, "yellow") => [(2, 0.12), (0, 0.24)]
		(4, "cyan") => []
	];

	let dot_str = g.to_dot_with_attr(
		&|_| {
			Some(vec![
				attr("bgcolor", THEME[0]),
				attr("fontcolor", THEME[4]),
				attr("label", "Flow Graph"),
			])
		},
		&|node| {
			Some(vec![
				attr("fillcolor", THEME[1]),
				attr("fontcolor", THEME[4]),
				attr("label", &format!("{}", node.key())),
			])
		},
		&|edge| {
			Some(vec![
				attr("fontcolor", THEME[4]),
				attr("label", &edge.to_string()),
				attr("color", THEME[4]),
			])
		}
	);

	println!("{}", dot_str);
}

// pub const THEME: [&str; 5] = [
// 	"#ffffff", // 0. background
// 	"#ffe5a9", // 1. medium
// 	"#423f3b", // 2. dark
// 	"#ff6666", // 3. accent
// 	"#525266", // 4. theme
// ];

// fn print_graph(g: Graph<K, N, E>) {
// 	let dot_str = g.to_dot_with_attr(
// 		&|| {
// 			Some(vec![
// 				attr("bgcolor", THEME[0]),
// 				attr("fontcolor", THEME[4]),
// 				attr("label", "Flow Graph"),
// 			])
// 		 },
// 		&|node| {
// 			Some(vec![
// 				attr("fillcolor", THEME[1]),
// 				attr("fontcolor", THEME[4]),
// 				attr("label", &format!("{}", node.key())),
// 			])
// 		},
// 		&|_, _, edge| {
// 			let Flow (max, cur) = edge.0.get();
// 			let flow_str = format!("{}/{}", cur, max);
// 			let color = if cur == 0 { THEME[4] } else { THEME[3] };
// 			Some(vec![
// 				attr("fontcolor", THEME[4]),
// 				attr("label", &flow_str),
// 				attr("color", &color),
// 			])
// 		 }
// 	);

// 	println!("{}", dot_str);
// }