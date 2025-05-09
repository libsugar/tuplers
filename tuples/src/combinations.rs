//! Combinations

include!("./gen/combinations.rs");

#[cfg(test)]
mod tests {
    use super::TupleCombinations2;

    #[test]
    fn test1() {
        let a = (1, '2', "3", 4.0);
        let r = a.combinations_2();
        assert_eq!(r, ((1, '2'), (1, "3"), (1, 4.0), ('2', "3"), ('2', 4.0), ("3", 4.0)));
    }
}
