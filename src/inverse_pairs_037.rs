

fn inverse_pairs(arr: &mut [i32]) -> i32 {
	if arr.len() == 0 {
		return 0;
	}
	let mut count = 0;
	merge_sort(arr, 0, arr.len()-1, &mut count);
	return count;
}

//归并排序
fn merge_sort(arr: &mut [i32], left: usize, right: usize, count: &mut i32) {
	if left >= right {
		return;
	}
	let mid = (left + right) / 2;
	merge_sort(arr, left, mid, count);
	merge_sort(arr, mid+1, right, count);
	merge(arr, left, mid, right, count);
}

fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize, count: &mut i32) {
	//临时排序的数组
	let mut temp_arr = vec![0i32; right-left+1];
	let mut l: usize = left; //左边部分下标
	let mut r: usize = mid + 1; //右边部分下标
	let mut i: usize = 0; //临时数组当前下标
	while l <= mid && r <= right {
		if arr[l] > arr[r] {
			*count += (mid-l+1) as i32; //如果前面的元素大于后面的，那么在前面元素之后的元素都能和后面的元素构成逆序对
			if *count > 1000000007 {
				*count %= 1000000007;
			}
			temp_arr[i] = arr[r];
			i += 1;
			r += 1; 
		} else {
			temp_arr[i] = arr[l];
			i += 1;
			l += 1;
		}
	}

	while l <= mid {  //左边未处理完的部分
		temp_arr[i] = arr[l];
		i += 1;
		l += 1;
	}
	while r <= right {  //右边未处理完的部分
		temp_arr[i] = arr[r];
		i += 1;
		r += 1;
	}

	//把临时数组的数据替换到原数组
	for k in 0..temp_arr.len() {
		arr[k+left] = temp_arr[k];
	}

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 0];
        let r = inverse_pairs(&mut arr);
        println!("r is {}", r);
        assert_eq!(7, r);
    }
   
	#[test]
   fn test_02() {
        let mut arr = [364,637,341,406,747,995,234,971,571,219,993,407,416,366,315,301,601,650,418,355,460,
                505,360,965,516,648,727,667,465,849,455,181,486,149,588,233,144,174,557,67,746,550,474,
                162,268,142,463,221,882,576,604,739,288,569,256,936,275,401,497,82,935,983,583,523,697,478,147,795,380,973,958,115,773,870,259,655,446,
                863,735,784,3,671,433,630,425,930,64,266,235,187,284,665,874,80,45,848,38,811,267,575];
        let r = inverse_pairs(&mut arr);
        assert_eq!(2519, r);
    }
   
}










