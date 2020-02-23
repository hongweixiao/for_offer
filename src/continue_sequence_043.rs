fn find_continue_sequence(sum: i32) -> Vec<Vec<i32>> {
	let mut result: Vec<Vec<i32>> = vec!();
	if sum <= 2 {
		return result;
	}
	let mut left = 1;
	let mut right = 2;

	while left<right && right<sum {
		let curr_sum = calc_sequence_sum(left, right);
		if curr_sum == sum {
			let mut v = vec!();
			for i in left..=right {
				v.push(i);
			}
			result.push(v);
			left += 1;
		} else if curr_sum < sum {
			right += 1;
		} else {
			left += 1;
		}
	} 
	result
}

fn calc_sequence_sum(left: i32, right: i32) -> i32 {
	let l: f64 = left as f64;
	let r: f64 = right as f64;
	let sum: i32 = ((l+r)*((r-l+1_f64)/2_f64)) as i32;
	sum
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let sum = 101;
    	let r = find_continue_sequence(sum);
    	println!("-------------");
    	println!("{:?}", r);
    	println!("-------------");
    }	
}

