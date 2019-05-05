use crate::*;

/// Calculate a singular Ramsey Number: `R(r, s)`.
pub fn ramsey(r: usize, s: usize) -> usize {
    let mut nvertices = r.min(s);
    let mut graph;
    let mut rcliques;
    let mut scliques;

    'sizes: loop {
        // let edges = (nvertices * (nvertices - 1)) >> 1;
        // dbg!((nvertices, edges));
        // dbg!(edges);

        // Build a Graph and The possible rcliques and scliques.
        graph = Graph::from_vertex_count(nvertices);
        rcliques = graph.find_possible_cliques(r);
        scliques = graph.find_possible_cliques(s);
        assert_eq!(nvertices, graph.n_vertices());

        'colorings: loop {
            // dbg!((edges, nvertices, run));

            if graph.find_cliques(&rcliques, &scliques) == (false, false) {
                // We have found a variant of the graph that does not have either r or b clique cs.
                // So now we have to increase v;
                nvertices += 1;

                break 'colorings;
            }
            if graph.increment() {
                // At this point we have failed to find a graph that doesn't satisfy the properties.
                // We have found the Ramsey number.
                return nvertices;
            }
        }
    }
}
