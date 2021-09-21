

pub struct Amat {
	nodes: Vec<u64>,
	matrix: Vec<bool>,
}

impl Amat {
	pub fn new() -> Self {
		Self {
			nodes: Vec::new(),
			matrix: Vec::new(),
		}
	}
}

// struct Matrix {
// 	body
// }