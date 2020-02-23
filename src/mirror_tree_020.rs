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
       left: left,
       right: right,
     })))
   }
 }

//递归
fn mirror(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    if root.is_none() {
        return None;
    }
    let root = root.unwrap();
    let l = mirror(root.borrow_mut().left.take());
    let r = mirror(root.borrow_mut().right.take());
    root.borrow_mut().left = r;
    root.borrow_mut().right = l;
    Some(root)

}

//非递归
fn mirror_ii(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    if root.is_none() {
        return None;
    }
    let mut d = VecDeque::new();
    let root = root.unwrap();
    d.push_back(Rc::clone(&root)); 
    while d.len() != 0 {
        for _ in 0..d.len() {
            let curr = d.pop_front().unwrap();
            let l = curr.borrow_mut().left.take();
            let r = curr.borrow_mut().right.take();
            curr.borrow_mut().left = r;
            curr.borrow_mut().right = l;
            if curr.borrow_mut().left.is_some() {
                d.push_back(Rc::clone(&curr.borrow_mut().left.as_ref().unwrap())); 
            }
            if curr.borrow_mut().right.is_some() {
                d.push_back(Rc::clone(&curr.borrow_mut().right.as_ref().unwrap())); 
            }
        }
    }
    Some(root)
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
        


        
        let r = mirror(l4);

        println!("-------------");
        println!("{:?}", r);
        println!("-------------");
    }

    #[test]
    fn test_02() {
        let l1 = TreeNode::new(1, None, None);
        let l3 = TreeNode::new(3, None, None);
        let l2 = TreeNode::new(2, l1, l3);

        let l6 = TreeNode::new(6, None, None);
        let l9 = TreeNode::new(9, None, None);
        let l7 = TreeNode::new(7, l6, l9);


        let l4 = TreeNode::new(4, l2, l7);
        
       
        let r = mirror_ii(l4);

        println!("****************");
        println!("{:?}", r);
        println!("****************");
    }

    
}