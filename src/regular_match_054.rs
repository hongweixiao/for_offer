pub fn is_match(s: String, p: String) -> bool {
	let mut dp = vec!(vec!(false; p.len()+1); s.len()+1);  //长度代表当前匹配到的字符串的长度，不是下标
	dp[0][0] = true;

	let s_chars = s.chars().collect::<Vec<char>>();
	let p_chars = p.chars().collect::<Vec<char>>();

	//预处理
	//s: ""  
	//p: "a*b*"
	//若p的当前字符p[i-1] == '*',把p[i-2..i-1]当空字符处理，此时dp[0][i] 取决于 dp[0][i-2];
	for i in 1..=p_chars.len() {
		if p_chars[i-1] == '*' {
			dp[0][i] = dp[0][i-2];
		}
	}

	(1..s_chars.len()+1).for_each(|i| {
		(1..p_chars.len()+1).for_each(|j| {
			if s_chars[i-1] == p_chars[j-1] || p_chars[j-1] == '.' {  //当前字母相等，或者pattern当前字母为‘.’
				dp[i][j] = dp[i-1][j-1];
				if i == 2 && j == 4 {
						println!("dp[1][3] is {}", dp[i-1][j-1]);
				}
			} else if p_chars[j-1] == '*' {
				if p_chars[j-2] != s_chars[i-1] && p_chars[j-2] != '.' {   //pattern的前一个字母 != str的当前字母，切不是 ‘.’, //则把 (pattern[j-2])* 看做是空串		
					dp[i][j] = dp[i][j-2];							
					if i == 1 && j == 3 {
						println!("dp[1][1] is {} ", dp[i][j-2]);
					}
				} else {
					//3种情况
					// 1、 把 (pattern[j-2])* 看做是空串
					// 2、 (pattern[j-2])* 看做是 (pattern[j-2])，即*当做1次
					// 3、 (pattern[j-2])* 看做是  (pattern[j-2])(pattern[j-2])(pattern[j-2]).. 即*当做多次		
							
					dp[i][j] = dp[i][j-2] || dp[i][j-1]	|| dp[i-1][j]; 
				}
			}
		})
	});
	dp[s_chars.len()][p_chars.len()]	
        
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	//assert!(is_match("aa".to_string(), "a*".to_string()));
    	//assert!(is_match("aaa".to_string(), "ab*ac*a".to_string()));
    	println!("{}",is_match("aaa".to_string(), "ab*ac*a".to_string()) );
    }	
}