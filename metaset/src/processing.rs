// standard rust
use std::fmt::{Display, Formatter};
use std::rc::Rc;
// crate
use crate::metaset::MetaSet;

mod util;
mod process_chain;
mod process_node;
pub mod processor;

pub use process_chain::ProcessChain;
pub use process_node::ProcessNode;

// TODO add error for
// - node cycle
//    - path: mut& Set<id>

#[derive(Debug, Clone, Copy)]
pub enum ProcessingErrorType
{
    InvalidConfig,
    ExternalFailure,
    TooManyInputs,
    MissingInputs,
    InvalidInputs,
    InvalidInputId
}

impl Display for ProcessingErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self
        {
            Self::InvalidConfig => write!(f, "InvalidConfig"),
            Self::ExternalFailure => write!(f, "ExternalFailure"),
            Self::TooManyInputs => write!(f, "TooManyInputs"),
            Self::MissingInputs => write!(f, "MissingInputs"),
            Self::InvalidInputs => write!(f, "InvalidInputs"),
            Self::InvalidInputId => write!(f, "InvalidInputId"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessingError {
    pub error_type: ProcessingErrorType,
    pub node_id: Option<usize>
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.node_id {
            Some(id) => write!(f, "node_id: {}, error_type: {}", id, self.error_type),
            None => write!(f, "node_id: null, error_type: {}", self.error_type)
        }
    }
}

pub type ProcessingResult<ItemType> = Result<Rc<MetaSet<ItemType>>, ProcessingError>;
