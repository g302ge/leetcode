// leetcode test helper function

// deeply check vector equality
pub fn vec_deep_eq<T: std::cmp::PartialEq>(left: &Vec<T>, right: &Vec<T>) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right.iter())
        .filter(|&(a, b)| a == b)
        .count()
        == left.len()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn test_vec_deep_eq() {
        let left = vec![1, 2, 3];
        let right = vec![1, 2, 3];

        assert_eq!(utils::vec_deep_eq(&left, &right), true);
    }
}
