#[derive(Debug)]
pub enum OperationError {
    OperationNotPermitted,
    ConstraintViolation(String),
    ObjectNotFound,
    ObjectAlreadyExists,
    InvalidOperation,
}