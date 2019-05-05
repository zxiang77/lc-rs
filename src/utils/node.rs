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

        let comparator = |a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>| -> bool {
            if *a == *b {
                return true;
            }

            if *a == None || *b == None {
                return false;

            }

            let av = match a {
                Some(aref) => aref.borrow().borrow(),
                _ => panic!("")
            };

//            let al = &Rc::try_unwrap(a.unwrap()).unwrap().into_inner();
//            let al = (*a.unwrap()).into_inner();
            true
        };

//        let jj = Some;
        comparator(&self.left, &other.left) && comparator(&self.right, &other.right)
    }
}
