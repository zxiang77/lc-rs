use std::rc::Rc;
use std::cell::{RefCell, Ref};
use core::borrow::Borrow;

/// TreeNode
#[derive(Debug, Eq)]
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

            a.as_ref().unwrap() == b.as_ref().unwrap()
        };

        comparator(&self.left, &other.left) && comparator(&self.right, &other.right)
    }
}

#[cfg(test)]
mod node_tests {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    pub fn test_complex_tree_equality() {
        test_equality(&get_complex_tree1);
    }

    fn get_complex_tree1() -> TreeNode {
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

    #[test]
    pub fn test_inequality() {
        assert_eq!(get_complex_tree1() == get_complex_tree2(), false);
        assert_eq!(get_complex_tree1() != get_complex_tree2(), true)
    }

    fn get_complex_tree2() -> TreeNode {
        let mut node1 = TreeNode::new(5);
        let mut node2 = TreeNode::new(6);
        let mut node3 = TreeNode::new(4);
        let mut node4 = TreeNode::new(4);
        let mut node5 = TreeNode::new(3);

        node3.left = Some(Rc::new(RefCell::new(node4)));
        node3.right = Some(Rc::new(RefCell::new(node5)));
        node1.left = Some(Rc::new(RefCell::new(node3)));
        node1.right = Some(Rc::new(RefCell::new(node2)));
        node1
    }

    fn get_single_node() -> TreeNode {
        TreeNode::new(5)
    }

    fn test_single_node_tree() {
        test_equality(&|| -> TreeNode {
            TreeNode::new(3)
        });
    }

    #[test]
    fn test_tree_equality() {
        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(1);

        assert_eq!(node1 == node2, true)

    }

    fn test_equality<F>(builder: &F) where F: Fn() -> TreeNode {
        assert_eq!(builder() == builder(), true);
        assert_eq!(builder() != builder(), false);
    }
}
//
//pub struct FaultyTreeNode {
//    pub val: i32,
//    pub left: Option<RefCell<FaultyTreeNode>>,
//    pub right: Option<RefCell<FaultyTreeNode>>,
//}
//
//impl FaultyTreeNode {
//    #[inline]
//    pub fn new(val: i32) -> Self {
//        FaultyTreeNode {
//            val,
//            left: None,
//            right: None,
//        }
//    }
//}

#[cfg(test)]
mod faulty_node_tests {
//    use super::FaultyTreeNode as TreeNode;
//    use std::cell::RefCell;
//
//    #[test]
//    fn test() {
//        assert_eq!(build_tree() != None, true)
//    }
//
//    fn build_tree() -> TreeNode {
//        let mut node1 = TreeNode::new(5);
//        let mut node2 = TreeNode::new(6);
//        let mut node3 = TreeNode::new(4);
//
//        let ref mut j = node1;
//        let ref mut f = node1;
//        j.val = 4;
//        f.val = 3;
//
////        node1.left = Some(node2);
////        node1.right = Some(node3);
////        let mut node4 = RefCell::new(TreeNode::new(1));
////        let r1 = node4.borrow_mut();
////        let r2 = node4.borrow_mut();
////        r1
//    }
}
