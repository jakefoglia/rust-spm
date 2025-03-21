// standard rust
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
// crate
use crate::metaset::{MetaItem, MetaSet};

// exports
pub mod node_types;

pub trait Node<Item, Error>
where Item: MetaItem
{
    fn idk(&mut self) -> bool {false}
    fn get_items(&mut self, nodes: &[Rc<RefCell<dyn Node<Item, Error>>>]) -> Result<Rc<MetaSet<Item>>, Error>;
}

pub type NodeSlice<'a, Item, Error> = &'a[Rc<RefCell<dyn Node<Item, Error>>>];
pub fn get_items<Item, Error>(nodes: NodeSlice<Item, Error>, id: usize) -> Result<Rc<MetaSet<Item>>, Error>
where Item: MetaItem
{
    let node = Rc::clone(&nodes[id]);
    let mut node_ref: RefMut<dyn Node<Item, Error>> = (*node).borrow_mut();
    node_ref.get_items(nodes)
}
