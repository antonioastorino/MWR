use super::error::MathMatrixError;

#[derive(Debug)]
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
			Err(MathMatrixError::FailedToInitialize(format!(
				"Size of data != rows * cols: {} != {}",
				data.len(),
				rows * cols
			)))
		}
	}

	pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
		if row < self.rows && col < self.cols {
			self.data[col * self.rows + row] = value;
		}
	}

	pub fn get_value(&self, row: usize, col: usize) -> f64 {
		if row < self.rows && col < self.cols {
			return self.data[col * self.rows + row];
		} else {
			return -1.0;
		}
	}

	pub fn multiply_by(&self, other: &Matrix) -> Self {
		let rows = self.rows;
		let cols = other.cols;
		let mut out_mat = Matrix::new(rows, cols, vec![0f64; rows * cols]).unwrap();
		for i in 0..self.rows {
			for j in 0..other.cols {
				let mut sum: f64 = 0.;
				for k in 0..self.cols {
					sum += self.get_value(i, k) * other.get_value(k, j);
				}
				out_mat.set_value(i, j, sum);
			}
		}
		return out_mat;
	}

	pub fn print(&self) {
		for i in 0..self.rows {
			for j in 0..self.cols {
				print!("{}\t", self.get_value(i, j));
			}
			print!("\n");
		}
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
		mat.set_value(2, 0, 100.);
		mat.set_value(1, 1, 10.);
		assert_eq!(mat.data[3], 10.);
	}

	#[test]
	fn test_new_matrix_error() {
		let mat = Matrix::new(2, 3, vec![0.1, 0.3, 5., 6., 0.]).unwrap_err();
		assert_eq!(
			mat.to_string(),
			"Error: Size of data != rows * cols: 5 != 6"
		);
	}

	#[test]
	fn test_multiply_by() {
		let mat1 = Matrix::new(3, 3, vec![1.0, 0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, -1.0]).unwrap();
		let mat2 = Matrix::new(3, 2, vec![2.0, 1.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
		let res = mat1.multiply_by(&mat2);
		println!("mat1:");
		mat1.print();
		println!("mat2:");
		mat2.print();
		println!("product:");
		res.print();
		assert_eq!(res.data, vec![4.0, 0.0, 3.0, 4.0, 0.0, 2.0]);
	}
}
