use std::collections::HashMap;

fn find_nums_appear_once(array: &[i32], num1: &mut [i32], num2: &mut [i32]) {
	if array.len() == 0 {
		return;
	}
	let mut map = HashMap::new();
	for n in array {
		let counter = map.entry(n).or_insert(0);
    	*counter += 1;
	}

	let mut i = 0;

	for n in array {

		if map.get(&n) == Some(&1) {
			if i == 0 {
				num1[0] = *n;
			} else {
				num2[0] = *n;
			}
			i += 1;
		} 
	}

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let array = [2,4,3,6,3,2,5,5];
    	let mut num1 = [0];
    	let mut num2 = [0];
    	find_nums_appear_once(&array, &mut num1, &mut num2);
    	println!("-------------");
    	println!("{:?}  {:?}", num1, num2);
    	println!("-------------");
    }

    
	
}

