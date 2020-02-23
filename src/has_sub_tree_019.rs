
#[derive(Debug, Default)]
pub struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    
    pub fn new(value: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>> ) ->  Option<Box<TreeNode>> {
        Some(Box::new(TreeNode {
            value,
            left,
            right,
        }))
    }

}

fn has_sub_tree(root1: Option<Box<TreeNode>>, root2: Option<Box<TreeNode>> ) -> bool {
	if root1.is_none() || root2.is_none() {
		return false;
	}
	if let (Some(n1), Some(n2)) = (root1.as_ref(), root2.as_ref()) {
		return n1.value == n2.value && contains(Some(n1), Some(n2));
	} else {
		return false;
	}

	return has_sub_tree(root1.unwrap().left, root2) || has_sub_tree(root1.unwrap().right, root2);
}

fn contains(node1: Option<&Box<TreeNode>>, node2: Option<&Box<TreeNode>>) -> bool {
	if node1.is_none() && !node2.is_none() {
		return false;
	}
	if node2.is_none() {
		return true;
	}
	if let (Some(n1), Some(n2)) = (node1.as_ref(), node2.as_ref()) {
		return n1.value == n2.value 
				&& contains(n1.left.as_ref(), n2.left.as_ref()) 
				&& contains(n1.right.as_ref(), n2.right.as_ref());
	} else {
		return false;
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


		let l11 = TreeNode::new(1, None, None);
		let l33 = TreeNode::new(3, None, None);
		let l22 = TreeNode::new(2, l11, l33);

		assert!(has_sub_tree(l2, l22));
    	
    }

	#[test]
    fn test_02() {
		let l1 = TreeNode::new(1, None, None);
		let l3 = TreeNode::new(3, None, None);
		let l2 = TreeNode::new(2, l1, l3);


		let l11 = TreeNode::new(1, None, None);
		let l33 = TreeNode::new(4, None, None);
		let l22 = TreeNode::new(2, l11, l33);

		assert!(!has_sub_tree(l2, l22));
    	
    }
	
}






