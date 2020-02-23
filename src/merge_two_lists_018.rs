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


	fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
		let (mut list1, mut list2) = (list1, list2);
		let mut list3 = ListNode::new(0, None).unwrap();
		let mut p3 = &mut list3;
		while let(Some(p1), Some(p2)) = (list1.as_ref(), list2.as_ref()) {
			if p2.value > p1.value {
				p3.next = list1;  //将较小链表连接到新链表尾节点，所有权移动
				p3 = p3.next.as_mut().unwrap(); //将list3尾节点指向它的后继节点
				list1 = p3.next.take(); //将链表从尾节点取下来，将所有权返给较小的链表
			} else {
				p3.next = list2;  //将较小链表连接到新链表尾节点，所有权移动
				p3 = p3.next.as_mut().unwrap(); //将list3尾节点指向它的后继节点
				list2 = p3.next.take(); //将链表从尾节点取下来，将所有权返给较小的链表
			}
		}

		p3.next = if list1.is_some() {
					list1
		} else {
					list2
		};

		list3.next
	}



	#[cfg(test)]
	mod tests {
	    use super::*;
	    #[test]
	    fn test_01() {
			
	    	let n7 = ListNode::new(7, None);
			let n5 = ListNode::new(5, n7);	
	    	let n3 = ListNode::new(3, n5);
	    	let list1 = ListNode::new(1, n3);

	    	let n4 = ListNode::new(4, None);
	    	let list2 = ListNode::new(2, n4);

	    	let r = merge_two_lists(list1, list2);	

	    	println!("-------------");
	    	println!("{:?}", r);
	    	println!("-------------");
	    }

	    fn test_02() {
		
	    	
	    }
		
	}