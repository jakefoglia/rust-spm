use crate::metaset::Item;
use crate::processing::ProcessNode;

use super::{ProcessingError, ProcessingErrorType, ProcessingResult};

pub struct ProcessChain<ItemType: Item>
{
    pub nodes: Vec<ProcessNode<ItemType>>, // index: id
    pub root_id: usize
}

impl<ItemType> ProcessChain<ItemType>
where ItemType: Item
{
    pub fn resolve(&self) -> ProcessingResult<ItemType>
    {
        if let Some(root) = self.nodes.get(self.root_id)
        {
            root.resolve(self.root_id, &self.nodes)
        }
        else
        {
            Err(ProcessingError{node_id: None,
                                error_type: ProcessingErrorType::MissingInputs})
        }
    }
}
