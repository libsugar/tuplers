use tuples::*;

#[test]
fn test() {
    let a = (1, 2, 3, 4, 5);
    assert_eq!(a.arity(), 5);
}

#[test]
fn test_2() {
    let a = ();
    assert_eq!(a.arity(), 0);
}
