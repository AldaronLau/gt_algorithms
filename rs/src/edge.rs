pub struct Edge {
	pub adjacents: Vec<usize>,
	pub color: char,
}

impl Edge {
	pub fn count_for_k(subscript: usize) -> usize {
		let degree = subscript - 1;
		(subscript * degree) / 2
	}
}
