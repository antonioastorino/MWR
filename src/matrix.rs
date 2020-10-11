use super::error::MathMatrixError;
use super::error::MathMatrixErrorKind::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
	rows: usize,
	cols: usize,
	data: Vec<f64>,
}

impl std::ops::Add for Matrix {
	type Output = Result<Matrix, MathMatrixError>;

	fn add(self, other: Matrix) -> Result<Matrix, MathMatrixError> {
		if self.get_size() == other.get_size() {
			let mut new_data = vec![0f64; self.rows * self.cols];
			for i in 0..(self.rows * self.cols) {
				new_data[i] = self.data[i] + other.data[i];
			}
			Ok(Matrix {
				rows: self.rows,
				cols: self.cols,
				data: new_data,
			})
		} else {
			Err(MathMatrixError::new(
				SizeMismatch,
				"Operation not allowed between matrices with different sizes".to_owned(),
			))
		}
	}
}

impl std::ops::Sub for Matrix {
	type Output = Result<Matrix, MathMatrixError>;

	fn sub(self, other: Matrix) -> Result<Matrix, MathMatrixError> {
		if self.get_size() == other.get_size() {
			let mut new_data = vec![0f64; self.rows * self.cols];
			for i in 0..(self.rows * self.cols) {
				new_data[i] = self.data[i] - other.data[i];
			}
			Ok(Matrix {
				rows: self.rows,
				cols: self.cols,
				data: new_data,
			})
		} else {
			Err(MathMatrixError::new(
				SizeMismatch,
				"Operation not allowed between matrices with different sizes".to_owned(),
			))
		}
	}
}

impl Matrix {
	/* Column major. Example:
		- rows: 3
		- cols: 2
		- data: [a b c d e f]
		a d
		b e
		c f
	*/
	pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MathMatrixError> {
		if rows * cols == 0 {
			return Err(MathMatrixError::new(
				FailedToInitialize,
				"Rows and columns must be lager than 0".to_owned(),
			));
		}
		if rows * cols == data.len() {
			Ok(Self { rows, cols, data })
		} else {
			Err(MathMatrixError::new(
				FailedToInitialize,
				format!(
					"Size of data != rows * cols: {} != {}",
					data.len(),
					rows * cols
				),
			))
		}
	}

	pub fn zeros(rows: usize, cols: usize) -> Result<Self, MathMatrixError> {
		return Self::new(rows, cols, vec![0f64; rows * cols]);
	}

	pub fn identity(rows: usize, cols: usize) -> Result<Self, MathMatrixError> {
		let mut data = vec![0f64; rows * cols];
		for j in 0..cols {
			for i in 0..rows {
				data[i + rows * j] = if i == j { 1.0 } else { 0.0 }
			}
		}
		return Self::new(rows, cols, data);
	}

	pub fn set_value(&mut self, row: usize, col: usize, value: f64) -> Result<(), MathMatrixError> {
		if row > self.rows {
			return Err(MathMatrixError::new(
				OutOfBoundary,
				format!("Row {} > {}", row, self.rows),
			));
		}
		if col > self.cols {
			return Err(MathMatrixError::new(
				OutOfBoundary,
				format!("Column {} > {}", col, self.cols),
			));
		} else {
			self.data[col * self.rows + row] = value;
		}
		Ok(())
	}

	pub fn get_value(&self, row: usize, col: usize) -> Result<f64, MathMatrixError> {
		if row > self.rows {
			return Err(MathMatrixError::new(
				OutOfBoundary,
				format!("Row {} > {}", row, self.rows),
			));
		}
		if col > self.cols {
			return Err(MathMatrixError::new(
				OutOfBoundary,
				format!("Column {} > {}", col, self.cols),
			));
		} else {
			return Ok(self.data[col * self.rows + row]);
		}
	}

	pub fn multiplied_by_matrix(&self, other: &Matrix) -> Result<Self, MathMatrixError> {
		if self.cols != other.rows {
			return Err(MathMatrixError::new(
				SizeMismatch,
				"Multiplication allowed for NxM * MxO".to_owned(),
			));
		}
		let rows = self.rows;
		let cols = other.cols;
		let mut out_mat = Matrix::new(rows, cols, vec![0f64; rows * cols]).unwrap();
		for i in 0..self.rows {
			for j in 0..other.cols {
				let mut sum: f64 = 0.;
				for k in 0..self.cols {
					sum += self.get_value(i, k)? * other.get_value(k, j)?;
				}
				out_mat.set_value(i, j, sum).unwrap();
			}
		}
		return Ok(out_mat);
	}

	pub fn multiplied_by_scalar(&self, scalar: f64) -> Self {
		let mut output_matrix = self.clone();
		for i in 0..self.rows {
			for j in 0..self.cols {
				output_matrix
					.set_value(i, j, self.get_value(i, j).unwrap() * scalar)
					.unwrap();
			}
		}
		return output_matrix;
	}

	pub fn transposed(&self) -> Self {
		// Create an empty matrix with transposed size
		let mut transposed_matrix = Self::zeros(self.cols, self.rows).unwrap();
		for j in 0..self.cols {
			for i in 0..self.rows {
				transposed_matrix
					.set_value(j, i, self.get_value(i, j).unwrap())
					.ok();
			}
		}
		return transposed_matrix;
	}

	pub fn decompose(&self) -> Result<(Matrix, Matrix), MathMatrixError> {
		let (rows, cols) = self.get_size();
		if rows != cols {
			return Err(MathMatrixError::new(
				OperationNotPermitted,
				"LU decomposition allowed only for square matrices".to_owned(),
			));
		}
		let mut u = self.clone();
		let mut l = Matrix::identity(rows, cols)?;
		for i in 1..rows {
			for j in 0..i {
				let numerator = u.get_value(i, j)?;
				let denominator = u.get_value(j, j)?;
				if denominator == 0.0 {
					return Err(MathMatrixError::new(
						FailedToDecompose,
						"Found zero".to_owned(),
					));
				}
				let multiplier = numerator / denominator;
				l.set_value(i, j, multiplier)?;
				let mut tmp_mat = Matrix::identity(rows, cols)?;
				tmp_mat.set_value(i, j, -multiplier)?;
				u = tmp_mat.multiplied_by_matrix(&u)?;
			}
		}
		return Ok((l, u));
	}

	pub fn invert(&self) -> Result<Matrix, MathMatrixError> {
		let size = self.rows;
		let (l_mat, u_mat) = self.decompose()?;
		/*
		Resource: https://www.youtube.com/watch?v=dza5JTvMpzk
		- Create one column at a time of the identity matrix.
		- Find the corresponding column of the inverse matrix.
		- Combine all the resulting columns.
		*/
		// Solve for y L*Y = I using "forward substitution"
		let mut y_mat = Matrix::identity(size, size)?;
		for col in 0..size {
			for row in (col + 1)..size {
				let mut elem = -l_mat.get_value(row, col)?;
				let mut computation_message = format!(
					"Y{row},{col} = L{row},{col} [{l_row_col}]",
					row = row,
					col = col,
					l_row_col = elem
				);
				for i in (col + 1)..row {
					let l_row_i = l_mat.get_value(row, i)?;
					let y_i_col = y_mat.get_value(i, col)?;
					elem += -l_row_i * y_i_col;
					computation_message = format!(
						"{} - L{row},{i}[{l_row_i}] * Y{i},{col}[{y_i_col}]",
						computation_message,
						row = row,
						col = col,
						i = i,
						l_row_i = l_row_i,
						y_i_col = y_i_col,
					);
				}
				y_mat.set_value(row, col, elem).ok();
				println!("{}", computation_message);
				println!("Elem: {}", elem);
			}
		}

		// Solve for A (= mat^(-1)) U*A = Y using "back substitution"
		// 	for row in (0..rows).rev() {
		// //
		// 	}
		// let mut inverted_matrix = Matrix::zeros(cols, rows)?;
		let mut x_mat = Matrix::zeros(size, size)?;
		for col in 0..size {
			for row in (0..size).rev() {
				let mut elem = y_mat.get_value(row, col)?;
				let divider = u_mat.get_value(row, row)?;
				let mut computation_message = format!(
					"X{row},{col} = 1/U{row},{row}*(Y{row},{col}",
					row = row,
					col = col
				);
				for i in (row + 1)..size {
					computation_message = format!(
						"{} - U{row},{i} * X{i},{col}",
						computation_message,
						row = row,
						col = col,
						i = i
					);
					elem += -u_mat.get_value(row, i)? * x_mat.get_value(i, col)?;
				}
				x_mat.set_value(row, col, elem / divider)?;
				println!("{})", computation_message);
			}
		}
		return Ok(x_mat);
	}

	pub fn get_size(&self) -> (usize, usize) {
		return (self.rows, self.cols);
	}

	pub fn get_data(&self) -> Vec<f64> {
		return self.data.clone();
	}

	pub fn print(&self) {
		for i in 0..self.rows {
			for j in 0..self.cols {
				print!("{:.3}\t", self.get_value(i, j).unwrap());
			}
			println!();
		}
		println!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let mat = Matrix::new(2, 3, vec![0.1, 0.3, 5.0, 6.0, 0.0, 0.0]).unwrap();
		assert_eq!(mat.rows, 2);
		assert_eq!(mat.cols, 3);
		assert_eq!(mat.data, vec![0.1, 0.3, 5.0, 6.0, 0.0, 0.0]);
	}

	#[test]
	fn test_identity() {
		let mat = Matrix::identity(3, 4).unwrap();
		assert_eq!(mat.rows, 3);
		assert_eq!(mat.cols, 4);
		assert_eq!(
			mat.data,
			vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
		);
	}
	#[test]
	fn test_zeros() {
		let mat = Matrix::zeros(2, 1).unwrap();
		assert_eq!(mat.rows, 2);
		assert_eq!(mat.cols, 1);
		assert_eq!(mat.data, vec![0.0, 0.0]);
	}

	#[test]
	fn test_transpose() {
		let mat = Matrix::new(2, 3, vec![0.1, 0.3, 5.0, 6.0, 0.0, 0.0])
			.unwrap()
			.transposed();
		assert_eq!(mat.rows, 3);
		assert_eq!(mat.cols, 2);
		assert_eq!(mat.data, vec![0.1, 5.0, 0.0, 0.3, 6.0, 0.0]);
	}

	#[test]
	fn test_set_value() {
		let mut mat = Matrix::new(2, 3, vec![0.1, 0.3, 5.0, 6.0, 0.0, 0.0]).unwrap();
		mat.set_value(2, 0, 100.).unwrap();
		mat.set_value(1, 1, 10.).unwrap();
		assert_eq!(mat.data[3], 10.0);
	}

	#[test]
	fn test_new_matrix_error() {
		let mat = Matrix::new(2, 3, vec![0.1, 0.3, 5.0, 6.0, 0.0]).unwrap_err();
		assert_eq!(
			mat.to_string(),
			"FailedToInitialize error: Size of data != rows * cols: 5 != 6"
		);
	}

	#[test]
	fn test_multiplied_by_matrix() {
		let mat1 = Matrix::new(3, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, -1.0]).unwrap();
		let mat2 = Matrix::new(3, 2, vec![2.0, 1.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
		let calculated = mat1.multiplied_by_matrix(&mat2).unwrap();
		let expected = Matrix::new(3, 2, vec![4.0, 0.0, 3.0, 4.0, 0.0, 1.0]).unwrap();
		assert_eq!(calculated, expected);
	}

	#[test]
	fn test_multiplied_by_scalar() {
		let mat1 = Matrix::new(3, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, -1.0]).unwrap();
		let mat2 = mat1.multiplied_by_scalar(2.0);
		assert_eq!(
			mat2.data,
			vec![2.0, 0.0, 2.0, 4.0, 0.0, 2.0, 2.0, 0.0, -2.0]
		);
	}

	#[test]
	fn test_decompose() {
		let l_original = Matrix::new(
			4,
			4,
			vec![
				1.0, 2.0, -3.0, 4.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
			],
		)
		.unwrap();
		l_original.print();
		let u_original = Matrix::new(
			4,
			4,
			vec![
				3.0, 0.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 3.0, -2.0, 1.0, 0.0, 1.0, -7.0, 2.0, 1.0,
			],
		)
		.unwrap();
		u_original.print();

		let mat = l_original.multiplied_by_matrix(&u_original).unwrap();
		println!("{:?}", mat.get_data());
		let (l, u) = mat.decompose().unwrap();
		assert_eq!(l, l_original,);
		assert_eq!(u, u_original);
		assert_eq!(l.multiplied_by_matrix(&u).unwrap(), mat)
	}

	#[test]
	fn test_invert() {
		let data: Vec<f64> = vec![
			3.0, 6.0, -9.0, 12.0, 2.0, 5.0, -6.0, 8.0, 3.0, 4.0, -8.0, 12.0, 1.0, -5.0, -1.0, 5.0,
		];
		let mat = Matrix::new(4, 4, data).unwrap();
		let inv_mat = mat.invert().unwrap();
		let identity = inv_mat.multiplied_by_matrix(&mat).unwrap();
		identity.print();
		assert_eq!(identity, Matrix::identity(4, 4).unwrap());
	}
}
