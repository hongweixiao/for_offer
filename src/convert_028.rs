use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use std::mem;



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

fn convert (root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
      return None;
    }
    //中序遍历
    let mut v: Vec<Option<Rc<RefCell<TreeNode>>>> = vec!();
    inOder(root, &mut v);

    //重新构建
    for i in 0..(v.len()-1) {
        //let node_pre = mem::replace(&mut v[i], None).unwrap();
       // let node_next = mem::replace(&mut v[i+1], None).unwrap(); 
       // node_pre.borrow_mut().right = Some(Rc::clone(&node_next));
      //  node_next.borrow_mut().left = Some(Rc::clone(&node_pre));
      //  mem::replace(&mut v[i], Some(node_pre));
      //  mem::replace(&mut v[i+1], Some(node_next));
        v[i].as_ref().unwrap().borrow_mut().right = Some(Rc::clone(&(v[i+1].as_ref().unwrap())));
        v[i+1].as_ref().unwrap().borrow_mut().left = Some(Rc::clone(&(v[i].as_ref().unwrap())));
    }

    return Some(Rc::clone(&(v[0].as_ref().unwrap())));

}

fn inOder(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) {
    if node.is_none() {
      return;
    }
    let node = node.unwrap();
    inOder(Rc::clone(&node).borrow_mut().left.take(), v);
    v.push(Some(Rc::clone(&node)));
    inOder(node.borrow_mut().right.take(), v);
}

fn print(node: Option<Rc<RefCell<TreeNode>>>) {
    if node.is_none() {
        return;
    }
    let node = node.unwrap();
    println!("{}", node.borrow().val);
    print(node.borrow_mut().right.take());
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

        println!("{:?}", l4);
        println!("*************");
        
        let r = convert(l4);

       
       print(r);

       
        
        
    }
   
}





