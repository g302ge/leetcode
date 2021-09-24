pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let total = m + n;
        if total % 2 == 1 {
            let mid = total / 2;
            return Solution::get_kth_num(&nums1, &nums2, mid + 1) as f64;
        } else {
            let mid1 = total / 2;
            let mid2 = total / 2 - 1;
            let median = Solution::get_kth_num(&nums1, &nums2, mid1 + 1)
                + Solution::get_kth_num(&nums1, &nums2, mid2 + 1);
            return (median as f64) / 2;
        }
    }

    fn get_kth_num(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k: usize) -> i32 {
        let m: usize = nums1.len() as usize;
        let n: usize = nums2.len() as usize;
        let mut i: usize = 0;
        let mut j: usize = 0;
        loop {
            if i == m {
                return nums2[j + k - 1];
            }
            if j == n {
                return nums1[i + k - 1];
            }
            if k == 1 {
                return std::cmp::min(nums1[i], nums2[j]);
            }

            let half = k / 2;
            let ni = std::cmp::min(i + half, m) - 1;
            let nj = std::cmp::min(j + half, n) - 1;
            let p1 = nums1[ni];
            let p2 = nums2[nj];
            if p1 <= p2 {
                k -= ni - i + 1;
                i = ni + 1;
            } else {
                k -= nj - j + 1;
                j = nj + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode4() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2 as f64);
    }
}
