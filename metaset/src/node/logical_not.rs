// standard rust
use std::rc::Rc;
// crate
use crate::{Item, MetaSet, Node, NodeSlice, NodeResolveError};
use super::get_items;

pub struct LogicalNot<ItemType>
where ItemType: Item
{
    dep_id: Option<usize>,
    parent_ids: Vec<usize>,
    items: Option<Rc<MetaSet<ItemType>>>
}

impl<ItemType> Node<ItemType> for LogicalNot<ItemType>
where ItemType: Item
{
    fn get_dependency_ids(&self) -> &[usize] {
        match self.dep_id.as_ref()
        {
            Some(ref id) => std::slice::from_ref(id),
            None => &[]
        }
    }

    fn get_parent_ids(&self) -> &[usize] {
        return &self.parent_ids;
    }

    fn clear_cached_items(&mut self) {
        self.items = None;
    }

    fn get_items(&mut self, nodes: NodeSlice<ItemType>) -> Result<Rc<MetaSet<ItemType>>, NodeResolveError>
    {
        if self.items.is_none()
        {
            if self.dep_id.is_none()
            {
                return Err(NodeResolveError::MissingDependencies);
            }

            // cache the final result
            let dep_set: Rc<MetaSet<ItemType>> = get_items(nodes, self.dep_id.unwrap())?;
            match dep_set.as_ref() {
                // swap the include and exclude set
                &MetaSet::Include {ref set} => {
                    self.items = Some(Rc::new(MetaSet::Exclude{set: set.clone()}));
                }
                &MetaSet::Exclude {ref set} => {
                    self.items = Some(Rc::new(MetaSet::Include{set: set.clone()}));
                }
            }
        }

        return Ok(self.items.clone().unwrap());
    }
}
