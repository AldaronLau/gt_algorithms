extern crate rayon;

// use rayon::prelude::*;

pub struct Edge {
	pub adjacents: Vec<usize>,
}

pub struct Graph {
	pub edges: Vec<Edge>
}

impl Edge {
	pub fn count_for_k(subscript: usize) -> usize {
		let degree = subscript - 1;
		(subscript * degree) / 2
	}
}

impl Graph {
	pub fn complete(size: usize) -> Graph {
		let mut edges = Vec::new();
		let mut adjacents = Vec::new();
	
		adjacents.push(1);
		adjacents.push(2);
		edges.push(Edge { adjacents: adjacents });
		Graph { edges: edges }
	}
}

//impl rayon::iter::IntoParallelRefIterator for Graph;

#[cfg(test)]
mod tests {
//	use rayon::iter::IntoParallelRefIterator;
//	use Graph;

	#[test]
	fn test() { }
}
