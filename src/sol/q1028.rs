use std::rc::Rc;
use std::cell::RefCell;
use super::super::utils::node::TreeNode;
use std::mem;
pub struct Solution {

}

impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack : Vec<RefCell<TreeNode>> = vec![];

        // init
        let (root, dep, st) = Solution::nxt(&s);

        while st.len() > 0 {
            let (root, dep2, st) = Solution::nxt(st);
            let v = root.unwrap();

            let _ = stack.push(v);
            while dep2 <= stack.len() {
                let _ = stack.pop();
            }

            let parent= stack.get(1).unwrap();
        }

        match root {
            Some(r) => Some(Rc::new(r)),
            None => None
        }
    }

    fn nxt(s: &str) -> (Option<RefCell<TreeNode>>, usize, &str) {
        let mut ctr : usize = 0;

        while  !"-".eq(&s[ctr..ctr]) {
            ctr += 1
        }

        let node = match s.get(0..ctr).unwrap().to_string().parse::<i32>() {
            Ok(x) => Some(RefCell::new(TreeNode::new(x))),
            Err(_) => None
        };

        (node, ctr, s.get(ctr..).unwrap())
    }

    fn t2() -> &'static i32 {
        let x = 33;
        &x
    }

    pub fn test() {
        let v = vec![];
        v.iter();
        v.into_iter();
    }
}
