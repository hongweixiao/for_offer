
fn jump_floor(n: i32) -> i32{
	if n <= 2 {
		return n;
	}
	let mut one = 1;
	let mut two = 2;
	let mut sum = 0;
	for _ in 3..=n {
		sum = one + two;
		one = two;
		two = sum;
	} 
	return sum;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
		assert_eq!(0, jump_floor(0));
		assert_eq!(1, jump_floor(1));
		assert_eq!(2, jump_floor(2));
		assert_eq!(5, jump_floor(4));
		assert_eq!(8, jump_floor(5));
    }
	
}