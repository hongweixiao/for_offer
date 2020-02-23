
//分治法
pub fn find(target: i32, arr: &[&[i32]]) -> bool {
	if arr.len() == 0 {
		return false;
	}
	for i in  0..arr.len() {
		for j in (0..arr[i].len()).rev() {
			if arr[i][j] == target {
				return true;
			} else if arr[i][j] > target {
				continue;
			} else {
				break;
			}
		}
	}
	return false ;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_01() {
		let arr1: [i32;5] = [1, 2, 3, 5, 8];
		let arr2: [i32;5] = [2, 4, 5, 7, 9];
		let arr3: [i32;5] = [3, 9, 11, 12, 13];
		let y_array: [&[i32];3] = [&arr1, &arr2, &arr3];
        assert_eq!(true, find(3, &y_array));
        assert_eq!(false, find(14, &y_array));
    }
	
	#[test]
    fn test_find_02() {
		let y_array: [&[i32];0] = [];
        assert_eq!(false, find(3, &y_array));
        assert_eq!(false, find(14, &y_array));
    }
}