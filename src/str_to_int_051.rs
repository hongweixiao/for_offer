fn convert(str: String) -> i32 {
	if str.trim().is_empty() {
		return 0;
	}
	let mut negative = 1;  //正负符号
	let mut result = 0; //结果

	for (i, c) in str.chars().enumerate() {
		if i==0 && c == '+' {
			continue;
		}
		if i==0 && c == '-' {
			negative = - 1;
			continue;
		}
		if !c.is_digit(10) {
			return 0;
		}
		
		//检查越界
		if negative*result >  i32::max_value()/10 {
			return 0;
		}

		let digit = c.to_digit(10).unwrap() as i32;  

		if negative*result == i32::max_value()/10 { 
			//为正判断digit是否大于7 为负判断digit是否小于-8
			if (negative+1)/2 + digit > 8 {
				return 0;
			}
		}
		result = result*10 + negative*digit;  //累加
	}
	result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(convert("123".to_string()), 123);
    	assert_eq!(convert("+2147483647".to_string()), 2147483647);
    	assert_eq!(convert("2147483648".to_string()), 0);
    	assert_eq!(convert("-2147483648".to_string()), -2147483648);
    	assert_eq!(convert("-2147483649".to_string()), 0);
    	assert_eq!(convert("a147483649".to_string()), 0);
    	assert_eq!(convert("12 3".to_string()), 0);
    }	
}
