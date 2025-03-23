// standard rust
use std::rc::Rc;
// crate
use crate::{simple_difference, simple_intersection, simple_union, Item, MetaSet, Node, NodeSlice, SimpleItemSet};
use super::{get_items, NodeResolveError};

pub struct LogicalAnd<ItemType>
where ItemType: Item
{
    dep_ids: Vec<usize>,
    parent_ids: Vec<usize>,
    items: Option<Rc<MetaSet<ItemType>>>
}

impl<ItemType> Node<ItemType> for LogicalAnd<ItemType>
where ItemType: Item
{
    fn get_dependency_ids(&self) -> &[usize] {
        return &self.dep_ids;
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
            if self.dep_ids.len() == 0
            {
                return Err(NodeResolveError::MissingDependencies);
            }

            let mut include_set : Option<SimpleItemSet<ItemType>> = None;
            let mut exclude_set : Option<SimpleItemSet<ItemType>> = None;

            let mut dep_incl_sets: Vec<Rc<MetaSet<ItemType>>> = Vec::new();
            let mut dep_excl_sets: Vec<Rc<MetaSet<ItemType>>> = Vec::new();

            for id in &self.dep_ids
            {
                let dep_set: Rc<MetaSet<ItemType>> = get_items(nodes, *id)?;
                match dep_set.as_ref() {
                    &MetaSet::Include {..} => {
                        dep_incl_sets.push(Rc::clone(&dep_set));
                    }
                    &MetaSet::Exclude {..} => {
                        dep_excl_sets.push(Rc::clone(&dep_set));
                    }
                }
            }

            if dep_incl_sets.len() > 0
            {
                include_set = Some(SimpleItemSet::default());
            }

            if dep_excl_sets.len() > 0
            {
                exclude_set = Some(SimpleItemSet::default());
            }

            dep_incl_sets.iter().for_each(|set| {
                include_set = Some(
                    simple_intersection(include_set.as_ref().unwrap(), set.get_include_set())
                );
            });

            dep_excl_sets.iter().for_each(|set| {
                exclude_set = Some(
                    simple_union(exclude_set.as_ref().unwrap(), set.get_exclude_set())
                );
            });

            // cache the final result
            if include_set.is_some()
            {
                if exclude_set.is_some()
                {
                    self.items = Some(Rc::new(
                        MetaSet::Include { set: simple_difference(include_set.as_ref().unwrap(), exclude_set.as_ref().unwrap()) }
                    ));
                }
                else
                {
                    self.items = Some(Rc::new(
                        MetaSet::Include { set: include_set.unwrap() }
                    ));
                }
            }
            else
            {
                 self.items = Some(Rc::new(
                    MetaSet::Exclude { set: exclude_set.unwrap() }
                 ));
            }
        }

        return Ok(self.items.clone().unwrap());
    }
}
