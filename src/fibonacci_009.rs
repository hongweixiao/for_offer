
fn fibnacci(n: i32) -> i32{
	if n == 0 || n == 1 {
		return n;
	}
	let mut sum = 0;
	let mut one = 0;
	let mut two = 1;
	for _ in 2..=n {
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
		assert_eq!(0, fibnacci(0));
		assert_eq!(1, fibnacci(1));
		assert_eq!(1, fibnacci(2));
		assert_eq!(2, fibnacci(3));
    }
	
	
}