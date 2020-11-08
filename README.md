# tuples

![Rust](https://github.com/BatchOperator/tuples/workflows/Rust/badge.svg)
[![version](https://img.shields.io/crates/v/tuples)](https://crates.io/crates/tuples)
[![documentation](https://docs.rs/tuples/badge.svg)](https://docs.rs/tuples)
![LICENSE](https://img.shields.io/crates/l/tuples)

Provides many useful tools related to tuples

- AsRef
- AsMut
- AsOption
- AsResult
- AsDeref
- AsDerefMut
- Transpose Option
- Transpose Result
- Cloned
- Copied
- Flatten
- Combin
- Mapping
- Iter
- IntoIter
- FromIter
- Collect
- Meta Trait
- Shorthand Macro

## Examples

- map
    ```rust
    let a = (1, 2, 3);
    let b = a.map(|v| v * 3);
    assert_eq!(b, (3, 6, 9));
    ```
- mapN
    ```rust
    let a = (1, 2, 3, 4, 5);
    let b = a.map3(|v| v * 5);
    assert_eq!(b, (1, 2, 3, 20, 5));
    ```
- as_ref
    ```rust
    let t = (5, 6, 7);
    let (a, b, c) = t.as_ref();
    assert_eq!(*a, 5);
    assert_eq!(*b, 6);
    assert_eq!(*c, 7);
    ```
- cloned
    ```rust
    let a = (&1, &2, &3);
    let b = a.cloned();
    assert_eq!(b, (1, 2, 3))
    ```
- flatten
    ```rust
    let a = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    let b = a.flatten();
    assert_eq!(b, (1, 2, 3, 4, 5, 6, 7, 8, 9));
    ```
- meta
    ```rust
    let a = (1, 2, 3, 4, 5);
    assert_eq!(a.arity(), 5);

    let b = ();
    assert_eq!(b.arity(), 0);
    ```
- iter
    ```rust
    let a = (1, 2, 3)
        .into_iter()
        .map(|v| v * 3)
        .collect_tuple::<tuple![3;]>();
    let b: (i32, i32, i32) = (3, 6, 9);
    assert_eq!(a, b);
    ```
    ```rust
    let a = (1, 2, 3)
        .into_iter()
        .map(|v| v * 3)
        .try_collect_tuple::<tuple![3;]>();
    let b: Option<(i32, i32, i32)> = Some((3, 6, 9));
    assert_eq!(a, b);
    ```
    ```rust
    let a = (1, 2, 3)
        .into_iter()
        .map(|v| v * 3)
        .collect_tuple_try::<tuple![3;]>();
    let b: (Option<i32>, Option<i32>, Option<i32>) = (Some(3), Some(6), Some(9));
    assert_eq!(a, b);
    ```
- transpose
    ```rust
    let a = Some((1, 2, 3)).transpose();
    assert_eq!(a, (Some(1), Some(2), Some(3)));

    let b = (Some(1), Some(2), Some(3)).transpose();
    assert_eq!(b, Some((1, 2, 3)));
    ```
    ```rust
    let a: (Result<u8, ()>, Result<u8, ()>, Result<u8, ()>) = (Ok(1), Ok(2), Ok(3));
    let b: Result<(u8, u8, u8), ()> = a.transpose();
    assert_eq!(b, Ok((1, 2, 3)));
    ```
    ```rust
    let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
    let b: Result<(u8, u8, u8), i64> = a.transpose();
    assert_eq!(b, Err(-1));
    ```
- combin
    ```rust
    let a = (1, 2).push(3);
    assert_eq!(a, (1, 2, 3));

    let b = (2, 1).left(3);
    assert_eq!(b, (3, 2, 1));

    let c = (1, 2, 3).concat((4, 5, 6));
    assert_eq!(c, (1, 2, 3, 4, 5, 6))
    ```
