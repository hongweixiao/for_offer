fn get_ugly_number(n: i32) -> i32 {
	let n = n as usize;
	if n == 0 {
		return 0;
	}
	let mut v = vec!();
	v.push(1);
	let mut i2: usize = 0;
	let mut i3: usize = 0;
	let mut i5: usize = 0;
	while v.len() < n {
		let n2 = 2 * (*(v.get(i2).unwrap()));
		let n3 = 3 * (*(v.get(i3).unwrap()));
		let n5 = 5 * (*(v.get(i5).unwrap()));
		let min = n2.min(n3.min(n5));
		if min == n2 {
			i2 += 1;
		} else if min == n3 {
			i3 += 1;
		} else {
			i5 += 1;
		}
		if v.last() != Some(&min) {
			v.push(min);
		}
	}
	return *v.last().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let r = get_ugly_number(7);
        println!("r is {}", r);
        assert_eq!(8, r);
    }
   
   
}