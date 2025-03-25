// standard rust
use std::rc::Rc;
// crate
use crate::metaset::{MetaSet, Item};
use crate::processing::{Processor, ProcessingError, ProcessingResult};

pub struct LogicalNot ();

impl<ItemType> Processor<ItemType> for LogicalNot
where ItemType: Item
{
    fn compute_items(&mut self, mut inputs: impl Iterator<Item = ProcessingResult<ItemType>>) -> ProcessingResult<ItemType>
    {
        let input = inputs.next();
        if input.is_none()
        {
            return Err(ProcessingError::MissingInputs);
        }

        let next_input = inputs.next();
        if next_input.is_some()
        {
            return Err(ProcessingError::TooManyInputs);
        }

        match input.unwrap()?.as_ref() {
            // swap the include and exclude set
            &MetaSet::Include {ref set} => {
                Ok(Rc::new(MetaSet::Exclude { set: set.clone() }))
            }
            &MetaSet::Exclude {ref set} => {
                Ok(Rc::new(MetaSet::Include { set: set.clone() }))
            }
        }
    }
}
