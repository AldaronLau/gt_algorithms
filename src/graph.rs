use crate::*;

/// The maximum number of vertices a Graph can have.
pub const MAX_GRAPH_VERTICES: usize = 64;

/// A bitstring that represents a graph.  No length.
pub struct BitString([u64; 32]);

/// A complete graph colored with red and blue edges.
#[derive(Clone)]
pub struct Graph {
    // Number of vertices in the graph.
    vertices: usize,
    // Number of edges in the graph.
    edges: usize,
    // Bitstring of relations between vertices.
    colors: [u64; 32],
}

impl Graph {
    /// Create a new Red/Blue complete graph.
    #[inline(always)]
    pub fn from_vertex_count(vertices: usize) -> Self {
        Graph {
            vertices,
            edges: (vertices * (vertices - 1)) / 2,
            colors: [0; 32],
        }
    }

    /// Create a new Clique from a list of vertices.
    #[inline(always)]
    pub fn from(vertices: &[bool]) -> Self {
        let vertex_count = vertices.len();

        let mut graph = Graph {
            vertices: vertex_count,
            edges: (vertex_count * (vertex_count - 1)) / 2,
            colors: [0; 32],
        };

        // Build all of the edges.
        for vertex_a in 0..vertices.len() {
            if vertices[vertex_a] == false {
                continue;
            }

            for vertex_b in 0..vertices.len() {
                if vertices[vertex_b] == false || vertex_a == vertex_b {
                    continue;
                }

                if vertices[vertex_a] && vertices[vertex_b] {
                    graph.add(vertex_a, vertex_b);
                }
            }
        }

        graph
    }

    /// Set the color for an edge to blue (0).
    #[inline(always)]
    pub fn set_zero(&mut self, index: usize) {
        debug_assert!(index < self.edges);

        let i = index / 64;
        let j = index % 64;

        //        print!("BFrom {:b}", self.colors[i]);
        self.colors[i] &= !(1 << j);
        //        println!(" to {:b}", self.colors[i]);
    }

    /// Set the color for an edge to red (1).
    #[inline(always)]
    pub fn set_one(&mut self, index: usize) {
        debug_assert!(index < self.edges);

        let i = index / 64;
        let j = index % 64;

        //        print!("RFrom {:b}", self.colors[i]);
        self.colors[i] |= 1 << j;
        //        println!(" to {:b}", self.colors[i]);
    }

    /// Get the color for an edge.  1 for red, 0 for blue.
    #[inline(always)]
    pub fn get(&self, index: usize) -> bool {
        debug_assert!(index < self.edges);

        let i = index / 64;
        let j = index % 64;

        //        println!("Get {:b}", self.colors[i]);
        (self.colors[i] & (1 << j)) != 0
    }

    /// Get the number of edges.
    #[inline(always)]
    pub fn n_edges(&self) -> usize {
        self.edges
    }

    /// Get the number of vertices.
    #[inline(always)]
    pub fn n_vertices(&self) -> usize {
        //        assert!(is_triangular(self.edges));
        //        triangle_root(self.edges)
        self.vertices
    }

    /// Get the index of an edge from two vertices.
    #[inline(always)]
    pub fn get_index(first: usize, second: usize) -> usize {
        if first > second {
            triangle_num(first - 1) + second
        } else {
            triangle_num(second - 1) + first
        }
    }

    /// Get the relation between two vertices.
    #[inline(always)]
    pub fn relation(&self, first: usize, second: usize) -> bool {
        self.get(Graph::get_index(first, second))
    }

    /// Add an edge between two vertices.
    #[inline(always)]
    pub fn add(&mut self, first: usize, second: usize) {
        self.set_one(Graph::get_index(first, second));
    }

    /// Find the next variation of the graph.
    #[inline(always)]
    pub fn increment(&mut self) -> bool {
        let mut digit = 0;

        // Go through all of the edges.
        loop {
            // Can no longer get or set, so we've hit the last iteration.
            if digit == self.n_edges() {
                break true;
            }

            if self.get(digit) {
                self.set_zero(digit);
                digit += 1;
            } else {
                self.set_one(digit);
                break false;
            }
        }
    }

    /// Get all of the possible cliques with `n` vertices on a complete graph of this size.
    #[inline(always)]
    pub fn find_possible_cliques(&self, n: usize) -> Vec<BitString> {
        // There are not enough vertices in the graph to have a clique of this size.
        if n > self.n_vertices() {
            return vec![];
        }

//        dbg!(self.n_vertices());

        let mut returnv = vec![];
        let mut current = vec![false; self.n_vertices()];

        for _i in 0..n {
            add(current.as_mut_slice());
        }

        // at this point current looks something like 111000

        // println!("Enter Loop.");
        loop {
            // dbg!(&current);

            let graph = Self::from(&current);

            // println!("OGC {:b}", graph.colors[0]);

            returnv.push(BitString(graph.colors));

            if next(&mut current) {
                break;
            }
        }
        // println!("Exit Loop.");

        returnv
    }

    /// Check for cliques of size r and b.  When using the function, start n at the size of the
    /// graph and decrease it.  `bitstrings` must be a result from `find_possible_cliques()` with
    /// the same value for `n`.
    #[inline(always)]
    pub fn find_cliques(&self, prcs: &Vec<BitString>, pbcs: &Vec<BitString>) -> (bool, bool) {
/*        println!(
            "K{} Searching for {} possible red clique(s) and for {} possible blue clique(s)",
            self.vertices,
            prcs.len(),
            pbcs.len()
        );*/

        let gc = self.colors;

        let mut has_red = false;
        let mut has_blue = false;

        // Check for RED Cliques of size r.
        for pc in prcs {
            let c = simd_and(pc.0, gc, self.edges);
            if simd_eq(c, pc.0, self.edges) {
                // We have a RED Clique of 3 Vertices.
                //println!("Found a Red Clique!");
                has_red = true;
                break;
            }
        }

        for pc in pbcs {
            let c = simd_and(pc.0, gc, self.edges);
            if simd_is_zero(c, self.edges) {
                //println!("Found a Blue Clique!");
                // We have a BLUE Clique of 3 Vertices.
                has_blue = true;
                break;
            }
        }

        //        dbg!((has_red, has_blue));

        (has_red, has_blue)
    }
}

/// Add a vertex to a clique.
pub fn add(current: &mut [bool]) {
    for i in 0..current.len() {
        if current[i] {
            continue;
        }
        current[i] = true;
        break;
    }
}

/// Calculate the next possible selection of vertices to form a clique.
pub fn next(which_vertices: &mut [bool]) -> bool {
    // Start at the end of the list of vertices.
    let mut index = which_vertices.len();

    let mut found_zero = false;

    loop {
        index -= 1;
        if which_vertices[index] == false {
            found_zero = true;
        }
        // The one after a zero.
        if found_zero && which_vertices[index] {
            // Swap 1 (move the 1 to the right).
            which_vertices.swap(index, index + 1);
            // Count 1s after the 1.
            let mut count_index = index + 1;
            let mut count = 0;
            'counter: loop {
                count_index += 1;
                if count_index == which_vertices.len() {
                    break 'counter;
                }
                if which_vertices[count_index] {
                    count += 1;
                }
            }
            // Move all of the 1 to be consecutive after this 1.
            let mut write_index = index + 1;
            'writer: loop {
                write_index += 1;
                if write_index == which_vertices.len() {
                    break 'writer;
                }
                if count != 0 {
                    count -= 1;
                    which_vertices[write_index] = true;
                } else {
                    which_vertices[write_index] = false;
                }
            }
            break false;
        }
        if index == 0 {
            // We have found the last variant.
            break true;
        }
    }
}

/// Get the nth trianglular number.
pub fn triangle_num(n: usize) -> usize {
    (n * (n + 1)) / 2
}
