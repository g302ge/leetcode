pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut start, mut end) = (0, 0);
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for i in (0..s.len()).rev() {
            for j in i..s.len() {
                if i == j || j - i == 1 && s[i] == s[j] {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];
                }
                if dp[i][j] && j - i > end - start {
                    start = i;
                    end = j;
                }
            }
        }
        s[start..=end].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode5() {
        let result = Solution::longest_palindrome("babcd".to_string());
        assert_eq!("bab", result);
    }
}
