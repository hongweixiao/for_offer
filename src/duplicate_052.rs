use std::collections::HashSet;

fn duplicate(numbers: &[i32]) -> Option<i32> {
	if numbers.len() <= 1 {
		return None;
	}
	let mut set = HashSet::new();
	for n in numbers {
		if set.contains(n) {
			return Some(*n);
		}
		set.insert(n);
	}
	None
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	let arr = [2, 3, 4, 2, 3, 4];
    	assert_eq!(duplicate(&arr), Some(2));
    }	
}