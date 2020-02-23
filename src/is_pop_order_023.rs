fn is_pop_order(pushA: &[i32], popA: &[i32]) -> bool{
	if pushA.len() == 0 || popA.len() == 0 || pushA.len() != popA.len() {
		return false;
	}
	let mut j = 0;
	let mut stack = vec!();
	for i in 0..pushA.len() {
		stack.push(pushA[i]);
		while !stack.is_empty() && popA[j] == *(stack.last().unwrap()) {
			stack.pop();
			j += 1;
		}
	}
	return stack.is_empty();
}