#[allow(dead_code)]
mod utils;
#[allow(dead_code)]
mod l94;
#[allow(dead_code)]
mod l95;
#[allow(dead_code)]
mod l96;
#[allow(dead_code)]
mod l98; 
#[allow(dead_code)]
mod l99;
#[allow(dead_code)]
mod l100;
#[allow(dead_code)]
mod l101;
#[allow(dead_code)]
mod l102;
#[allow(dead_code)]
mod l103;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
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
