//! Split to tuple at n

include!("../gen/split_to_tuple_at.rs");

#[test]
fn test() {
    let t = (1, 2, 3, 4, 5, 6);
    let a = t.split_to_tuple_at_1();
    assert_eq!(a, ((1,), (2, 3, 4, 5, 6)));
    let b = t.split_to_tuple_at_3();
    assert_eq!(b, ((1, 2, 3), (4, 5, 6)));
    let c = t.split_to_tuple_at_5();
    assert_eq!(c, ((1, 2, 3, 4, 5), (6,)));
}
