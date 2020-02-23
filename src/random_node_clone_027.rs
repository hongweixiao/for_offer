use std::collections::HashMap;
use std::sync::Mutex;


lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ListNode, ListNode>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };    
}



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ListNode {
	value: i32,
	next: Option<Box<ListNode>>,
	random: Option<Box<ListNode>>,
}

impl ListNode {
	pub fn new(value: i32, next: Option<Box<ListNode>>, random: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		Some(Box::new(ListNode{
			value,
			next,
			random,
		}))
	}
}


fn copy(old_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	if old_head.is_none() {
		return None;
	}
	let mut map = HASHMAP.lock().unwrap();
	let old_head = *old_head.unwrap();
	if map.contains_key(&old_head) {
		let new_node =  map.remove(&old_head).unwrap(); //这里好像不能用get
		return Some(Box::new(new_node));
	}
	let mut new_node = ListNode {value: old_head.value, next: None, random: None};
	map.insert(old_head.clone(), new_node.clone());
	std::mem::drop(map);  //释放锁
	new_node.next = copy(old_head.next);
	new_node.random = copy(old_head.random);
	return Some(Box::new(new_node));
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
		
    	let n5 = ListNode::new(5, None, None);
		let n4 = ListNode::new(4, n5, None);	
    	let n3 = ListNode::new(3, n4, None);
		let n2 = ListNode::new(2, n3, None);

    	let head = ListNode::new(1, n2, None);

    	println!("old_node is {:?}", head);
    	println!("~~~~~~~~~~~~~~~~~");

    	let n = copy(head.clone());
    	println!("new node is {:?}", n);
    	println!("-------------");
    	assert_eq!(head, n);
    }

	
}