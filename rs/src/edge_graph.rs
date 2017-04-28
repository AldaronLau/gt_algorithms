use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

use VertexGraph;
use Edge;
use UNCOLORED;

pub struct EdgeGraph {
	pub edges: Vec<Edge>,
}

// Edge represented by vertices
#[derive(PartialEq)]
struct VertexEdge {
	vertices: (usize, usize),
}

impl VertexEdge {
	fn has_vertex(&self, vertex: usize) -> bool {
		return self.vertices.0 == vertex || self.vertices.1 == vertex;
	}

	fn is_adjacent(&self, ve: &VertexEdge) -> bool {
		return self.has_vertex(ve.vertices.0) ||
			self.has_vertex(ve.vertices.1);
	}
}

impl EdgeGraph {
	pub fn complete(size: usize) -> EdgeGraph {
		let vg = VertexGraph::complete(size);
		let mut ves = Vec::new();
		let mut edges = Vec::new();

		for i in 0..vg.vertices.len() {
			let vertices = &vg.vertices[i];
			for j in &vertices.adjacents {
				let edge = VertexEdge { vertices: (i, *j) };
				let edge2 = VertexEdge { vertices: (*j, i) };

				if !ves.contains(&edge2) {
					ves.push(edge);
				}
			}
		}
		for i in 0..ves.len() {
			edges.push(Edge { adjacents: Vec::new(), color: UNCOLORED });

			for j in 0..ves.len() {
				if ves[i].is_adjacent(&ves[j]) && j != i {
					edges[i].adjacents.push(j);
				}
			}
		}

		EdgeGraph { edges: edges }
	}

	pub fn export(&self) -> String {
		let a = self.edges.par_iter().map(|a| {
			let color = if a.color == UNCOLORED { 'a' } else { a.color };
			let mut string : String = format!("{}:", color);
			let len = a.adjacents.len() - 1;
			for i in 0..len {
				string.push_str(format!("{} ", a.adjacents[i]).as_str());
			}
			string.push_str(format!("{},", a.adjacents[len]).as_str());
			string
		});
		let b : Vec<String> = a.collect();
		let mut c = "".to_string();
		for i in b {
			c.push_str(i.as_str());
		}
		c.pop();
		c
	}
}
