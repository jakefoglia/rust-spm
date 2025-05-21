use crate::metaset::Item;
use crate::processing::ProcessNode;

use super::{ProcessingError, ProcessingResult};

pub struct ProcessChain<ItemType: Item>
{
    pub nodes: Vec<ProcessNode<ItemType>>, // index: id
}

impl<ItemType> ProcessChain<ItemType>
where ItemType: Item
{
    pub fn resolve(&self) -> ProcessingResult<ItemType>
    {
        if let Some(root) = self.nodes.get(0)
        {
            // TODO
            Err(ProcessingError::MissingInputs)
            // root.resolve()
        }
        else
        {
            Err(ProcessingError::MissingInputs)
        }
    }
}
