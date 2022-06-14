use tuples::*;

#[test]
fn test() {
    let a = (1, 2, 3);
    let b = a.map(|v| v * 3);
    assert_eq!(b, (3, 6, 9));
}

#[test]
fn test_n() {
    let a = (1, 2, 3, 4, 5);
    let b = a.map3(|v| v * 5);
    assert_eq!(b, (1, 2, 3, 20, 5));
}

#[test]
fn test_all() {
    let a = (1, 2, 3);
    let b = a.map_all(|v| v * 10, |v| v * 100, |v| v * 1000);
    assert_eq!(b, (10, 200, 3000));
}
