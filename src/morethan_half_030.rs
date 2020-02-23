fn morethan_half_one(arr: &[i32]) -> i32 {
	if arr.len() == 0 {
		return 0;
	}
	if arr.len() == 1 {
		return arr[0];
	}
	let mut pre_value = arr[0];
	let mut count = 1;
	for i in 1..arr.len() {
		if pre_value == arr[i] {
			count += 1;
		} else {
			count -= 1;
			if count == 0 {
				if i == arr.len() - 1 {  //最后一个值了，count还==0，说明不存在超过一半的数
					return 0;
				}
				pre_value = arr[i];
				count = 1;
			}
		}
	}
	if count <= 0 {
		return 0;
	}
	return pre_value;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let arr = [1,2,3,2,2,2,5,4,2];
        let r = morethan_half_one(&arr);
        assert_eq!(2, r);
        
    }
     #[test]
    fn test_02() {
        let arr: [i32;0] = [];
        let r = morethan_half_one(&arr);
        assert_eq!(0, r);
        
        
    }
     #[test]
    fn test_03() {
        let arr: [i32;1] = [3];
        let r = morethan_half_one(&arr);
        assert_eq!(3, r);
        
        
    }
   
}