use gt_algorithms::*;

#[test]
fn graph_consistency() {
    let mut graph = Graph::from_vertex_count(MAX_GRAPH_VERTICES);
    for i in 0..MAX_GRAPH_VERTICES {
        dbg!(i);

        assert_eq!(graph.get(i), FALSE);
        graph.set_one(i);
        assert_eq!(graph.get(i), TRUE);
        graph.set_zero(i);
        assert_eq!(graph.get(i), FALSE);
    }
}

#[test]
fn graph_index() {
    assert_eq!(Graph::get_index(1, 0), 0);
    assert_eq!(Graph::get_index(2, 0), 1);
    assert_eq!(Graph::get_index(2, 1), 2);
    assert_eq!(Graph::get_index(3, 0), 3);
    assert_eq!(Graph::get_index(3, 1), 4);
    assert_eq!(Graph::get_index(3, 2), 5);
    assert_eq!(Graph::get_index(4, 0), 6);
    assert_eq!(Graph::get_index(4, 1), 7);
    assert_eq!(Graph::get_index(4, 2), 8);
    assert_eq!(Graph::get_index(4, 3), 9);

    assert_eq!(Graph::get_index(0, 1), 0);
    assert_eq!(Graph::get_index(0, 2), 1);
    assert_eq!(Graph::get_index(1, 2), 2);
    assert_eq!(Graph::get_index(0, 3), 3);
    assert_eq!(Graph::get_index(1, 3), 4);
    assert_eq!(Graph::get_index(2, 3), 5);
    assert_eq!(Graph::get_index(0, 4), 6);
    assert_eq!(Graph::get_index(1, 4), 7);
    assert_eq!(Graph::get_index(2, 4), 8);
    assert_eq!(Graph::get_index(3, 4), 9);
}

#[test]
fn next_check() {
    let mut which_vertices = [false; 6];
    let mut count = 0;
    for _i in 0..3 {
        add(&mut which_vertices);
    }

    loop {
        println!("{:?}", which_vertices);
        count += 1;
        if next(&mut which_vertices) {
            break;
        }
    }

    // 6 choose 3 = 20
    assert_eq!(20, count);
}
