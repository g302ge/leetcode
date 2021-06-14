use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let (None, None) = (&p, &q) {
            return true;
        } else {
            if let (Some(ref left), Some(ref right)) = (p, q) {
                if left.borrow().val == right.borrow().val {
                    return Solution::is_same_tree(
                        left.borrow().left.clone(),
                        right.borrow().left.clone(),
                    ) && Solution::is_same_tree(
                        left.borrow().right.clone(),
                        right.borrow().right.clone(),
                    );
                }
                return false;
            }
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::l100::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn leetcode100() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(3)));
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        let other = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(3)));
        other.borrow_mut().left = Some(left);
        other.borrow_mut().right = Some(right);

        //assert_eq!(vec![2, 1, 3], l94::Solution::inorder_traversal(Some(root)));
        //assert_eq!(vec![2, 1, 3], l94::Solution::inorder_traversal(Some(other)));

        let result = Solution::is_same_tree(Some(root), Some(other));
        assert_eq!(result, true);
    }
}
