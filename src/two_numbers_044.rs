fn find_numbers_with_sum(array: &[i32], sum: i32) -> Vec<i32> {
	let mut v: Vec<i32> = vec!();
	if array.len() <= 1 || array[0] > sum {
		return v;
	}
	let mut left: usize = 0;
	let mut right: usize = array.len() - 1;
	while left < right {
		let curr_sum = array[left] + array[right];
		if curr_sum == sum {
			v.push(array[left]);
			v.push(array[right]);
			return v;
		} else if curr_sum < sum {
			left += 1;
		} else {
			right -= 1;
		}
	}
	v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
		let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    	let sum = 11;
    	let r = find_numbers_with_sum(&array, sum);
    	println!("-------------");
    	println!("{:?}", r);
    	println!("-------------");
    }	
}