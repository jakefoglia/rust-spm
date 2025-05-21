// standard rust
use std::rc::Rc;
// crate
use crate::metaset::{MetaSet, Item};
use crate::processing::{ProcessingError, ProcessingErrorType, ProcessingResult};
use crate::processing::processor::Processor;

pub struct LogicalNot ();

impl<ItemType> Processor<ItemType> for LogicalNot
where ItemType: Item
{
    fn compute_items(&self, id: usize, inputs: &[ProcessingResult<ItemType>]) -> ProcessingResult<ItemType>
    {
        if inputs.len() == 0
        {
            return Err(ProcessingError{node_id: Some(id),
                                        error_type: ProcessingErrorType::MissingInputs});
        }
        else if inputs.len() > 1
        {
            return Err(ProcessingError{node_id: Some(id),
                                        error_type: ProcessingErrorType::TooManyInputs});

        }

        let input: Rc<MetaSet<ItemType>> = inputs[0].clone()?;
        match input.as_ref() {
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
