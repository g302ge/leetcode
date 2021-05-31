use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_vaild_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::check_vaild_bst(root, std::i32::MIN, std::i32::MAX)
    }

    fn check_vaild_bst(root: Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> bool {
        match root {
            // because the Rc is clone and increase the pointer count and pass it to the function
            Some(node) => {
                let current = node.borrow();
                current.val < max &&
                    current.val > min &&
                    Solution::check_vaild_bst(current.left.clone(), min, current.val) &&
                    Solution::check_vaild_bst(current.right.clone(), current.val, max)
            },
            None => true
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::l98;
    use crate::TreeNode;

    #[test]
    fn leetcode98(){
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        let left = Rc::new(RefCell::new(TreeNode::new(1)));
        let right = Rc::new(RefCell::new(TreeNode::new(3)));
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);
        
        let vaild = l98::Solution::is_vaild_bst(Some(root));
        assert_eq!(vaild, true);
    }
}
