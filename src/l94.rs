use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, counter: &mut Vec<i32>){
            if let Some(node) = root{
                traverse(node.borrow_mut().left.take(), counter);
                counter.push(node.borrow().val);
                traverse(node.borrow_mut().right.take(), counter);
            }
        }

        let mut counter = vec![];
        traverse(root, &mut counter);
        counter
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::l94::Solution;
    use crate::l94::TreeNode;

    #[test]
    fn leetcode94(){
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        // if ref keyword is not exists the compiler will complain the partially moved of root
        if let Some(ref node) = root {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
            if let Some(ref node) = node.borrow_mut().right{
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            }
        } 

        let result = Solution::inorder_traversal(root);
        assert_eq!(result, vec![1, 3, 2]);
    }
}

