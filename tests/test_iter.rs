use tuples::*;

#[test]
fn test_try_1() {
    let a = (1, 2, 3)
        .into_iter()
        .map(|v| v * 3)
        .try_collect_tuple::<tuple![3;]>();
    let b: Option<(i32, i32, i32)> = Some((3, 6, 9));
    assert_eq!(a, b);
}

#[test]
fn test_try_2() {
    let a = (1, 2, 3)
        .into_iter()
        .map(|v| v * 3)
        .collect_tuple_try::<tuple![3;]>();
    let b: (Option<i32>, Option<i32>, Option<i32>) = (Some(3), Some(6), Some(9));
    assert_eq!(a, b);
}
