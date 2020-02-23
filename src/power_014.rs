
fn power(base: f64, exponent: i32) -> f64{
	if exponent == 0 {
		return 1f64;
	}

	//绝对值
	let mut exp = exponent;
	if exponent < 0 {
		exp = -exponent;
	}

	//求幂次方
	let mut r = power_with_unsigned_exponent(base, exponent);

	//如果exponent小于0，则要取倒数
	if exponent < 0 {
		return 1f64/r;
	} 
	return r;
}

fn power_with_unsigned_exponent(base: f64, exponent: i32) -> f64{
	if exponent == 0 {
		return 1f64;
	}
	if exponent == 1 {
		return base;
	}
	//递归求一半值
	let mut r = power_with_unsigned_exponent(base, (exponent/2) as i32);
	//乘上另一半
	r *= r;
	//如果exponent是奇数，还要乘一次基数
	if exponent%2 != 0 {
		r *= base;
	}
	r
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(8f64, power(2f64, 3));
    	assert_eq!(1f64, power(2f64, 0));
    	assert_eq!(2f64, power(2f64, 1));
    	assert_eq!(27f64, power(3f64, 3));
    }
	
}