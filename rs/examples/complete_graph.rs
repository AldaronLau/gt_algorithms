extern crate gt_algorithms;
extern crate rayon;

use gt_algorithms::{ EdgeGraph /*, Edge*/ };
use std::fs::File;
use std::io::prelude::*;

use std::time::{ Instant };

pub fn main() {
	// Time it
	let now = Instant::now();

	let s = 43;
	let cg = EdgeGraph::complete(s);//.edges;
	let graph_file = cg.export();

	println!("{}", graph_file);

	let mut file = File::create(format!("cg{}.text", s)).unwrap();
	file.write_all(graph_file.as_bytes()).unwrap();

/*	let a = cg.par_iter().map(|a| {
		let string : String = format!("{:?}", a.adjacents);
		string
	});
	let b : Vec<String> = a.collect();

	println!("Edges For {} Vertices: {}", s, Edge::count_for_k(s));

	for i in b {
		println!("{}", i);
	}*/


/*	for i in 0..10 {
		let j = i + 1;
		println!("Edges For {} Vertices: {}", j, Edge::count_for_k(j));
	}*/


	// Generate & Print Complete Graph
//	let size = 43;

	// Vertex Graph
/*	let vertexgraph = VertexGraph::complete(size).vertices;
	for i in 0..size {
		println!("Vertices: {:?}", vertexgraph[i].adjacents);
	}*/

	// Edge Graph
/*	let edgegraph = EdgeGraph::complete(size).edges;
	for i in 0..size {
		println!("Edges: {:?}", edgegraph[i].adjacents);
	}*/

	let new = ( now.elapsed().subsec_nanos() as f32 ) / 1_000_000_000.0;
	println!("Done in {}.", new);
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
