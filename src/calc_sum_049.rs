fn calc_sum(n: i32) -> i32 {
	let l: f64 = 1_64;
    let r: f64 = n as f64;
    let sum: i32 = ((l+r)*((r-l+1_f64)/2_f64)) as i32;
    sum

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
    	assert_eq!(calc_sum(100), 5050);
    }	
}