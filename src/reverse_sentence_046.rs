fn reverse_sentence(mut str: String) -> String {
	if str.len() <= 1 {
		return str;
	}
	//翻转整个字符串
	str = str.chars().rev().collect();

	
	//翻转单个单词
	let mut left: usize = 0;
	let mut right: usize = 0;

	let mut chars = str.chars().collect::<Vec<char>>();
	while right<chars.len() {
		left = right;
		while chars[left] == ' ' {
			left += 1;
		}
		right = left;
		while right<chars.len() && chars[right] != ' ' {
			right += 1;
		}
		revert(&mut chars, left, right-1);
	}
	chars.iter().collect::<String>()
}

fn revert(chars: &mut Vec<char>, mut left: usize, mut right: usize) {
	while left < right {
		chars.swap(left, right);
		left += 1;
		right -= 1;
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
	
    	let str = "student. a am I".to_string();
    	let r = reverse_sentence(str);
    	println!("-------------");
    	println!("{:?}", r);
    	println!("-------------");
    }	
}