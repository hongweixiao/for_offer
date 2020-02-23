fn number_of_1_between_1_and_n(n: u64) -> u64 {
	let mut count = 0_u64;  
	let mut i = 1_u64;  //当前位（个十百）
	while i<=n {
		let left = n/(i*10); //高位值
		let curr = (n/i)%10; //当前位的值
		let right = n-(n/i)*i; //低位值
		if curr == 0 {
			//由高位决定
			count += left*i;
		} else if curr == 1 {
			//由高位和地位决定
			count += left*i + (right+1);
		} else {
			//由高位决定（right+1）
			count += (left+1)*i;
		}
		i *= 10;  //进位
	}
	count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let n = 21345_u64;
        assert_eq!(18821_u64, number_of_1_between_1_and_n(n));
    }
   
   
}