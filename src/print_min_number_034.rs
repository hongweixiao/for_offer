fn print_min_number(numbers: &mut [i32]) -> String {
	if numbers.len() == 0 {
		return String::new();
	}
	numbers.sort_by(|a, b| {   //排序
			let s1 = format!("{}{}", a, b);
			let s2 = format!("{}{}", b, a);
			s1.cmp(&s2)
	});
	return numbers.iter()
			.map(|n| n.to_string())
			.collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let mut numbers = [3, 5, 1, 4, 2];
        let r = print_min_number(&mut numbers);
        assert_eq!("12345".to_string(), r);
    }
   
   
}