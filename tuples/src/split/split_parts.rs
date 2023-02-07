//! Split into n parts

include!("../gen/split_parts.rs");

#[test]
fn test() {
    let t = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    let a = t.split_2_parts();
    assert_eq!(a, ((0, 1, 2, 3, 4), (5, 6, 7, 8, 9)));
    let b = t.split_3_parts();
    assert_eq!(b, ((0, 1, 2, 3), (4, 5, 6), (7, 8, 9)));
    let c = t.split_4_parts();
    assert_eq!(c, ((0, 1, 2), (3, 4, 5), (6, 7), (8, 9)));
    let d = t.split_5_parts();
    assert_eq!(d, ((0, 1), (2, 3), (4, 5), (6, 7), (8, 9)));
}
