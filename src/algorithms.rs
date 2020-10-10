use super::error::MathMatrixError;
use super::error::MathMatrixErrorKind::*;
use super::matrix::Matrix;

pub fn decompose(mat: &Matrix) -> Result<(Matrix, Matrix), MathMatrixError> {
	let (rows, cols) = mat.get_size();
	let mut u = mat.clone();
	let mut l = Matrix::identity(rows, cols).unwrap();
	for i in 1..rows {
		for j in 0..i {
			let numerator = u.get_value(i, j).unwrap();
			let denominator = u.get_value(j, j).unwrap();
			if denominator == 0.0 {
				return Err(MathMatrixError::new(
					FailedToDecompose,
					"Found zero".to_owned(),
				));
			}
			let multiplier = numerator / denominator;
			l.set_value(i, j, multiplier).unwrap();
			let mut tmp_mat = Matrix::identity(rows, cols).unwrap();
			tmp_mat.set_value(i, j, -multiplier).unwrap();
			u = tmp_mat.multiplied_by_matrix(&u).unwrap();
		}
	}
	return Ok((l, u));
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_decompose() {
		let data: Vec<f64> = vec![
			1.0, 1.0, 3.0, 0.0, 1.0, -2.0, -6.0, 0.0, -1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
		];
		let mat = Matrix::new(4, 4, data).unwrap();
		let (l, u) = decompose(&mat).unwrap();
		assert_eq!(
			l.get_data(),
			[1.0, 1.0, 3.0, 0.0, 0.0, 1.0, 3.0, -0.0, 0.0, 0.0, 1.0, -0.0, 0.0, 0.0, 0.0, 1.0]
		);
		assert_eq!(
			u.get_data(),
			[1.0, 0.0, 0.0, 0.0, 1.0, -3.0, 0.0, 0.0, -1.0, 4.0, -8.0, 0.0, 0.0, 0.0, 0.0, 1.0]
		);
		assert_eq!(l.multiplied_by_matrix(&u).unwrap(), mat)
	}
}
