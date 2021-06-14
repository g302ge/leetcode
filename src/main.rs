#[allow(dead_code)]
mod l1;
#[allow(dead_code)]
mod utils;

#[allow(dead_code)]
mod l100;
#[allow(dead_code)]
mod l101;
#[allow(dead_code)]
mod l102;
#[allow(dead_code)]
mod l103;
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
mod l105;
#[allow(dead_code)]
mod l106;

#[allow(dead_code)]
mod l108;

#[allow(dead_code)]
mod l110;

#[allow(dead_code)]
mod l114;

#[allow(dead_code)]
mod l144;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
