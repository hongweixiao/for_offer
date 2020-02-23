
fn rent_cover(n: i32) -> i32{
	if n <= 2 {
		return n;
	}
	let mut a = 1;
	let mut b = 2;
	for _ in 3..=n {
		b += a;
		a = b - a;
	}
	return b;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(4, rent_cover(3));
    }
	
}