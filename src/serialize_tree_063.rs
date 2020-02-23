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



 //序列化
 //思路：入队顺序(广度优先遍历) 返回"4_2,7_1,3,6,9_#,#,#,#,#,#,#,#" 结构类型的数据
 fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut result = String::new();
    if root.is_none() {
        return result;
    }
    let mut d = VecDeque::new();
    let root = root.unwrap();
    result.push_str(&root.borrow().val.to_string());  //根节点的值
    d.push_back(root);
    result.push_str("_");
    while !d.is_empty() {
        let size = d.len();
        for i in 0..size {
            let node = d.pop_front().unwrap();
            if let Some(left) = node.borrow_mut().left.take() {
                result.push_str(&left.borrow().val.to_string());  //左子节点值
                d.push_back(left);  //左子节点入队列
            } else {
                 result.push_str("#");
            };
            result.push_str(",");

            if let Some(right) = node.borrow_mut().right.take() {
                result.push_str(&right.borrow().val.to_string()); //右子节点值
                d.push_back(right); //右子节点入队列
            } else {
                 result.push_str("#");
            };
            if i != size-1 {
                result.push_str(",");
            }
        }
        result.push_str("_");
    }
    result.pop();
    result
 }
 
//反序列化
 fn deserialize(data: String) -> Option<Rc<RefCell<TreeNode>>> {
    if data.trim().is_empty() {
        return None;
    }
    let mut d: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let level_arr: Vec<&str> = data.split("_").collect();
    //构造根节点
    let root_val = level_arr[0].split(",").nth(0).unwrap();
    let root_val = root_val.parse::<i32>().unwrap();
    let root = TreeNode::new_v2(root_val);
    //根节点入队列
    d.push_back(Rc::clone(&root));

    for i in 1..level_arr.len()-1 {  //最后一组都是空节点，直接过滤
        let vals: Vec<&str>  = level_arr[i].split(",").collect();
        let mut j: usize = 0;
        while j<vals.len() {
            let node = d.pop_front().unwrap();

            //构造左右孩子
            let left = TreeNode::new_v2(vals[j].parse::<i32>().unwrap());
            let right = TreeNode::new_v2(vals[j+1].parse::<i32>().unwrap());

            //父节点绑定左右孩子
            node.borrow_mut().left = Some(Rc::clone(&left));
            node.borrow_mut().right = Some(Rc::clone(&right));

            //左右孩子节点入队列
            d.push_back(left);
            d.push_back(right);

            //每次用掉2个节点
            j += 2;
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
        
        let r = serialize(l4);

        println!("-------------");
        println!("{:?}", r);
        println!("-------------");

        let n = deserialize(r);

         println!("{:?}", n);
        
    }
   
}