use super::matrix::Matrix;

pub fn decompose(mat: Matrix) {
	let (rows, cols) = mat.get_size();
	let mut U = mat.clone();
	let mut L = Matrix::identity(rows, cols).unwrap();
	L.print();
	for i in 1..rows {
		for j in 0..i {
			println!("Row: {}. Col {}", i, j);
			let multiplier = U.get_value(i, j).unwrap() / U.get_value(j, j).unwrap();
			println!("Multiplier {}", multiplier);
			let new_row = (U.get_row(i).unwrap()
				- U.get_row(j).unwrap().multiplied_by_scalar(multiplier)).unwrap();

			U.set_row(i, new_row.get_data()).unwrap();
			L.set_value(i, j, -multiplier).unwrap();
			
		}
	}
	U.print();
	L.print();
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_decompose() {
		let data: Vec<f64> = vec![1.0, 1.0, 2.0, 1.0, -2.0, 3.0, -1.0, 3.0, 1.0];
		decompose(Matrix::new(3, 3, data).unwrap());
	}
}
