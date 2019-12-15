use std::collections::BTreeSet;

pub fn sorted_set<T>(values: &[T]) -> Vec<T>
    where T: Ord + Copy {
    values.iter().cloned().collect::<BTreeSet<T>>().iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sorted_set() {
        let vals = vec![7, 2, 2, 4, 3, 7, 9, 1];
        let vs = sorted_set(&vals);
        assert_eq!(vs, [1, 2, 3, 4, 7, 9]);
        let vals = vec![7usize, 2, 2, 4, 3, 7, 9, 1];
        let vs = sorted_set(&vals);
        assert_eq!(vs, [1, 2, 3, 4, 7, 9]);
    }
}
