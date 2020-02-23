use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;


#[derive(Debug)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
	fn new(val: i32) -> Rc<RefCell<TreeNode>> {
		Rc::new(RefCell::new(
			TreeNode {val,
				left: None,
				right: None,
				parent: None,
			}))
	}
}

fn get_next(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
	if node.is_none() {
		return None;
	}
	let node = node.unwrap();
	if node.borrow().right.is_some() { //有右子树，下一结点是右子树中的最左结点
		let mut target = node.borrow_mut().right.take().unwrap();
		while target.borrow().left.is_some() {
			let left = target.borrow_mut().left.take().unwrap();
			target = left;
		}
		return Some(target);
	} else {
		if node.borrow().parent.is_some() {

			 
			//无右子树，且结点是该结点父结点的左子树，则下一结点是该结点的父结点

			let parent = node.borrow_mut().parent.take().unwrap();
			let parent = parent.upgrade().unwrap(); //Weak 转 Rc

			if parent.borrow().left.is_some() {
				let left = parent.borrow_mut().left.take().unwrap();   //当前节点父节点的左孩子
				if Rc::ptr_eq(&node, &(left)) {
					parent.borrow_mut().left = Some(Rc::clone(&left));  //左孩子加回去
					node.borrow_mut().parent = Some(Rc::downgrade(&parent));  //Rc转Weak加回去
					return Some(parent);
				}
			} 

			if parent.borrow().right.is_some() {
				let right = parent.borrow_mut().right.take().unwrap();   //当前节点父节点的右孩子
				if Rc::ptr_eq(&node, &(right)) {
					//无右子树，且结点是该结点父结点的右子树，则我们一直沿着父结点追朔，直到找到某个结点是其父结点的左子树
					//如果存在这样的结点,那么这个结点的父结点就是我们要找的下一结点
					let mut target = Rc::clone(&parent);  //父节点
					let mut curr = Rc::clone(&node);  //当前节点
					loop {
						println!("1111");

						if target.borrow().left.is_some() {
							let left = target.borrow_mut().left.take().unwrap();   //当前节点父节点的左孩子
							if Rc::ptr_eq(&curr, &(left)) {
								break;
							}
							target.borrow_mut().left = Some(left);  //加回去
						}
						curr = Rc::clone(&target);

						if target.borrow().parent.is_none() {
							break;
						} 

						let t = target.borrow_mut().parent.take().unwrap(); //父节点的父节点
						target = t.upgrade().unwrap(); //Weak 转 Rc
						curr.borrow_mut().parent = Some(Rc::downgrade(&target)); //父节点加回去
					}
					return Some(target);

				}
			}

		}

	}
	None
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let n1 = TreeNode::new(1);
        let n2 = TreeNode::new(2);
        let n3 = TreeNode::new(3);
        let n4 = TreeNode::new(4);
        let n5 = TreeNode::new(5);
        let n6 = TreeNode::new(6);
        let n7 = TreeNode::new(7);

        n2.borrow_mut().left =   Some(Rc::clone(&n1));
        n2.borrow_mut().right =  Some(Rc::clone(&n3));
        n1.borrow_mut().parent = Some(Rc::downgrade(&n2));
        n3.borrow_mut().parent = Some(Rc::downgrade(&n2));


        n6.borrow_mut().left =   Some(Rc::clone(&n5));
        n6.borrow_mut().right =  Some(Rc::clone(&n7));
        n5.borrow_mut().parent = Some(Rc::downgrade(&n6));
        n7.borrow_mut().parent = Some(Rc::downgrade(&n6));


        n4.borrow_mut().left =   Some(Rc::clone(&n2));
        n4.borrow_mut().right =  Some(Rc::clone(&n6));
        n2.borrow_mut().parent = Some(Rc::downgrade(&n4));
        n6.borrow_mut().parent = Some(Rc::downgrade(&n4));

        let r = get_next(Some(Rc::clone(&n5)));
        println!("{:?}", r);

    }
    
}
