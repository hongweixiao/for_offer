use std::rc::Rc;

#[derive(Debug)]
pub struct ListNode {
	value: i32,
	next: Option<Rc<ListNode>>,
}

impl ListNode {
	pub fn new(value: i32, next: Option<Rc<ListNode>>) -> Option<Rc<ListNode>> {
		Some(Rc::new(ListNode{
			value,
			next,
		}))
	}
}

fn find_first_common_node(head1: Option<Rc<ListNode>>, head2: Option<Rc<ListNode>>) -> Option<Rc<ListNode>> {
	if head1.is_none() || head2.is_none() {
		return None;
	}
	let mut stack1: Vec<*const ListNode> = vec!();
	let mut stack2: Vec<*const ListNode> = vec!();
	let mut stack3: Vec<*const ListNode> = vec!();
	let mut p1 = head1.as_ref();
	let mut p2 = head2.as_ref();
	
	while let Some(node) = p1 {
		let addr = node.as_ref() as *const ListNode;
		p1 = p1.unwrap().next.as_ref();
		stack1.push(addr);
	}

	while let Some(node) = p2 {
		let addr = node.as_ref() as *const ListNode;
		p2 = p2.unwrap().next.as_ref();
		stack2.push(addr);
	}

	while !stack1.is_empty() && !stack2.is_empty() {
		let addr1 = stack1.pop().unwrap();
		let addr2 = stack2.pop().unwrap();
		if addr1 == addr2 {
			stack3.push(addr1);
		} else {
			break;
		}
	}

	if stack3.is_empty() {
		return None;
	}

	let mut head = head1;
	let first_common_addr = stack3.pop().unwrap();
	
	while let Some(node) = head.as_ref() {
		let addr = node.as_ref() as *const ListNode;
		if addr == first_common_addr {
			break;
		} else {
		 	//head = Rc::try_unwrap(head.unwrap()).ok().unwrap().next;
			head = Some(Rc::clone(&(node.next.as_ref().unwrap())));
		}
	}

	return head;
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let n5 = ListNode::new(5, None);
		let n4 = ListNode::new(4, n5);	
    	let n3 = ListNode::new(3, n4);

    	//let n2 = Rc::new(ListNode{value: 2, next: n3});

    	let head1 = ListNode::new(11, Some(Rc::clone(&n3.clone().unwrap())));
    	//let head111 = ListNode::new(111, head1);

    	let head22 = ListNode::new(22, n3);
    	let head222 = ListNode::new(222, head22);
    	let r =  find_first_common_node(head1, head222);
    	println!("-------------");
    	println!("{:?}", r);
    	println!("-------------");
    }

    
	
}



