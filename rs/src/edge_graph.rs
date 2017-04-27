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
}
