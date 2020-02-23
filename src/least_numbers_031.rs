use std::collections::BinaryHeap;

fn get_least_numbers(arr: &[i32], k: usize) -> Vec<i32>{
	if arr.len() == 0 || k == 0 || arr.len() < k {
		return vec!();
	}
	let mut heap = BinaryHeap::new();  //排序二叉堆
	for i in 0..arr.len() {
		if i<k {
			heap.push(arr[i]);
		} else {
			if heap.peek().unwrap() > &arr[i] {
				heap.pop();
				heap.push(arr[i]);
			}
		}
	}
	return heap.into_vec();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let arr = [4,5,1,6,2,7,3,8];
        let r = get_least_numbers(&arr, 4);
        assert_eq!(r, vec!(4, 3, 1, 2));
        
    }
     #[test]
    fn test_02() {
      let arr = [4,5,1,6,2,7,3,8];
        let r = get_least_numbers(&arr, 10);
        assert_eq!(r, vec!());
        
    }
   
}