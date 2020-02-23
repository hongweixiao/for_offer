struct Stack {
	bucket_all: Vec<i32>,
	bucket_min: Vec<i32>,
}
	

impl Stack {

	fn new() -> Self {
		Stack {
			bucket_all: vec!(),
			bucket_min: vec!(),
		}
	}
	
	fn push(&mut self, val: i32) {
		self.bucket_all.push(val);
		if !self.bucket_min.is_empty() {
			if self.min().unwrap() > val {
				self.bucket_min.push(val);
			}
		} else {
			self.bucket_min.push(val);
		}
	}

	fn pop(&mut self) {
		match self.bucket_all.pop() {
			Some(n) => {
				if n == self.min().unwrap() {
					self.bucket_min.pop();
				}
			},
			None => return,
		}
	}


	
	fn top(&self) -> Option<i32> {
		self.bucket_all.last().map(|&n| n)
	}

	fn min(&self) -> Option<i32> {
		self.bucket_min.last().map(|&n| n)
	}
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let mut stack = Stack::new();
        stack.push(-1);
        stack.push(0);
        stack.push(2);

		assert_eq!(Some(-1), stack.min());
		assert_eq!(Some(2), stack.top());
		stack.pop();
		assert_eq!(Some(0), stack.top());
		stack.pop();
		assert_eq!(Some(-1), stack.top());
		stack.pop();
		assert_eq!(None, stack.top());
		assert_eq!(None, stack.min());
		stack.push(2);
		stack.push(-1);
		assert_eq!(Some(-1), stack.min());
    }


    
}