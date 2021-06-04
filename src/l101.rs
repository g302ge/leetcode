use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution;

impl Solution {

  pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool{
    fn dfs(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
      if let (None, None) = (&left, &right){
        return true;
      }
      if let (Some(ref l), Some(ref r)) = (left, right){
        if l.borrow().val == r.borrow().val {
          return dfs(l.borrow().left.clone(), r.borrow().right.clone()) && 
          dfs(l.borrow().right.clone(), r.borrow().left.clone());
        }
        return false;
      }
      false
    }

    if let Some(ref node) = root {
      return dfs(Some(node.clone()), Some(node.clone()))
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use std::cell::RefCell;
  use crate::TreeNode;
  use crate::l101::Solution;

  #[test]
  fn leetcode101() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let first_left = Rc::new(RefCell::new(TreeNode::new(2)));
    let first_right = Rc::new(RefCell::new(TreeNode::new(2)));
    let first_left_left = Rc::new(RefCell::new(TreeNode::new(3)));
    let first_left_right = Rc::new(RefCell::new(TreeNode::new(4)));
    let first_right_left = Rc::new(RefCell::new(TreeNode::new(4)));
    let first_right_right = Rc::new(RefCell::new(TreeNode::new(3)));

    first_left.borrow_mut().left = Some(first_left_left);
    first_left.borrow_mut().right = Some(first_left_right);
    first_right.borrow_mut().left = Some(first_right_left);
    first_right.borrow_mut().right = Some(first_right_right);
    root.borrow_mut().left = Some(first_left);
    root.borrow_mut().right = Some(first_right);

    let result = Solution::is_symmetric(Some(root));

    assert_eq!(result, true);
  }
}