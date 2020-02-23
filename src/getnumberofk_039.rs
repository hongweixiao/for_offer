fn get_number_of_k(array: &[i32], k: i32) -> i32 {
	if array.len() == 0 {
		return 0;
	}
	let index = binary_search(array, k, 0, array.len()-1);
	if index < 0 {
		return 0;
	}
	let mut count = 1;
	let mut i: usize = (index + 1) as usize;
	while i<array.len() && array[i] == k {
		count += 1;
		i += 1;
	}
	let mut i = index - 1;
	while i>=0 && array[i as usize] == k {
		count += 1;
		i -= 1;
	}
	return count;

}

fn binary_search(array: &[i32], k: i32, left: usize, right: usize) -> i32 {
	if array.len() == 0 || k > array[right] || k < array[left] {
		return -1;
	}
	let mid = (left + right) / 2;
	if array[mid] == k {
		return mid as i32;
	} else if array[mid] > k {
		return binary_search(array, k, left, mid-1);
	} else {
		return binary_search(array, k, mid+1, right);
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let array = [3, 3, 3, 4, 5, 9];
    	let r = get_number_of_k(&array, 3);
    	println!("-------------");
    	println!("{:?}", r);
    	println!("-------------");
    	assert_eq!(3, r);
    }

    
	
}
