fn find_greatest_sum_of_subArray(arr: &[i32]) -> i32 {
	if arr.len() == 0 {
		return 0;
	}
	let mut max = i32::min_value();  //最大和
	let mut curr_count = 0;  //当前和
	for i in 0..arr.len() {
		if curr_count < 0 {
			curr_count = arr[i]; //舍弃前面和小于0的部分
		} else {
			curr_count += arr[i];  //纳入计算
		}
		if curr_count > max {  //替换成新的最大值
			max = curr_count;
		}
	}
	return max;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let arr = [1, -2, 3, 10, -4, 7, 2, -5];
        let r = find_greatest_sum_of_subArray(&arr);
        assert_eq!(r, 18);
        
    }
     #[test]
    fn test_02() {
      let arr = [];
        let r = find_greatest_sum_of_subArray(&arr);
        assert_eq!(r, 0);
        
    }
   
}