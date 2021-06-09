use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::traversal(root, &mut result);
        result
    }

    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>){
        if let Some(ref node) = root {
            result.push(node.borrow().val);
            Self::traversal(node.borrow().left.clone(), result);
            Self::traversal(node.borrow().right.clone(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::TreeNode;
    use crate::l144::Solution;

    #[test]
    fn leetcode144(){
        let tree = Some(Rc::new(
                    RefCell::new(
                            TreeNode {
                                val: 1,
                                left: None,
                                right: Some(Rc::new(
                                            RefCell::new(
                                                    TreeNode {
                                                        val: 2,
                                                        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                                                        right: None
                                                    }
                                                )
                                        ))
                            }
                        )
                ));
        let result = Solution::preorder_traversal(tree);
        assert_eq!(result, vec![1, 2, 3]);
    }
}
