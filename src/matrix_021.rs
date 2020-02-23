

fn print_matrix(matrix: &[&[i32]]) -> Vec<i32>{
	let mut v = vec!();
	if matrix.len() == 0 || matrix[0].len() == 0 {
		return v;
	}
	let mut top: usize = 0;
	let mut left: usize = 0;
	let mut bottom: usize = matrix.len() - 1;
	let mut right: usize = matrix[0].len() - 1;
	loop {
		//上边行
		for col in left..=right {
			v.push(matrix[top][col as usize]);
		}
		//向下走
		top += 1;
		//是否越界
		if top > bottom {
			break;
		}

		//右边行
		for row in top..=bottom {
			v.push(matrix[row as usize][right]);
		}
		//向左走
		right -= 1;
		//是否越界
		if left > right {
			break;
		}
		//下边行
		for col in (left..=right).rev() {
			v.push(matrix[bottom][col as usize]);
		}
		//向上走
		bottom -= 1;
		//检查越界
		if top > bottom {
			break;
		}
		//左边行
		for row in (top..=bottom).rev() {
			v.push(matrix[row as usize][left]);
		}
		//向右转
		left += 1;
		//检查越界
		if left > right {
			break;
		}
	}
	v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let a1 = [1, 2];
        let a2 = [3, 4];
        let a3 = [5, 6];
        let a = [&a1[..], &a2[..], &a3[..]];
        let r = print_matrix(&a);
        println!("--------------");
        println!("{:?}", r);
        println!("--------------");
    }


    
}
