
fn reorder_array( array: &mut [i32]) {
	let len = array.len();
	if len <= 1 {
		return;
	}
	let mut i: usize = 0;
	while i<len {
		if array[i]%2 != 0 {
			//奇数 i继续往后找
			i += 1;
		} else {
			//偶数停下来，找到该偶数后面的奇数
			let mut j: usize = i+1;
			while(array[j]%2 == 0) { //j是偶数则后移
				if j == len-1 { //最后一位是偶数，表示移完了
					return;
				}
				j += 1;
			}
			//此时array[j] 为奇数
			let temp = array[j];
			while i<j {
				array[j] = array[j-1];
				j -= 1;
			}
			array[i] = temp;
		}
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	let mut array = [1, 3, 4, 6, 5, 8];
    	reorder_array(&mut array);
    	assert_eq!([1, 3, 5, 4, 6, 8], array );
    }
    #[test]
    fn test_02() {
    	let mut array = [1, 2, 4, 6, 8];
    	reorder_array(&mut array);
    	assert_eq!([1, 2, 4, 6, 8], array );
    }
	
}