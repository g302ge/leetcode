use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub struct Solution;

impl Solution {

  pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    Self::depth(root) != -1
  }

  fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
      let left = Self::depth(node.borrow().left.clone());
      if left == -1 {
        return -1;
      }

      let right = Self::depth(node.borrow().right.clone());
      if right == -1 {
        return -1;
      }

      if (left - right).abs() < 2 {
        return std::cmp::max(left, right) + 1;
      }else {
          return -1;
      }

    }
    0
  }
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use std::cell::RefCell;
  use crate::TreeNode;
  use crate::l110;
  #[test]
  fn leetcode110() {
    let tree = Some(
      Rc::new(
        RefCell::new(
          TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(
              Rc::new(
                RefCell::new(
                  TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(20))))
                  }
                )
              )
            )
          }
        )
      )
    );

    let result = l110::Solution::is_balanced(tree);
    assert_eq!(result, true);
  }
}