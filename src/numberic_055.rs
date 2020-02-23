use regex::Regex;

lazy_static! {
        static ref RE: Regex = Regex::new("^[-+]?\\d*(?:\\.\\d*)?(?:[eE][+\\-]?\\d+)?$").unwrap();
}

fn is_numberic(str: &str) -> bool {
    RE.is_match(str)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert!(is_numberic("+132"));
    	assert!(is_numberic("-132"));
    	assert!(!is_numberic("+1a32"));
		assert!(is_numberic("-1E-16"));
    	

    }	
}