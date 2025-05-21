// standard rust
 use std::rc::Rc;
// crate
use crate::metaset::{MetaSet, Item};
use crate::processing::{ProcessingError, ProcessingErrorType, ProcessingResult};
use crate::processing::processor::Processor;

pub struct Source<ItemType: Item>
{
    pub items: Option<Rc<MetaSet<ItemType>>>
}

impl<ItemType> Processor<ItemType> for Source<ItemType>
where ItemType: Item
{
    fn compute_items(&self, id: usize, inputs: &[ProcessingResult<ItemType>]) -> ProcessingResult<ItemType>
    {
        if inputs.len() > 0
        {
            return Err(ProcessingError{node_id: Some(id),
                                       error_type: ProcessingErrorType::TooManyInputs});

        }

        if self.items.is_none()
        {
            return Err(ProcessingError{node_id: Some(id),
                                       error_type: ProcessingErrorType::ExternalFailure});

        }

        Ok(self.items.as_ref().unwrap().clone())
    }
}
