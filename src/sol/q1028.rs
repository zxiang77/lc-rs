use std::rc::Rc;
use std::cell::RefCell;
use super::super::utils::node::TreeNode;
use std::collections::HashMap;
use core::borrow::Borrow;

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
        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

        let h = shared_map.borrow_mut();
        match root {
            Some(r) => Some(Rc::new(r)),
            None => None
        }
    }
    /// https://doc.rust-lang.org/1.8.0/book/lifetimes.html the lifetime link to read
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

//    fn t2() -> &'static i32 {
//        let x = 33;
//        &x
//    }

    pub fn test() {
        for (k, v) in Solution::get_data() {
            assert_eq!(Solution::recover_from_preorder(k).unwrap().into_inner().borrow(), &v)
        }
    }

    pub fn get_data() -> HashMap<String, TreeNode>
    {
        let mut data = HashMap::new();
        let input1 = String::from("23");
        let res1 = TreeNode::new(23);
        data.insert(input1, res1);

        let input2 = String::from("23-3");
        let mut res2 = TreeNode::new(23);
        res2.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        data.insert(input2, res2);
        data
    }
}
