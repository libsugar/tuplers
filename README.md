# tuples

![Rust](https://github.com/libsugar/tuplers/workflows/Rust/badge.svg)
[![version](https://img.shields.io/crates/v/tuples)](https://crates.io/crates/tuples)
[![documentation](https://docs.rs/tuples/badge.svg)](https://docs.rs/tuples)
![LICENSE](https://img.shields.io/crates/l/tuples)

Provides many useful tools related to tuples

- Support no-std
- Support wasm
- Pre-generated, faster compilation
- AsRef
- AsMut
- AsOption
- AsResult
- AsDeref
- AsDerefMut
- TupleGet
- Transpose Option
- Transpose Result
- Cloned
- Copied
- Flatten
- Combin
- Split
- Mapping
- Iter
- IntoIter
- FromIter
- Collect
- Meta Trait
- Shorthand Macro
- Call
- Apply
- Swap
- Sort
- Permutations
- Combinations

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

- map_all

  ```rust
  let a = (1, 2, 3);
  let b = a.map_all(|v| v * 10, |v| v * 100, |v| v * 1000);
  assert_eq!(b, (10, 200, 3000));
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

- get

  ```rust
  let a = (1, 2, 3, 4, 5);
  assert_eq!(*a.get(2), 3);

  let mut a = (1, 2, 3, 4, 5);
  *a.get_mut(3) = 6;
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

  ```rust
  let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
  let b = a.transpose1::<i64>();
  assert_eq!(b, Err(-1));
  ```

- combin

  ```rust
  let a = (1, 2).push_right(3);
  assert_eq!(a, (1, 2, 3));

  let b = (2, 1).push_left(3);
  assert_eq!(b, (3, 2, 1));

  let c = (1, 2, 3).concat((4, 5, 6));
  assert_eq!(c, (1, 2, 3, 4, 5, 6))
  ```

- split

  - split_parts

    ```rust
    let t = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

    let a = t.split_2_parts();
    assert_eq!(a, ((0, 1, 2, 3, 4), (5, 6, 7, 8, 9)));

    let b = t.split_3_parts();
    assert_eq!(b, ((0, 1, 2, 3), (4, 5, 6), (7, 8, 9)));

    let c = t.split_4_parts();
    assert_eq!(c, ((0, 1, 2), (3, 4, 5), (6, 7), (8, 9)));

    let d = t.split_5_parts();
    assert_eq!(d, ((0, 1), (2, 3), (4, 5), (6, 7), (8, 9)));
    ```

  - split_at

    ```rust
    let t = (1, 2, 3, 4, 5, 6);

    let a = t.split_at_1();
    assert_eq!(a, (1, (2, 3, 4, 5, 6)));

    let b = t.split_at_3();
    assert_eq!(b, ((1, 2, 3), (4, 5, 6)));

    let c = t.split_at_5();
    assert_eq!(c, ((1, 2, 3, 4, 5), 6));
    ```

  - split_by

    ```rust
    let t = (1, 2, 3, 4, 5, 6);
    let t2 = (1, 2, 3, 4, 5);

    let a = t.split_by_2();
    assert_eq!(a, ((1, 2), (3, 4), (5, 6)));

    let b = t2.split_by_2();
    assert_eq!(b, ((1, 2), (3, 4), 5));

    let c = t.split_by_3();
    assert_eq!(c, ((1, 2, 3), (4, 5, 6)));

    let d = t2.split_by_3();
    assert_eq!(d, ((1, 2, 3), (4, 5)));

    let e = t.split_by_6();
    assert_eq!(e, ((1, 2, 3, 4, 5, 6)));
    ```

  - split_to_tuple_at

    ```rust
    let t = (1, 2, 3, 4, 5, 6);

    let a = t.split_to_tuple_at_1();
    assert_eq!(a, ((1,), (2, 3, 4, 5, 6)));

    let b = t.split_to_tuple_at_3();
    assert_eq!(b, ((1, 2, 3), (4, 5, 6)));

    let c = t.split_to_tuple_at_5();
    assert_eq!(c, ((1, 2, 3, 4, 5), (6,)));
    ```

  - split_to_tuple_by

    ```rust
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
    ```

- call

  ```rust
  let r = (1, 2, 3).call(|a, b, c| a + b + c);
  assert_eq!(r, 6);
  ```

- apply

  ```rust
  let a = (1, 2, 3);
  fn foo(a: i32, b: i32, c: i32) -> i32 {
      a + b + c
  }
  let r = foo.apply_tuple(a);
  assert_eq!(r, 6)
  ```

- swap

  ```rust
  let mut a = (1, 2, 3, 4, 5);
  a.swap(1, 3);
  assert_eq!(a, (1, 4, 3, 2, 5));
  ```

- swap_n

  **Not enabled by default**

  ```toml
  features = ["tuple_swap_n"]
  ```

  ```rust
  let mut a = (1, 2, 3, 4, 5);
  a.swap_1_3();
  assert_eq!(a, (1, 4, 3, 2, 5));
  ```

- sort

  Currently implemented sorting algorithm

  - selection sort (default alias)

  ***

  - sort

    ```rust
    let mut a = (6, 2, 6, 8, 0, 5);
    a.sort();
    assert_eq!(a, (0, 2, 5, 6, 6, 8))
    ```

  - sort_desc

    ```rust
    let mut a = (6, 2, 6, 8, 0, 5);
    a.sort_desc();
    assert_eq!(a, (8, 6, 6, 5, 2, 0))
    ```

  - sort_by

    ```rust
    let mut a = (6, 2, 6, 8, 0, 5);
    a.sort_by(|a, b|
        if a > b { core::cmp::Ordering::Less }
        else { core::cmp::Ordering::Greater }
    );
    assert_eq!(a, (0, 2, 5, 6, 6, 8))
    ```

  - sort_by_key

    ```rust
    let mut a = ((6, 2), (6, 8), (0, 5));
    a.sort_by_key(|a| a.1);
    assert_eq!(a, ((6, 2), (0, 5), (6, 8)))
    ```

  - sort_by_key_desc

    ```rust
    let mut a = ((6, 2), (6, 8), (0, 5));
    a.sort_by_key_desc(|a| a.1);
    assert_eq!(a, ((6, 8), (0, 5), (6, 2)))
    ```

  ***

  - sorted / sorted_xx

    Variants of Transferring Ownership

    ```rust
    let a = (6, 2, 6, 8, 0, 5);
    let a = a.sorted();
    assert_eq!(a, (0, 2, 5, 6, 6, 8))
    ```

- permutations

  ```rust
  let a = (1, '2', "3");
  let r = a.permutations_2();
  assert_eq!(r, ((1, '2'), (1, "3"), ('2', 1), ('2', "3"), ("3", 1), ("3", '2')));
  ```

- combinations

  ```rust
  let a = (1, '2', "3", 4.0);
  let r = a.combinations_2();
  assert_eq!(r, ((1, '2'), (1, "3"), (1, 4.0), ('2', "3"), ('2', 4.0), ("3", 4.0)));
  ```
