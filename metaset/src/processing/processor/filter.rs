use std::rc::Rc;
// crate
use crate::metaset::{MetaSet, Item, SimpleItemSet};
use crate::processing::{ProcessingError, ProcessingResult};
use crate::processing::processor::Processor;


type FilterFn<ItemType> = Box<dyn Fn(&SimpleItemSet<ItemType>) -> ProcessingResult<ItemType>>;

pub struct Filter<ItemType: Item>
{
    pub filter_criteria: Option<FilterFn<ItemType>>
}

impl<ItemType> Processor<ItemType> for Filter<ItemType>
where ItemType: Item
{
    fn compute_items(&self, inputs: &[ProcessingResult<ItemType>]) -> ProcessingResult<ItemType> {
        if self.filter_criteria.is_none()
        {
            return Err(ProcessingError::InvalidConfig);
        }

        if inputs.len() == 0
        {
            return Err(ProcessingError::MissingInputs);
        }
        else if inputs.len() > 1
        {
            return Err(ProcessingError::TooManyInputs);
        }

        let input: Rc<MetaSet<ItemType>> = inputs[0].clone()?;

        match input.as_ref() {
             MetaSet::Include {ref set} => self.filter_criteria.as_ref().unwrap()(set),
            _ => Err(ProcessingError::InvalidInputs)
        }
    }
}

