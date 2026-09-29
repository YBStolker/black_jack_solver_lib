use thiserror::Error;

#[derive(Debug, Error)]
pub enum CardError {
    #[error("Cannot convert u32 to Card. Number '{0}' is out of range.")]
    OutOfRangeU32(u32),
    #[error("Cannot convert usize to Card. Number '{0}' is out of range.")]
    OutOfRangeUsize(usize),
}

impl From<u32> for CardError {
    fn from(value: u32) -> Self {
        CardError::OutOfRangeU32(value)
    }
}

impl From<usize> for CardError {
    fn from(value: usize) -> Self {
        CardError::OutOfRangeUsize(value)
    }
}
