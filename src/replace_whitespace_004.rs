

pub fn replace(input: &str) -> String {
	input.replace(" ", "%20")
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace_01() {
		assert_eq!(String::from("a%20b%20c"), replace("a b c"));
    }
	
	#[test]
    fn test_replace_02() {
		assert_eq!(String::from("a%20%20b%20c"), replace("a  b c"));
    }
}