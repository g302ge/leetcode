use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let length = inorder.len();
        let val = preorder[0];
        let index = inorder.iter().position(|&x| x == val).unwrap();
        let mut root = TreeNode::new(val);
        if index > 0 {
            root.left =
                Self::build_tree(preorder[1..index + 1].to_vec(), inorder[0..index].to_vec());
        }
        if index < length - 1 {
            root.right = Self::build_tree(
                preorder[index + 1..].to_vec(),
                inorder[index + 1..].to_vec(),
            );
        }
        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use crate::l105;
    use crate::l94;

    #[test]
    fn leetcode105() {
        let tree = l105::Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        let result = l94::Solution::inorder_traversal(tree);
        assert_eq!(result, vec![9, 3, 15, 20, 7]);
    }
}
