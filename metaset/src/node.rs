use crate::metaset::Item;
use crate::processing::{Processor, ProcessingResult};

pub struct ProcessNode<ItemType: Item>
{
    id: usize,
    dep_ids: Vec<usize>,
    items: Option<ProcessingResult<ItemType>>,
    processor: Box<dyn Processor<ItemType>>
}
