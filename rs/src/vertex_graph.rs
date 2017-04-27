use rayon::prelude::*;

use Vertex;
use UNCOLORED;

pub struct VertexGraph {
	pub vertices: Vec<Vertex>,
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

			Vertex { adjacents: adjacents, color: UNCOLORED }
		}).collect();

		VertexGraph { vertices: vertices }
	}
}
