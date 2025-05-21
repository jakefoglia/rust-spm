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
    fn compute_items(&mut self, mut inputs: Box<dyn Iterator<Item = ProcessingResult<ItemType>>>) -> ProcessingResult<ItemType> {
        if self.filter_criteria.is_none()
        {
            return Err(ProcessingError::InvalidConfig);
        }

        let input = inputs.next();
        if input.is_none()
        {
            return Err(ProcessingError::MissingInputs);
        }
        let input = input.unwrap();

        let next_input = inputs.next();
        if next_input.is_some()
        {
            return Err(ProcessingError::TooManyInputs);
        }

        if let Err(err) = input
        {
            return Err(err);
        }

        match input.unwrap().as_ref() {
             MetaSet::Include {ref set} => self.filter_criteria.as_ref().unwrap()(set),
            _ => Err(ProcessingError::InvalidInputs)
        }
    }
}

