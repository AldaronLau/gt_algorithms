# Graph Theory Algorithms ( gt_algorithms )
Graph theory algorithms in Rust.

## Edge File Format
Edges are listed in order starting at index 0.  The first character of an edge defines it's color: a or b.  After the color there is a colon.  Adjacent edges are listed after the colon, separated by spaces.  Edges are separated by commas.

### Example
a:1 2,b:0 2,a:0 1

#### Interpretation
Edge 0: Color A, Adjacent to Edge 1 & 2

Edge 2: Color B, Adjacent to Edge 0 & 2

Edge 3: Color A, Adjacent to Edge 0 & 1
