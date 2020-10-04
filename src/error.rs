#[derive(Debug)]
pub enum MathMatrixError {
	FailedToInitialize(String),
}

impl ToString for MathMatrixError {
	fn to_string(&self) -> String {
		match self {
			MathMatrixError::FailedToInitialize(string) => {
				return format!("Error: {}", string);
			}
		}
	}
}
