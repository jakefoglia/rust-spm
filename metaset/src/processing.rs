// standard rust
use std::iter::Iterator;
use std::rc::Rc;

// crate
use crate::metaset::{Item, MetaSet, SimpleItemSet};

mod logical_not;
mod logical_or;
mod logical_and;
mod error;

pub use logical_and::LogicalAnd;
pub use logical_or::LogicalOr;
pub use logical_not::LogicalNot;

pub use error::ProcessingError;
pub type ProcessingResult<ItemType> = Result<Rc<MetaSet<ItemType>>, ProcessingError>;



pub trait Processor<ItemType>
where ItemType: Item
{
    fn compute_items(&mut self, inputs: impl Iterator<Item = ProcessingResult<ItemType>>) -> ProcessingResult<ItemType>;
}

fn simple_intersection<ItemType>(set1: &SimpleItemSet<ItemType>, set2: &SimpleItemSet<ItemType>) -> SimpleItemSet<ItemType>
where ItemType: Item
{
    let mut result: SimpleItemSet<ItemType> = SimpleItemSet::default();

    for item in set1
    {
        assert!(!result.contains(item));
        if set2.contains(item)
        {
            result.insert(item.clone());
        }
    }

    result
}

fn simple_union<ItemType>(set1: &SimpleItemSet<ItemType>, set2: &SimpleItemSet<ItemType>) -> SimpleItemSet<ItemType>
where ItemType: Item
{
    let mut result: SimpleItemSet<ItemType> = (*set1).clone();

    for item in set2
    {
        result.insert(item.clone());
    }

    result
}

fn simple_difference<ItemType>(set1: &SimpleItemSet<ItemType>, set2: &SimpleItemSet<ItemType>) -> SimpleItemSet<ItemType>
where ItemType: Item
{
    let mut result: SimpleItemSet<ItemType> = SimpleItemSet::default();

    for item in set1
    {
        assert!(!result.contains(item));
        if !set2.contains(item)
        {
            result.insert(item.clone());
        }
    }

    result
}


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
