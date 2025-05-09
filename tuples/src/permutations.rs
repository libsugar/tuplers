//! Permutations

include!("./gen/permutations.rs");

#[cfg(test)]
mod tests {
    use super::TuplePermutations2;

    #[test]
    fn test1() {
        let a = (1, 2, 3);
        let r = a.permutations_2();
        assert_eq!(r, ((1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)));
    }

    
    #[test]
    fn test2() {
        let a = (1, '2', "3");
        let r = a.permutations_2();
        assert_eq!(r, ((1, '2'), (1, "3"), ('2', 1), ('2', "3"), ("3", 1), ("3", '2')));
    }
}
