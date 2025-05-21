use crate::metaset::Item;
use crate::processing::ProcessingResult;
use crate::processing::processor::Processor;

use super::{ProcessingError, ProcessingErrorType};

pub struct ProcessNode<ItemType: Item>
{
    pub id: usize,
    pub dep_ids: Vec<usize>,
    pub items: Option<ProcessingResult<ItemType>>,
    pub processor: Box<dyn Processor<ItemType>>
}

impl<ItemType> ProcessNode<ItemType>
where ItemType: Item
{
    pub fn resolve(
        &self,
        id: usize,
        nodes: &[ProcessNode<ItemType>]
    ) -> ProcessingResult<ItemType> {
        let inputs: Vec<ProcessingResult<ItemType>> = self.dep_ids.iter().map(|dep_id: &usize| {
            match nodes.get(*dep_id) {
                Some(ref node) => {node.resolve(*dep_id, nodes)},
                None => Err(ProcessingError{node_id: Some(id),
                                            error_type: ProcessingErrorType::InvalidConfig})
            }
        }).collect();

        self.processor.compute_items(id, &inputs)
    }
}
