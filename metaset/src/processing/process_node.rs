use crate::metaset::Item;
use crate::processing::ProcessingResult;
use crate::processing::processor::Processor;

pub struct ProcessNode<ItemType: Item>
{
    pub id: usize,
    pub dep_ids: Vec<usize>,
    pub items: Option<ProcessingResult<ItemType>>,
    pub processor: Box<dyn Processor<ItemType>>
}
