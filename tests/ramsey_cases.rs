// TODO: Check to see if R(1, x) = 1, R(2, x) = x (up to x=6), and R(3, 3) = 6.  Any more tests
// would take to much time.

use gt_algorithms::*;

#[test]
fn r22() {
    assert_eq!(ramsey(2, 2), 2);
}

#[test]
fn r23() {
    assert_eq!(ramsey(2, 3), 3);
}

#[test]
fn r24() {
    assert_eq!(ramsey(2, 4), 4);
}

#[test]
fn r25() {
    assert_eq!(ramsey(2, 5), 5);
}

#[test]
fn r3() {
    assert_eq!(ramsey(3, 3), 6);
}
