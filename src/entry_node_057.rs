use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ListNode {
    val: i32,
    next: *mut ListNode,
}

impl ListNode {
	fn new(val: i32) -> ListNode {
		ListNode {
			val,
			next: ptr::null_mut(),
		}
	}
}


fn entry_node_of_loop(head: &mut ListNode) -> Option<&ListNode>{
	let mut slow = head as *mut ListNode;
	let mut fast = head as *mut ListNode;

	//快慢指针，快的每次走2步，慢的每次走1步，如果有环，那么快指针一定会在环内的某个点上，再次追上慢指针
	unsafe {
		while !slow.is_null() && !fast.is_null() {
			slow = (*slow).next;
			fast = (*fast).next;
			fast = (*fast).next;
			if fast == slow {
				break;
			}
		}

		//没有环
		if fast.is_null() || (*fast).next.is_null() {
			return None;
		}
		//快指针追上漫指针的点 到入口节点 一定等于 头结点到入口节点的距离 2*（A+B）= A+B+B+C
		//fast 重新指向头节点
		fast = head as *mut ListNode;
		while fast != slow {       
			slow = (*slow).next;
			fast = (*fast).next;
		}

		Some(&*fast)
	}

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	let mut n7 = ListNode::new(7);
    	let mut n6 = ListNode::new(6);
    	n6.next = &mut n7 as *mut ListNode;

    	let mut n5 = ListNode::new(5);
    	n5.next = &mut n6 as *mut ListNode;

    	let mut n4 = ListNode::new(4);
    	n4.next = &mut n5 as *mut ListNode;

    	let mut n3 = ListNode::new(3);
    	n3.next = &mut n4 as *mut ListNode;
    	n7.next = &mut n4 as *mut ListNode;

    	let mut n2 = ListNode::new(2);
    	n2.next = &mut n3 as *mut ListNode;

    	let mut n1 = ListNode::new(1);
    	n1.next = &mut n2 as *mut ListNode;

    	let r = entry_node_of_loop(&mut n1);
    	println!("{:?}", r);

    }	
}