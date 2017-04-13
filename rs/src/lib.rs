extern crate rayon;

use rayon::prelude::*;
// use rayon::iter::IntoParallelRefIterator;

pub struct Vertex {
	pub adjacents: Vec<usize>,
}

pub struct Edge {
	pub adjacents: Vec<usize>,
}

pub struct VertexGraph {
	pub vertices: Vec<Vertex>,
}

pub struct EdgeGraph {
	pub edges: Vec<Edge>,
}

impl Edge {
	pub fn count_for_k(subscript: usize) -> usize {
		let degree = subscript - 1;
		(subscript * degree) / 2
	}
}

impl VertexGraph {
	pub fn complete(size: usize) -> VertexGraph {
//		let s = Edge::count_for_k(size);
//		let v = ;
		let vertices = (0..size).into_par_iter().map(|a| {
			let mut adjacents = Vec::new();
			// TODO: Figure out which algorithm is faster.

			// Algorithm A
//			for i in 0..size {
//				if i != a {
//					adjacents.push(i);
//				}
//			}

			// Algorithm B
			for i in 0..a {
				adjacents.push(i);
			}
			for i in (a+1)..size {
				adjacents.push(i);
			}

			Vertex { adjacents: adjacents }
		}).collect();

		VertexGraph { vertices: vertices }
	}
}

impl EdgeGraph {
	pub fn complete(size: usize) -> EdgeGraph {
		let mut edges = Vec::new();
		let mut adjacents = Vec::new();
	
		adjacents.push(1);
		adjacents.push(2);
		edges.push(Edge { adjacents: adjacents });
		EdgeGraph { edges: edges }
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
