use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProcessingError
{
    InvalidConfig,
    ExternalFailure,
    TooManyInputs,
    MissingInputs,
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self
        {
            Self::InvalidConfig => write!(f, "InvalidConfig"),
            Self::ExternalFailure => write!(f, "ExternalFailure"),
            Self::TooManyInputs => write!(f, "TooManyInputs"),
            Self::MissingInputs => write!(f, "MissingInputs"),
        }
    }
}

impl std::error::Error for ProcessingError {}
