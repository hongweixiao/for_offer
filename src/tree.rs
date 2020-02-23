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