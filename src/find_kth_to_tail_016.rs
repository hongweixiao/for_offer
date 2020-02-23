#[derive(Debug)]
pub struct ListNode {
	value: i32,
	next: Option<Box<ListNode>>,
}

impl ListNode {
	pub fn new(value: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		Some(Box::new(ListNode{
			value,
			next,
		}))
	}
}

impl PartialEq for ListNode {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}


fn find_kth_to_tail(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>{
	if k == 0 {
		return None;
	}
	let mut head = head;
	let mut left = head.as_ref();
	let mut right = head.as_ref();
	let mut i = 0;
	while right.is_some() {
		if i >= k {
			if let Some(node) = left {
				left = node.next.as_ref();
			}
		}
		if let Some(node) = right {
			right = node.next.as_ref();
		}
		i += 1;
	}
	if i < k {
		return None;
	}
	let left_addr = if let Some(node) = left {
		node.as_ref() as *const ListNode
	} else {
		return None;
	};

	while let Some(node) = head.as_ref() {
		let addr = node.as_ref() as *const ListNode;
		if addr != left_addr {
			head = head.unwrap().next;
		} else {
			break;
		}
	}
	head
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let n5 = ListNode::new(5, None);
		let n4 = ListNode::new(4, n5);	
    	let n3 = ListNode::new(3, n4);
		let n2 = ListNode::new(2, n3);

    	let head = ListNode::new(1, n2);
    	
    	let n = find_kth_to_tail(head, 1);
    	println!("-------------");
    	println!("{:?}", n);
    	println!("-------------");
    	assert_eq!(5, n.unwrap().value);
    }

    fn test_02() {
	
    	let n5 = ListNode::new(5, None);
		let n4 = ListNode::new(4, n5);	
    	let n3 = ListNode::new(3, n4);
		let n2 = ListNode::new(2, n3);

    	let head = ListNode::new(1, n2);
    	
    	let n = find_kth_to_tail(head, 0);
    	assert_eq!(None, n);
    }
	
}