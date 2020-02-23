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

   pub fn new_v2(val: i32) -> Rc<RefCell<TreeNode>> {
     Rc::new(RefCell::new(TreeNode {
       val,
       left: None,
       right: None,
     }))
   }


 }

  //给定一棵二叉搜索树，请找出其中的第k小的结点
  //二叉搜索树的中序遍历是 排序数组
 fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: usize) -> Option<Rc<RefCell<TreeNode>>>{
    if root.is_none() || k == 0 {
      return None;
    }
    let mut v: Vec<Option<Rc<RefCell<TreeNode>>>> = vec!();
    iter_tree(root, &mut v, k);
    if k <= v.len() {
        return v[k-1].take();
    }

    None
    
 }

//中序遍历
 fn iter_tree(node: Option<Rc<RefCell<TreeNode>>>,v: &mut Vec<Option<Rc<RefCell<TreeNode>>>>, k: usize ) {
    if let Some(curr) = node {
        let mut curr_borrow = curr.borrow_mut();
        if let Some(left) = curr_borrow.left.take() {
            curr_borrow.left = Some(Rc::clone(&left));
            iter_tree(Some(left), v, k);
        };
        v.push(Some(Rc::clone(&curr)));
        if v.len() >= k {
            return;
        }

        if let Some(right) = curr_borrow.right.take() {
            curr_borrow.right = Some(Rc::clone(&right));
            iter_tree(Some(right), v, k);
        };
    }
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
        
        let r = kth_smallest(l4, 6);

        println!("-------------");
        println!("{:?}", r);
        println!("-------------");


        
    }
   
}