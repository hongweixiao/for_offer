//约瑟夫环 (链表法)
fn last_remaining(n: i32, m: i32) -> i32 {
	if n == 0 || m == 0 {
		return -1;
	}
	let mut v = (0..n).collect::<Vec<i32>>();
	let mut index = 0;
	while v.len() != 1 {
		index = (index+m-1)%(v.len() as i32);
		v.remove(index as usize);
	}
	v[0]
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(last_remaining(5,3), 3);
    }	
}