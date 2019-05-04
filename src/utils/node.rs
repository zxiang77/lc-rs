use std::rc::Rc;
use std::cell::RefCell;
use core::borrow::Borrow;

/// TreeNode
#[derive(Debug)]
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

impl PartialEq for TreeNode {
    fn eq(&self, other: &TreeNode) -> bool {
        if self.val != other.val {
            return false
        }

        let sl = self.left;
        let sr = self.right;

        let comparator = |a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>| -> bool {
            if a == b {
                return true;
            }

            if a == None || b == None {
                return false;
            }

            let al = a.unwrap().borrow().borrow();

            a.unwrap().borrow().borrow() == b.unwrap().borrow().borrow()
        };

        comparator(self.left, other.left) && comparator(self.right, other.right)
    }
}
