use super::error::MathMatrixError;
use super::error::MathMatrixErrorKind::*;

#[derive(Debug, Clone)]
pub struct Matrix {
	rows: usize,
	cols: usize,
	data: Vec<f64>,
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

	pub fn get_column(&self, index: usize) -> Self {
		let rows = self.rows;
		let cols = 1;
		let mut data = vec![0f64; rows];
		for i in 0..rows {
			data[i] = self.get_value(i, index).unwrap();
		}
		return Self { rows, cols, data };
	}

	pub fn get_row(&self, index: usize) -> Self {
		let rows = 1;
		let cols = self.cols;
		let mut data = vec![0f64; cols];
		for i in 0..cols {
			data[i] = self.get_value(index, i).unwrap();
		}
		return Self { rows, cols, data };
	}

	pub fn set_column(
		&mut self,
		column_number: usize,
		new_column: Vec<f64>,
	) -> Result<(), MathMatrixError> {
		let column_length = new_column.len();
		if column_length != self.rows {
			return Err(MathMatrixError::new(
				SizeMismatch,
				format!(
					"Passed column of length {} but expected {}",
					column_length, self.rows
				),
			));
		} else {
			for i in 0..column_length {
				self.set_value(i, column_number, new_column[i]).unwrap();
			}
		}
		Ok(())
	}

	pub fn set_row(&mut self, row_number: usize, new_row: Vec<f64>) -> Result<(), MathMatrixError> {
		let row_length = new_row.len();
		if row_length != self.cols {
			return Err(MathMatrixError::new(
				SizeMismatch,
				format!(
					"Passed row of length {} but expected {}",
					row_length, self.cols
				),
			));
		} else {
			for i in 0..row_length {
				self.set_value(row_number, i, new_row[i]).unwrap();
			}
		}
		Ok(())
	}

	pub fn print(&self) {
		for i in 0..self.rows {
			for j in 0..self.cols {
				print!("{}\t", self.get_value(i, j).unwrap());
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
	fn test_new_matrix() {
		let mat = Matrix::new(2, 3, vec![0.1, 0.3, 5., 6., 0., 0.]).unwrap();
		assert_eq!(mat.rows, 2);
		assert_eq!(mat.cols, 3);
		assert_eq!(mat.data, vec![0.1, 0.3, 5., 6., 0., 0.]);
	}

	#[test]
	fn test_set_value() {
		let mut mat = Matrix::new(2, 3, vec![0.1, 0.3, 5., 6., 0., 0.]).unwrap();
		mat.set_value(2, 0, 100.).unwrap();
		mat.set_value(1, 1, 10.).unwrap();
		assert_eq!(mat.data[3], 10.);
	}

	#[test]
	fn test_new_matrix_error() {
		let mat = Matrix::new(2, 3, vec![0.1, 0.3, 5., 6., 0.]).unwrap_err();
		assert_eq!(
			mat.to_string(),
			"FailedToInitialize error: Size of data != rows * cols: 5 != 6"
		);
	}

	#[test]
	fn test_multiplied_by_matrix() {
		let mat1 = Matrix::new(3, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, -1.0]).unwrap();
		let mat2 = Matrix::new(3, 2, vec![2.0, 1.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
		let res = mat1.multiplied_by_matrix(&mat2).unwrap();
		assert_eq!(res.data, vec![4.0, 0.0, 3.0, 4.0, 0.0, 1.0]);
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
	fn test_get_column() {
		let mat1 = Matrix::new(2, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0]).unwrap();
		/*
		1 1 0
		0 2 1
		*/
		let mat2 = mat1.get_column(0);
		assert_eq!(mat2.data, vec![1.0, 0.0]);
		let mat2 = mat1.get_column(1);
		assert_eq!(mat2.data, vec![1.0, 2.0]);
		let mat2 = mat1.get_column(2);
		assert_eq!(mat2.data, vec![0.0, 1.0]);
	}

	#[test]
	fn test_get_row() {
		let mat1 = Matrix::new(2, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0]).unwrap();
		/*
		1 1 0
		0 2 1
		*/
		let mat2 = mat1.get_row(0);
		assert_eq!(mat2.data, vec![1.0, 1.0, 0.0]);
		let mat2 = mat1.get_row(1);
		assert_eq!(mat2.data, vec![0.0, 2.0, 1.0]);
	}

	#[test]
	fn test_set_column() {
		let mut mat1 = Matrix::new(2, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0]).unwrap();
		/*
		1 1 0
		0 2 1
		*/
		mat1.set_column(0, vec![0.1, 0.2]).unwrap();
		assert_eq!(mat1.data, vec![0.1, 0.2, 1.0, 2.0, 0.0, 1.0]);
		mat1.set_column(1, vec![1.1, 1.2]).unwrap();
		assert_eq!(mat1.data, vec![0.1, 0.2, 1.1, 1.2, 0.0, 1.0]);
		mat1.set_column(2, vec![2.1, 2.2]).unwrap();
		assert_eq!(mat1.data, vec![0.1, 0.2, 1.1, 1.2, 2.1, 2.2]);
	}
	#[test]
	fn test_set_row() {
		let mut mat1 = Matrix::new(2, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0]).unwrap();
		/*
		1 1 0
		0 2 1
		*/
		mat1.set_row(0, vec![0.1, 0.2, 0.3]).unwrap();
		assert_eq!(mat1.data, vec![0.1, 0.0, 0.2, 2.0, 0.3, 1.0]);
		mat1.set_row(1, vec![1.1, 1.2, 1.3]).unwrap();
		assert_eq!(mat1.data, vec![0.1, 1.1, 0.2, 1.2, 0.3, 1.3]);
	}
}
