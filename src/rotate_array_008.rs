
fn min_number_in_rotate_array(arr: &[i32]) -> i32{
	if arr.len() == 0 {
		return 0;
	}
	if arr.len() == 1 {
		return arr[0];
	}
	let mut left: usize = 0;
	let mut right: usize = arr.len() - 1;

	//如果没旋转
	if arr[right] > arr[left] {
		return arr[0];
	}

	let mid: usize = (left + (right - left) / 2) as usize;

	while left < right {
		if arr[mid] > arr[mid+1] {
			return arr[mid+1];
		}
		if arr[mid-1] > arr[mid] {
			return arr[mid];
		}
		if arr[mid] > arr[left] {
			left = mid + 1;  
		} else {
			right = mid - 1;
		}
	}
	return arr[left];
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	    let arr = [3, 4, 5, 1, 2];
	    assert_eq!(1, min_number_in_rotate_array(&arr));
	    let arr = [2, 3, 4];
	    assert_eq!(2, min_number_in_rotate_array(&arr));
		
    }
	
	#[test]
    fn test_02() {
		let arr = [3, 4, 5, 0, 1, 2];
	    assert_eq!(0, min_number_in_rotate_array(&arr));
	    let arr = [];
	    assert_eq!(0, min_number_in_rotate_array(&arr));
    }
}