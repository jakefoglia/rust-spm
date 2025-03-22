use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NodeResolveError
{
    ExternalDependencyFailure,
    TooManyDependencies,
    MissingDependencies,
}

impl Display for NodeResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self
        {
            Self::ExternalDependencyFailure => write!(f, "ExternalDependencyFailure"),
            Self::MissingDependencies => write!(f, "MissingDependencies"),
            Self::TooManyDependencies => write!(f, "TooManyDependencies"),
        }
    }
}

impl std::error::Error for NodeResolveError {}
