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

 //思路：入栈顺序(广度优先遍历)
 fn print_top_to_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec!();
    if root.is_none() {
        return result;
    }
    let mut d = VecDeque::new();
    let root = root.unwrap();
    d.push_back(root);
    while !d.is_empty() {
        let node = d.pop_front().unwrap();
        if node.borrow_mut().left.is_some() {
        //  d.push_back(Rc::clone(&node.borrow_mut().left.take().unwrap()));
            d.push_back(node.borrow_mut().left.take().unwrap());
        }
        if node.borrow_mut().right.is_some() {
        //  d.push_back(Rc::clone(&node.borrow_mut().right.take().unwrap()));
            d.push_back(node.borrow_mut().right.take().unwrap());
        }
        // let val = Rc::try_unwrap(node).ok().unwrap().into_inner().val; //获取值
        let val = node.borrow().val; //获取值
        result.push(val);
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

        assert_eq!(vec!(4,2,7,1,3,6,9), r);
        
        
    }
   
}