// standard rust
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
// crate
use crate::metaset::{Item, MetaSet};

mod error;
mod logical_and;
mod logical_or;
mod logical_not;
pub mod types
{
    pub use super::logical_and::LogicalAnd;
    pub use super::logical_or::LogicalOr;
    pub use super::logical_not::LogicalNot;
}

pub use error::NodeResolveError;

pub trait Node<ItemType>
where ItemType: Item
{
    fn get_items(&mut self, nodes: &[Rc<RefCell<dyn Node<ItemType>>>]) -> Result<Rc<MetaSet<ItemType>>, NodeResolveError>;
}

pub type NodeSlice<'a, ItemType> = &'a[Rc<RefCell<dyn Node<ItemType>>>];
pub fn get_items<ItemType>(nodes: NodeSlice<ItemType>, id: usize) -> Result<Rc<MetaSet<ItemType>>, NodeResolveError>
where ItemType: Item
{
    let node = Rc::clone(&nodes[id]);
    let mut node_ref: RefMut<dyn Node<ItemType>> = (*node).borrow_mut();
    node_ref.get_items(nodes)
}
