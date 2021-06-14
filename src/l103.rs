use crate::TreeNode;
use std::cell::RefCell;
use std::{collections::VecDeque, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node.clone());
            let mut result: Vec<Vec<i32>> = vec![];
            let mut reverse = false;
            while queue.len() > 0 {
                let current = queue.clone();
                let mut level: Vec<i32> = vec![];
                for n in current {
                    level.push(n.borrow().val);
                    if let Some(ref left) = n.borrow().left {
                        queue.push_back(left.clone());
                    }
                    if let Some(ref right) = n.borrow().right {
                        queue.push_back(right.clone());
                    }
                    queue.pop_front();
                }
                if reverse {
                    level = level.into_iter().rev().collect();
                    reverse = false;
                } else {
                    reverse = true;
                }
                result.push(level);
            }
            return result;
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::l103::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn leetcode103() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        let result = Solution::level_order(root);
        assert_eq!(result, vec![vec![3], vec![20, 9], vec![15, 7]]);
    }
}
