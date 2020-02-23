
fn number_of1(mut n: i32) -> i32{
	let mut count = 0;
	while n!=0 {
		count += 1;
		n = n&(n-1);  //清除最右边的1
	}
	count
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(0, number_of1(0));
    	assert_eq!(1, number_of1(2));
    	assert_eq!(2, number_of1(3));
		assert_eq!(1, number_of1(8));
		assert_eq!(3, number_of1(7));
    }
	
}