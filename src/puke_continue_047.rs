use std::collections::BTreeSet;

fn is_continuous(numbers: &[i32]) -> bool {
	if numbers.len() != 5 {
		return false;
	}

	let mut set = BTreeSet::new();
	let mut count = 0;
	for n in numbers {
		if *n == 0 {
			count += 1;
		} else {
			set.insert(*n);
		}
	}

	if count+set.len() != 5 {
		return false;
	}
	// TreeSet map_first_last尚未实现稳定版

	let v = set.iter().map(|&n| n).collect::<Vec<i32>>();
	println!("v is {:?}", v);

	if v.last().unwrap() - v.first().unwrap() < 5 {
		return true;
	}

	return false;

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let numbers = [0, 0, 1, 2, 5];
    	assert!(is_continuous(&numbers));
    }	
}