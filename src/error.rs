#[derive(Debug)]
pub enum MathMatrixErrorKind {
	FailedToInitialize,
	OutOfBoundary,
	SizeMismatch,
	FailedToDecompose,
	OperationNotPermitted,
}

#[derive(Debug)]
pub struct MathMatrixError {
	kind: MathMatrixErrorKind,
	message: String,
}

impl MathMatrixError {
	pub fn new(kind: MathMatrixErrorKind, message: String) -> Self {
		Self { kind, message }
	}

	pub fn get_kind(&self) -> String {
		return format!("{:?}", self.kind);
	}

	pub fn get_message(&self) -> String {
		return self.message.clone();
	}
}

impl ToString for MathMatrixError {
	fn to_string(&self) -> String {
		return format!("{:?} error: {}", self.kind, self.message);
	}
}
