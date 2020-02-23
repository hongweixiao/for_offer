

static mut STACK1: Vec<i32> = Vec::new();

static mut STACK2: Vec<i32> = Vec::new();



pub unsafe fn push(n: i32) {
	STACK1.push(n); 
}

pub unsafe fn pop() -> Option<i32>{
	if !STACK2.is_empty() {
		return STACK2.pop();
	}
	while !STACK1.is_empty() {
		STACK2.push(STACK1.pop().unwrap());
	}
	return STACK2.pop();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_01() {
	    unsafe{
	   		assert_eq!(None, pop());
	    	push(1);
	    	push(2);
	    	assert_eq!(Some(1), pop());
	    	assert_eq!(Some(2), pop());
	    	assert_eq!(None, pop());
	    }
		
    }
	
	#[test]
    fn test_find_02() {
		
    }
}