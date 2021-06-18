pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut window = Vec::new();
        let mut max = 0;
        for c in s.chars() {
            if let Some(i) = window.iter().position(|p| *p == c) {
                window.push(c);
                let (_, r) = window.split_at(i);
                window = r[1..].to_vec();
            } else {
                window.push(c);
            }
            max = std::cmp::max(max, window.len());
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::l3;

    #[test]
    fn leetcode3() {
        let result = l3::Solution::length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(result, 3);
    }
}
