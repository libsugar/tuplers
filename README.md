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
- Transpose
- Combin
- Mapping
- Iter
- IntoIter
- FromIter
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
- transpose
    ```rust
    let a = Some((1, 2, 3)).transpose();
    assert_eq!(a, (Some(1), Some(2), Some(3)));
    ```
    ```rust
    let b = (Some(1), Some(2), Some(3)).transpose();
    assert_eq!(b, Some((1, 2, 3)));
    ```