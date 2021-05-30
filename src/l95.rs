use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution;

impl Solution{
    pub fn gen_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        Solution::generate(1, n)
    }
    
    fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }
        let mut trees = Vec::new();
        for i in start..=end {
            let left = Solution::generate(start, i - 1);
            let right = Solution::generate(i + 1, end);
            for l in &left{
                for r in &right{
                    let current = Rc::new(RefCell::new(TreeNode::new(i)));
                    current.borrow_mut().left = l.clone();
                    current.borrow_mut().right = r.clone();
                    trees.push(Some(current));
                }
            }
        }
        trees
    }
}

#[cfg(test)]
mod tests{
    use crate::l94;
    use crate::l95;
    use crate::utils;


    #[test]
    fn leetcode95(){
        let result = vec![vec![1,2,3], vec![1, 3, 2], vec![2, 1, 3], vec![3, 1, 2], vec![3, 2, 1]];

        let generates = l95::Solution::gen_trees(3);

        for tree in generates.iter(){

            let inorder = if let Some(ref root) = tree {
                l94::Solution::inorder_traversal(Some(root.clone()))
            } else {
                assert!(false);
                vec![]
            };

            if !result.iter().any(|res| utils::vec_deep_eq(&res, &inorder)) {
                assert!(false);
           }
        }

        assert!(true);
    }
}
