
fn jump_floorn(n: i32) -> i32{
	if n <= 2 {
		return n;
	}
	let n  = n as usize;
	let mut a = vec![0; n+2];
	a[0] = 1;
	a[1] = 1;
	a[2] = 2;
	for i in 3..=n {
		let i = i as usize;
		a[i] = 2*a[i-1];
	}
	return a[n as usize];
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(5, jump_floorn(4));
		assert_eq!(8, jump_floorn(4));
    }
	
}