fn first_not_repeating_char(str: &str) -> i32 {
	if str.len() == 0 {
		return -1;
	}
	let mut counts: [u8; 256] =  [0; 256];
	let chars: Vec<char> = str.chars().collect();
	for c in &chars {
		let n = *c as usize;
		counts[n] += 1;
	}
	for i in 0..chars.len() {
		let n = chars[i] as usize;
		if counts[n] == 1 {
			let i: i32 = i as i32;
			return i;
		}
	} 
	return -1;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let r = first_not_repeating_char("google");
        println!("r is {}", r);
        assert_eq!(4, r);
    }
   
   
}