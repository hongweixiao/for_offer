fn left_rotate_str(str: String, n: usize) -> String {
	if str.len() <= 1 || n == 0 {
		return str;
	}
	let mut n = n;
	if str.len() < n {
		n = n%str.len();
	}
	println!("n = {}", n);
	let mut left = String::new();
	let mut right = String::new();
	let mut i = 0;
	for c in str.chars() {
		if i<=n {
			right.push(c);
		} else {
			left.push(c);
		}
		i += 1;
	}
	left+&right
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let str = "abc123".to_string();
    	let r = left_rotate_str(str, 1);
    	println!("-------------");
    	println!("{:?}", r);
    	println!("-------------");
    }	
}