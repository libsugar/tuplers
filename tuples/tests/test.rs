use tuples::*;

#[test]
fn test_as_ref() {
    let t = (5, 6, 7);
    let (a, b, c) = t.as_ref();
    assert_eq!(*a, 5);
    assert_eq!(*b, 6);
    assert_eq!(*c, 7);
}
