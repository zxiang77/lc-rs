use std::rc::Rc;
use std::cell::RefCell;
use super::super::utils::node::TreeNode;
use std::collections::HashMap;
use core::borrow::Borrow;

pub struct Solution {

}

impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let (node, dep, remain) = Solution::parse_next(&s);

        let mut stack : Vec<Rc<RefCell<TreeNode>>> = vec![];
        stack.push(Rc::clone(node.as_ref().unwrap()));

        while remain.len() > 0 { // the stack size == current depth
            let (node2, dep2, remain) = Solution::parse_next(remain);
            match node2 {
                None => break,
                Some(n) => {
                    while stack.len() > dep2 {
                        stack.pop();
                    }

                    let mut cur = Solution::peek(&stack);
                    stack.push(Rc::clone(&n));

                    if cur.borrow_mut().left == None {
                        cur.borrow_mut().left = Some(n);
                    } else {
                        cur.borrow_mut().right = Some(n);
                    }
                }
            }
        }

        node

    }

    fn peek(v: &Vec<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        let l = v.len();
        Rc::clone(&v[l - 1])
    }

    /// https://doc.rust-lang.org/1.8.0/book/lifetimes.html the lifetime link to read
    fn parse_next(s: &str) -> (Option<Rc<RefCell<TreeNode>>>, usize, &str) {
        let mut start = 0;
        while s.chars().nth(start) != Some('-') {
            start += 1;
        }

        if start == 0 {
            return (None, 0, s);
        }

        let num = s[0..start].parse::<i32>().unwrap();
        let node = Some(Rc::new(RefCell::new(TreeNode::new(num))));
        let mut ctr = start;
        while s.chars().nth(ctr) == Some('-') {
            ctr += 1;
        }
        let j = &s[ctr..];

        (node, ctr - start, j)

    }


}

#[cfg(test)]
mod q1028_test {
    use super::*;
    #[test]
    pub fn test() {
        for (k, v) in self::get_data() {
            let clone = String::clone(&k);
            assert_eq!(Solution::recover_from_preorder(k).unwrap(), Rc::new(RefCell::new(v)));
            println!("done {}", clone)
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