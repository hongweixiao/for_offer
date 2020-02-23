fn multiply(numbers: Vec<i32>) -> Vec<i32> {
	let length = numbers.len();
	if length <= 1 {
		return numbers;
	}

	let mut result: Vec<i32> = vec!(1;length);

	//左下半部分
	let mut i = 1_usize;
	loop {

		result[i] = result[i-1] * numbers[i-1];
		i += 1;
		if i == length {
			break;
		}
	}
	//右上半部分
    let mut j: i32 = (length-2) as i32;
    let mut t = 1;
    loop {
        t *= numbers[(j+1) as usize];
        result[j as usize] *= t;
        j -= 1;
        if j < 0 {
            break;
        }
    }
	result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	let numbers = vec!(1, 2, 3, 4, 5);
    	let r = multiply(numbers);
    	println!("{:?}", r);
    }	
}