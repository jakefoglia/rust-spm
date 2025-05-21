use crate::metaset::Item;
use crate::processing::ProcessingResult;

mod logical_not;
mod logical_or;
mod logical_and;
mod source;
mod filter;

// re-exports
pub use logical_and::LogicalAnd;
pub use logical_or::LogicalOr;
pub use logical_not::LogicalNot;
pub use filter::Filter;
pub use source::Source;

pub trait Processor<ItemType>
where ItemType: Item
{
    fn compute_items(&self, inputs: &[ProcessingResult<ItemType>]) -> ProcessingResult<ItemType>;
}
