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

impl<ItemType> ProcessNode<ItemType>
where ItemType: Item
{
    pub fn resolve(
        &self,
        get_descendents: &dyn Fn(&Vec<usize>) -> Box<dyn Iterator<Item = ProcessNode<ItemType>>>
    ) -> ProcessingResult<ItemType> {
        let descendents: Box<dyn Iterator<Item = ProcessNode<ItemType>>> = get_descendents(&self.dep_ids);

        let inputs: Vec<ProcessingResult<ItemType>> =
            descendents.map(|d| d.resolve(get_descendents)).collect();

        self.processor.compute_items(&inputs)
    }
}
