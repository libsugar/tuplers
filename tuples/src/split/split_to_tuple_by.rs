//! Split to tuple by n

include!("../gen/split_to_tuple_by.rs");

#[test]
fn test() {
    let t = (1, 2, 3, 4, 5, 6);
    let t2 = (1, 2, 3, 4, 5);
    let a = t.split_to_tuple_by_2();
    assert_eq!(a, ((1, 2), (3, 4), (5, 6)));
    let b = t2.split_to_tuple_by_2();
    assert_eq!(b, ((1, 2), (3, 4), (5,)));
    let c = t.split_to_tuple_by_3();
    assert_eq!(c, ((1, 2, 3), (4, 5, 6)));
    let d = t2.split_to_tuple_by_3();
    assert_eq!(d, ((1, 2, 3), (4, 5)));
    let e = t.split_to_tuple_by_6();
    assert_eq!(e, (((1, 2, 3, 4, 5, 6),)));
}
