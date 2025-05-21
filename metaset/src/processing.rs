// standard rust
use std::fmt::{Display, Formatter};
use std::rc::Rc;
// crate
use crate::metaset::MetaSet;

mod util;

pub mod processor;
mod process_chain;
mod process_node;

pub use process_chain::ProcessChain;
pub use process_node::ProcessNode;


#[derive(Debug)]
pub enum ProcessingError
{
    InvalidConfig,
    ExternalFailure,
    TooManyInputs,
    MissingInputs,
    InvalidInputs
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self
        {
            Self::InvalidConfig => write!(f, "InvalidConfig"),
            Self::ExternalFailure => write!(f, "ExternalFailure"),
            Self::TooManyInputs => write!(f, "TooManyInputs"),
            Self::MissingInputs => write!(f, "MissingInputs"),
            Self::InvalidInputs => write!(f, "InvalidInputs"),
        }
    }
}

impl std::error::Error for ProcessingError {}
pub type ProcessingResult<ItemType> = Result<Rc<MetaSet<ItemType>>, ProcessingError>;
