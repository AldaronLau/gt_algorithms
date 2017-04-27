extern crate rayon;
extern crate gt_algorithms;

use gt_algorithms::*;
use rayon::prelude::*;

fn algorithm_a(a: usize, size: usize) -> Vertex {
	let mut adjacents = Vec::new();
	for i in 0..size {
		if i != a {
			adjacents.push(i);
		}
	}
	// Return Generated Variable
	Vertex { adjacents: adjacents }
}

fn algorithm_b(a: usize, size: usize) -> Vertex {
	let mut adjacents = Vec::new();
	for i in 0..a {
		adjacents.push(i);
	}
	for i in (a+1)..size {
		adjacents.push(i);
	}
	// Return Generated Variable
	Vertex { adjacents: adjacents }
}

fn complete(size: usize) -> VertexGraph {
	let vertices = (0..size).into_par_iter().map(|a| algorithm_a(a, size))
		.collect();
	VertexGraph { vertices: vertices }
}

fn main() {
	let a = complete(10);
	for i in a.vertices {
		println!("aa {:?}", i.adjacents);
	}
}
