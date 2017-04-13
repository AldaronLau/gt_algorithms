extern crate gt_algorithms;
extern crate rayon;

use gt_algorithms::{ VertexGraph, EdgeGraph, Edge };
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

pub fn main() {
	let cg = EdgeGraph::complete(3).edges;
	let a = cg.par_iter().map(|a| {
		let string : String = format!("{:?}", a.adjacents);
		string
	});
	let b : Vec<String> = a.collect();
	for i in b {
		println!("{}", i);
	}
	for i in 0..10 {
		let j = i + 1;
		println!("Edges For {} Vertices: {}", j, Edge::count_for_k(j));
	}

	let size = 8;
	let vertexgraph = VertexGraph::complete(size).vertices;
	for i in 0..size {
		println!("Vertices: {:?}", vertexgraph[i].adjacents );
	}
//		.reduce(
//			|a| { println!("aa {:?}", a.adjacents); (0, 0) },
//			|a, b| {});
//.map(|i| {
//		println!("aa {:?}", i.adjacents);
//	});
//	it.fold(0, 1);
//	for i in it {
//	}

/*	let bytes = 0..22_u8; // series of u8 bytes
	let sum = bytes.into_par_iter()
		.fold(|| 0_u32, |a: u32, b: u8| a + (b as u32))
		.sum::<u32>();
	println!("Sum = {}", sum);*/
//	assert_eq!(sum, (0..22).sum()); // compare to sequential
}
