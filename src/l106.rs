use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub struct Solution;

impl Solution {

  pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.len() == 0 {
      return None;
    }

    let plen = postorder.len();
    let pend = plen - 1;
    let val = postorder[pend];
    let index = inorder.iter().position(|&x| x == val).unwrap();
    Some(
      Rc::new(
        RefCell::new(
          TreeNode{
            val,
            left: if index > 0 {
              Self::build_tree(inorder[0..index].to_vec(), postorder[0..index].to_vec())
            } else {
              None
            },
            right: if index < pend {
              Self::build_tree(inorder[index+1..].to_vec(), postorder[index..pend].to_vec())
            }else {
              None
            }
          }
        )
      )
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::l94;
  use crate::l106;

  #[test]
  fn leetcode106() {
    let tree = l106::Solution::build_tree(vec![9,3,15,20,7], vec![9,15,7,20,3]);
    let result = l94::Solution::inorder_traversal(tree);
    assert_eq!(result, vec![9,3,15,20,7]);
  }
}