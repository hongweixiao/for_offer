use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

 impl TreeNode {
   #[inline]
   pub fn new(val: i32, left: Option<Rc<RefCell<TreeNode>>>,
                     right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
     Some(Rc::new(RefCell::new(TreeNode {
       val,
       left,
       right,
     })))
   }
 }

 //思路：入队顺序(广度优先遍历)
 fn print_top_to_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec!();
    if root.is_none() {
        return result;
    }
    let mut d = VecDeque::new();
    let root = root.unwrap();
    let mut rev = true; //反转标志
    d.push_back(root);
    while !d.is_empty() {
        let mut v: Vec<i32> = vec!();
        let size = d.len();
        for _ in 0..size {
            let node = d.pop_front().unwrap();
            let val = node.borrow().val; //获取值
            if rev {
                v.push(val);
            } else {
                v.insert(0, val);
            }
            if let Some(left) = node.borrow_mut().left.take() {
                d.push_back(left)
            };
            if let Some(right) = node.borrow_mut().right.take() {
                d.push_back(right)
            };
        }
        if v.len() > 0 {
            result.push(v);
        }
        rev = !rev;
    }
    result
 }


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let l1 = TreeNode::new(1, None, None);
        let l3 = TreeNode::new(3, None, None);
        let l2 = TreeNode::new(2, l1, l3);

        let l6 = TreeNode::new(6, None, None);
        let l9 = TreeNode::new(9, None, None);
        let l7 = TreeNode::new(7, l6, l9);


        let l4 = TreeNode::new(4, l2, l7);
        
        let r = print_top_to_bottom(l4);

        println!("-------------");
        println!("{:?}", r);
        println!("-------------");

        
        
    }
   
}