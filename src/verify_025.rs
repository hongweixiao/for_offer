fn verify_squence_of_BST(sequence: &[i32]) -> bool{
	if sequence.len() == 0 {
		return false;
	}
	return verify(sequence, 0, sequence.len()-1);
}

fn verify(sequence: &[i32], left: usize, right: usize) -> bool {
	if left >= right {
		return true;
	}
	let mut i: usize = left;
	while i < right {
		if sequence[i] > sequence[right] {  //找到左右子树部分
			break;
		}
		i += 1;
	}
	println!("i is {}, left is {}, right is {}", i, left, right);
	for j in i..right {   
		if sequence[j] < sequence[right] {  //右子树若存在小于根节点的树，则false
			println!("j is {} right is {}", j, right);
			return false;
		}
	}
	return verify(sequence, left, i-1) && verify(sequence, i, right-1);

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let sequence = [3, 4, 9, 5, 12, 11, 10];
        println!("{}", verify_squence_of_BST(&sequence));
        // assert!(verify_squence_of_BST(&sequence));
    }
   
}