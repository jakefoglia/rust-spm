// standard rust
use std::rc::Rc;
// crate
use crate::metaset::{MetaSet, Item, SimpleItemSet};
use crate::processing::{ProcessingError, ProcessingErrorType, ProcessingResult};
use crate::processing::processor::Processor;
use crate::processing::util::{simple_union, simple_intersection, simple_difference};

pub struct LogicalOr ();

impl<ItemType> Processor<ItemType> for LogicalOr
where ItemType: Item
{
    fn compute_items(&self, id: usize, inputs: &[ProcessingResult<ItemType>]) -> ProcessingResult<ItemType>
    {
        let mut include_set : Option<SimpleItemSet<ItemType>> = None;
        let mut exclude_set : Option<SimpleItemSet<ItemType>> = None;

        for input in inputs
        {
            let input: Rc<MetaSet<ItemType>> = input.clone()?;

            match input.as_ref() {
                &MetaSet::Include {ref set} => {
                    if include_set.is_none() {
                        include_set = Some(SimpleItemSet::default())
                    };

                    include_set = Some(
                        simple_union(include_set.as_ref().unwrap(), set)
                    );
                }
                &MetaSet::Exclude {ref set} => {
                    if exclude_set.is_none() {
                        exclude_set = Some(SimpleItemSet::default())
                    };

                    exclude_set = Some(
                        simple_intersection(exclude_set.as_ref().unwrap(), set)
                    );
                }
            }
        }

        if include_set.is_none() && exclude_set.is_none()
        {
            return Err(ProcessingError{node_id: Some(id),
                                       error_type: ProcessingErrorType::MissingInputs});
        }

        if include_set.is_some()
        {
            if exclude_set.is_some()
            {
                return Ok(Rc::new(
                    MetaSet::Include { set: simple_difference(include_set.as_ref().unwrap(), exclude_set.as_ref().unwrap()) }
                ));
            }
            else
            {
                return Ok(Rc::new(
                    MetaSet::Include { set: include_set.unwrap() }
                ));
            }
        }
        else
        {
             return Ok(Rc::new(
                MetaSet::Exclude { set: exclude_set.unwrap() }
             ));
        }

    }
}
