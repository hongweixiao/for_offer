use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
	fn new(val: i32) -> Rc<RefCell<TreeNode>> {
		Rc::new(RefCell::new(
			TreeNode {val,
				left: None,
				right: None,
			}))
	}
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
	match root {
		Some(node) => {
			let mut n_borrow = node.borrow_mut();
			check(n_borrow.left.take(), n_borrow.right.take())
		},
		None => true,
	}	    
}

fn check(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
	match (left, right) {
		(None, None) => true,
		(Some(l), Some(r)) => {
			let mut l_borrow = l.borrow_mut();
			let mut r_borrow = r.borrow_mut();
			if(l_borrow.val != r_borrow.val) {
				return false;
			}
			check(l_borrow.left.take(), r_borrow.right.take()) 
				&& check(l_borrow.right.take(), r_borrow.left.take())
		},
		_ => false,
	}
}