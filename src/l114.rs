use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
       if let Some(ref mut node) = root {
         Self::flatten(&mut node.borrow_mut().left);
         Self::flatten(&mut node.borrow_mut().right);
         let right = node.borrow_mut().right.take();
         let left = node.borrow_mut().left.take();
         node.borrow_mut().right = left;
         let mut right_clone = node.clone();            
         while right_clone.borrow().right.is_some() {
             let current = right_clone.borrow().right.clone().unwrap();
             right_clone = current;
         }
         right_clone.borrow_mut().right = right;
       }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::TreeNode;
    use crate::l114;
    use crate::l94;

    #[test]
    fn leetcode114() {
       let mut tree = Some(
                Rc::new(
                    RefCell::new(
                        TreeNode{
                            val: 1,
                            left: Some(Rc::new(RefCell::new(
                                        TreeNode {
                                            val: 2,
                                            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                                            right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                                        }
                                        ))),
                            right: Some(Rc::new(RefCell::new(
                                    TreeNode{
                                        val: 5,
                                        left: None,
                                        right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
                                    }
                                    )))
                        }
                        )
                    )
           ); 
    
      l114::Solution::flatten(&mut tree);
      let result = l94::Solution::inorder_traversal(tree);
      assert_eq!(result, vec![1, 2, 3, 4, 5, 6])
    }
}
