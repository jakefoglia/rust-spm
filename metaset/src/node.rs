// standard rust
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
// crate
use crate::metaset::{Item, MetaSet};

mod logical_and;
pub mod types
{
    pub use super::logical_and::LogicalAnd;
}

pub trait Node<ItemType, Error>
where ItemType: Item
{
    fn get_items(&mut self, nodes: &[Rc<RefCell<dyn Node<ItemType, Error>>>]) -> Result<Rc<MetaSet<ItemType>>, Error>;
}

pub type NodeSlice<'a, ItemType, Error> = &'a[Rc<RefCell<dyn Node<ItemType, Error>>>];
pub fn get_items<ItemType, Error>(nodes: NodeSlice<ItemType, Error>, id: usize) -> Result<Rc<MetaSet<ItemType>>, Error>
where ItemType: Item
{
    let node = Rc::clone(&nodes[id]);
    let mut node_ref: RefMut<dyn Node<ItemType, Error>> = (*node).borrow_mut();
    node_ref.get_items(nodes)
}

