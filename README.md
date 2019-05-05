# Graph Theory Algorithms ( gt_algorithms )
Graph theory algorithms in Rust.

## Graph Format
The graph format stores a list of colors.  The file is a sequence of characters: `'1'` for `RED`, and `'0'` for `BLUE`.  Take for instance, a 4-vertex graph with vertices 0, 1, 2 and 3.  The order is specified below.
```
1~0 // Add 2nd Vertex (1 edge)
2~0 // Add 3rd Vertex (2 edges)
2~1
3~0 // Add 4th Vertex (3 edges)
3~1
3~2
```

Example coloring file for K4:
```
110010
```
