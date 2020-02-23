#[derive(PartialEq, Eq, Clone, Debug)]
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


fn delete_duplication(p_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if p_head.is_none() || p_head.as_ref().unwrap().next.is_none() {
        return p_head;
    }
    let mut head = ListNode::new(0, p_head);
    let mut ptr = head.as_mut().unwrap();
    
    while ptr.next.is_some() {
        let mut curr_node = ptr.next.take();  //当前处理的节点
        let curr_value = curr_node.as_ref().unwrap().value;   //当前处理的节点的值
        let mut next = curr_node.as_mut().unwrap().next.take(); //当前处理的节点的下一个节点
        let mut flag = false; //是否重复的标志
        while next.is_some() {
            if next.as_ref().unwrap().value == curr_value {  //有重复节点
                next = next.as_mut().unwrap().next.take();  //往前走
                flag = true;
            } else {
                break;
            }
        }
        if flag {
            ptr.next = next;   //跳过重复节点
             flag = false; //重置
        } else {
            ptr.next = curr_node; //把curr_node加回来
            ptr = ptr.next.as_mut().unwrap(); //指针往前走
            ptr.next = next; //把curr_node的next也加回来
        }
       
    }
    
    head.unwrap().next.take()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        
        let n5 = ListNode::new(5, None);
        let n44 = ListNode::new(4, n5); 
        let n4 = ListNode::new(4, n44);    
        let n33 = ListNode::new(3, n4);
        let n3 = ListNode::new(3, n33);
        let n2 = ListNode::new(2, n3);

        let head = ListNode::new(1, n2);
        let n = delete_duplication(head);
        println!("{:?}", n);
    }
    
}

