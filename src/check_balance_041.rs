use std::cell::RefCell;
use std::rc::Rc;


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

//是否平衡二叉树
 fn is_balanced(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let node = root.unwrap();
    let left = tree_dept(node.borrow().left.as_ref());  //左子树深度
    let right = tree_dept(node.borrow().right.as_ref()); //右子树深度
    if (left-right).abs() > 1 { //相差超过1 则不平衡
        return false;
    }
    return is_balanced(node.borrow().left.as_ref()) && is_balanced(node.borrow().right.as_ref());

 }
//获取树的深度
fn tree_dept(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let node = root.unwrap();
    let left = tree_dept(node.borrow().left.as_ref());
    let right = tree_dept(node.borrow().right.as_ref());
    return 1 + left.max(right);
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

        let r = is_balanced(l4.as_ref());

        println!("-------------");
        println!("{:?}", r);
        println!("-------------");
        println!("&&&&&&&&&&&&&&&&&&&&&&&");
        println!("{:?}", l4);
        println!("&&&&&&&&&&&&&&&&&&&&&&&");
        
        
    }
   
}


