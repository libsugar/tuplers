//! Swaps two elements in the tuple
//! 
//! Require feature `tuple_swap_n`

#![allow(non_camel_case_types)]

include!("./gen/tuple_swap_n.rs");

#[test]
fn test() {
    let mut a = (1, 2, 3, 4, 5);
    a.swap_1_3();
    assert_eq!(a, (1, 4, 3, 2, 5));
}
