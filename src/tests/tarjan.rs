
#[test]
fn tarjan() {
	use crate::*;
	use std::cell::RefCell;

	type LLval = RefCell<u64>;

	let g = graph![(&str, LLval)
		("A", LLval::new(0)) => ["B", "E"]
		("B", LLval::new(0)) => ["F"]
		("C", LLval::new(0)) => ["B", "D", "G"]
		("D", LLval::new(0)) => ["G"]
		("E", LLval::new(0)) => ["A", "F"]
		("F", LLval::new(0)) => ["C", "G"]
		("G", LLval::new(0)) => ["H"]
		("H", LLval::new(0)) => ["D"]
	];

}