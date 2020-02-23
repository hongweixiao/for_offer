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


fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
	let mut prev = None;
	let mut next = head;
	while let Some(mut node) = next {
		next = node.next.take();  //next往后走
		node.next = prev;  //翻转
		prev = Some(node);  //prev往后走
	}
	prev
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
    	let n = reverse_list(head);
    	println!("-------------");
    	println!("{:?}", n);
    	println!("-------------");
    	assert_eq!(5, n.unwrap().value);
    }

    fn test_02() {
	
    	
    }
	
}