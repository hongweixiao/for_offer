type TreeNode = Option<Box<Node>>;

#[derive(Debug, Default)]
pub struct Node {
    value: i32,
    left: TreeNode,
    right: TreeNode,
}

impl Node {
    
    pub fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    //先序遍历

    pub fn pre_order(&self) {
        println!("{}", self.value);

        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    //中序遍历
    pub fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }

    //后续遍历
    fn pos_order(&self) {
        if let Some(ref left) = self.left {
            left.pos_order();
        }
        if let Some(ref right) = self.right {
            right.pos_order();
        }
        println!("{}", self.value);
    }

    
}

pub fn reconstruct_binary_tree(pre: &[i32], ina: &[i32]) -> TreeNode{
    if pre.len() == 0 || ina.len() == 0 {
        return None;
    }
    let mut root = Node::new(pre[0]);
    for i in 0..ina.len() {
        if ina[i] == pre[0] {
            root.left = reconstruct_binary_tree(&pre[1..i+1], &ina[0..i]);
            root.right = reconstruct_binary_tree(&pre[i+1..pre.len()], &ina[i+1..ina.len()]);
        }
    }
    return Some(Box::new(root));
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
		 let pre = [1,2,4,7,3,5,6,8];
         let ina = [4,7,2,1,5,3,8,6];
         let root = reconstruct_binary_tree(&pre, &ina).unwrap();
         root.pos_order();
    }
	
	#[test]
    fn test_02() {
		
    }
}