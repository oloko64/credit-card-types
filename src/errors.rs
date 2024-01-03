#[derive(Debug, Clone)]
pub enum CardTypeError {
    ParseIntError(std::num::ParseIntError),
    TryFromIntError(std::num::TryFromIntError),
}

impl std::fmt::Display for CardTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CardTypeError::ParseIntError(err) => write!(f, "{err}"),
            CardTypeError::TryFromIntError(err) => write!(f, "{err}"),
        }
    }
}

impl From<std::num::ParseIntError> for CardTypeError {
    fn from(err: std::num::ParseIntError) -> Self {
        CardTypeError::ParseIntError(err)
    }
}

impl From<std::num::TryFromIntError> for CardTypeError {
    fn from(err: std::num::TryFromIntError) -> Self {
        CardTypeError::TryFromIntError(err)
    }
}

impl std::error::Error for CardTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CardTypeError::ParseIntError(err) => Some(err),
            CardTypeError::TryFromIntError(err) => Some(err),
        }
    }
}
