use std::rc::Rc;
use std::cell::RefCell;
use core::borrow::Borrow;

/// TreeNode
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    // option for null support, Rc for multiple reference,
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

//            let al = &Rc::try_unwrap(a.unwrap()).unwrap().into_inner();
//            let al = (*a.unwrap()).into_inner();
//            let j = &a.unwrap();
            a.as_ref().unwrap() == b.as_ref().unwrap()
//            &Rc::clone(a.as_ref().unwrap()) == &Rc::clone(&b.as_ref().unwrap())
        };

//        let jj = Some;
        comparator(&self.left, &other.left) && comparator(&self.right, &other.right)
    }
}

#[cfg(test)]
mod node_tests {
    use crate::utils::node::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    pub fn test_tree_equality() {
        assert_eq!(&get_tree() == &get_tree(), true)
    }

    fn get_tree() -> TreeNode {
        let mut node1 = TreeNode::new(5);
        let mut node2 = TreeNode::new(6);
        let mut node3 = TreeNode::new(4);
        let mut node4 = TreeNode::new(3);
        let mut node5 = TreeNode::new(2);

        node3.left = Some(Rc::new(RefCell::new(node4)));
        node3.right = Some(Rc::new(RefCell::new(node5)));
        node1.left = Some(Rc::new(RefCell::new(node3)));
        node1.right = Some(Rc::new(RefCell::new(node2)));

        node1
    }
}