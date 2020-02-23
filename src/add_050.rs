fn add(n1: i32, n2: i32) -> i32 {
	let mut n1 = n1;
	let mut n2 = n2;
	while n2 !=0 {
		let t = n1 ^ n2; //不计算进位
		n2 = (n1 & n2) << 1; // 进位
		n1 = t;
	}
	n1
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(add(0, 3), 3);
    	assert_eq!(add(1, 2), 3);
    	assert_eq!(add(10, 2), 12);
    	assert_eq!(add(5, 7), 12);
    }	
}