mod node;
mod metaset;
mod processing;

pub use metaset::{MetaSet, Item};
pub use processing::{ProcessingError, ProcessingResult, Processor, LogicalAnd, LogicalOr, LogicalNot};
pub use node::ProcessNode;
