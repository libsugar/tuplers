use tuples::*;

#[test]
fn test() {
    let a: tuple![u8; 3] = (5, 5, 5);
    let b: (u8, u8, u8) = tuple![5; 3];
    assert_eq!(a, b);
}

#[test]
fn test2() {
    let a: tuple![3; u8] = (5, 5, 5usize);
    let b: (u8, i32, usize) = tuple![5; 3];
    assert_eq!(a, b);
}

#[test]
fn test3() {
    let a = (1, 2, 3).into_iter().map(|v| v * 3).collect_tuple::<tuple![3;]>();
    let b: (i32, i32, i32) = (3, 6, 9);
    assert_eq!(a, b);
}
