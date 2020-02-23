fn permutation(str: &str) -> Vec<String> {
	let mut v = vec!();
	if str == "" {
		return v;
	}
	if str.len() == 1 {
		v.push(str.to_string());
		return v;
	}
	rec_permutation(str.chars().collect::<Vec<char>>(), 0, &mut v);
	//排序
	v.sort();
	v

}

fn rec_permutation(mut chars: Vec<char>, begin: usize, result: &mut Vec<String>) {
	if begin == chars.len() {
		let s: String = chars.iter().collect();
		if !result.contains(&s) {
			result.push(s);
		}
	}
	let mut i: usize = begin;
	while i<chars.len() {
		chars.swap(begin, i);
		let chars_new = chars.clone();
		rec_permutation(chars_new, begin+1, result);
		chars.swap(begin, i);
		i += 1;
	}

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let s1 = "abc";
        let r = permutation(s1);
        println!("{:?}", r);
        assert_eq!(r, vec!("abc", "acb", "bac", "bca", "cab", "cba"));
        
    }
     #[test]
    fn test_02() {
        let s1 = "";
        let r = permutation(s1);
        println!("{:?}", r);
        let v: Vec<String> = vec!();
        assert_eq!(r, v);
        
    }
   
}