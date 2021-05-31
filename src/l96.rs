
pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
       if n == 0 {
        return 0;
       } 
       Solution::generate_tree_num(1, n)
    }

    fn generate_tree_num(start: i32, end: i32) -> i32{
        if start > end {
            return 1;
        }
        let mut number = 0;
        for i in start..= end {
            let left = Solution::generate_tree_num(start, i-1);
            let right = Solution::generate_tree_num(i + 1, end);
            number += left * right;
        }        

        number
    }
}

#[cfg(test)]
mod tests {
    use crate::l96;

    #[test]
    fn leetcode96(){
       let num = l96::Solution::num_trees(3);
       assert_eq!(num, 5);
    }
}
