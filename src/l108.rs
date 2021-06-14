use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree(&nums, 0, nums.len())
    }

    fn build_tree(nums: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if left >= right {
            return None;
        } else {
            let mid = (left + right) / 2;
            return Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: Solution::build_tree(nums, left, mid),
                right: Solution::build_tree(nums, mid + 1, right),
            })));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::l108;
    use crate::l94;

    #[test]
    fn leetcode108() {
        let seq = vec![-10, -3, 0, 5, 9];
        let tree = l108::Solution::sorted_array_to_bst(seq);
        let result = l94::Solution::inorder_traversal(tree);
        assert_eq!(result, vec![-10, -3, 0, 5, 9]);
    }
}
