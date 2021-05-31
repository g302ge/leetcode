mod utils;
mod l94;
mod l95;
mod l96;
mod l98; 
use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self{
        TreeNode{
            val,
            left: None,
            right: None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
