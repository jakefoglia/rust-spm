// standard rust
 use std::rc::Rc;
// crate
use crate::metaset::{MetaSet, Item};
use crate::processing::{ProcessingError, ProcessingResult};
use crate::processing::processor::Processor;

pub struct Source<ItemType: Item>
{
    pub items: Option<Rc<MetaSet<ItemType>>>
}

impl<ItemType> Processor<ItemType> for Source<ItemType>
where ItemType: Item
{
    fn compute_items(&mut self, mut inputs: Box<dyn Iterator<Item = ProcessingResult<ItemType>>>) -> ProcessingResult<ItemType>
    {
        if inputs.next().is_some()
        {
            return Err(ProcessingError::TooManyInputs);
        }

        if self.items.is_none()
        {
            return Err(ProcessingError::ExternalFailure);
        }

        Ok(self.items.as_ref().unwrap().clone())
    }
}
