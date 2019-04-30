use std::rc::Rc;
use std::cell::RefCell;
use super::super::utils::node::TreeNode;
pub struct Solution {

}

impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack : Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![]));
        unimplemented!()
    }

    pub fn test() {
        println!("Hello Rust repo");
    }
}
