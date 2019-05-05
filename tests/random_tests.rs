pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_bad_add() {
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    assert_ne!(bad_add(1, 2), 3);
}
