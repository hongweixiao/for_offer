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


fn find_path(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec!();
    if root.is_none() {
       return result;
    }
    let list: Vec<i32> = vec!();
    find(root, target, 0, list, &mut result);
    return result;
}

fn find(node: Option<Rc<RefCell<TreeNode>>>, target: i32, mut sum: i32, 
                                    mut list: Vec<i32>, result:  &mut Vec<Vec<i32>>) {
  if node.is_none() && target == sum && !result.contains(&list) {
      result.push(list);
      return;
  }
  if node.is_none() {
      return;
  }
  let node = node.unwrap();
  let val = node.borrow().val;
  sum += val;

  if sum > target {
     return;
  }

  list.push(val);

  find(node.borrow_mut().left.take(), target, sum, list.clone(), result);
  find(node.borrow_mut().right.take(), target, sum, list.clone(), result);
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
        
        let r = find_path(l4, 9);

        println!("-------------");
        println!("{:?}", r);
        println!("-------------");

        
        
    }
   
}















