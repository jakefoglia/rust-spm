// standard rust
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::rc::Rc;
// crate
use crate::metaset::{Item, MetaSet};

mod logical_not;
mod logical_or;
mod logical_and;
mod util;

// re-exports
pub use logical_and::LogicalAnd;
pub use logical_or::LogicalOr;
pub use logical_not::LogicalNot;

pub trait Processor<ItemType>
where ItemType: Item
{
    fn compute_items(&mut self, inputs: Box<dyn Iterator<Item = ProcessingResult<ItemType>>>) -> ProcessingResult<ItemType>;
}

#[derive(Debug)]
pub enum ProcessingError
{
    InvalidConfig,
    ExternalFailure,
    TooManyInputs,
    MissingInputs,
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self
        {
            Self::InvalidConfig => write!(f, "InvalidConfig"),
            Self::ExternalFailure => write!(f, "ExternalFailure"),
            Self::TooManyInputs => write!(f, "TooManyInputs"),
            Self::MissingInputs => write!(f, "MissingInputs"),
        }
    }
}

impl std::error::Error for ProcessingError {}
pub type ProcessingResult<ItemType> = Result<Rc<MetaSet<ItemType>>, ProcessingError>;

/*
pub fn get_items<ItemType>(nodes: NodeSlice<ItemType>, id: usize) -> Result<Rc<MetaSet<ItemType>>, NodeResolveError>
where ItemType: Item
{
    let node = Rc::clone(&nodes[id]);
    let mut node_ref: RefMut<dyn Node<ItemType>> = (*node).borrow_mut();
    node_ref.get_items(nodes)
}

pub fn clear_cache_recursive<ItemType>(nodes: NodeSlice<ItemType>, id: usize)
where ItemType: Item
{
    let mut ids: Vec<usize> = vec![id];

    while !ids.is_empty()
    {
        let id = ids.pop().unwrap();

        let node = Rc::clone(&nodes[id]);
        let mut node_ref: RefMut<dyn Node<ItemType>> = (*node).borrow_mut();

        node_ref.clear_cached_items();
        ids.extend_from_slice(node_ref.get_parent_ids());
    }
}
*/
