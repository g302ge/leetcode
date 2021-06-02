use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub struct Solution;

/// # Solution 
/// 
/// using morris traverse 
/// 1. if current node left child is None then redirct current = current.right 
/// 2. if current node left child is Some then find the most_right in left subtree 
///     1. if most_right right child is None then change it to current then current = current.left
///     2. if most_right right child is current change it to None then current = current.right 
impl Solution {
    
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        while Some(ref current) = root {
            // current is Rc reference need to clone to the next iteration 

        }     
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn leetcode99(){
        
    }
}
