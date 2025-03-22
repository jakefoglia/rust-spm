// standard rust
use std::rc::Rc;

// crate
use crate::{simple_difference, simple_intersection, simple_union, Item, MetaSet, Node, NodeSlice, SimpleItemSet};

use super::get_items;

          /*
          when ANDing 2+ streams, we do the following:
            0. split all incoming streams into separate lists by type (type 1, 2, 3)

            first we work on the type 1 and 2 sets to generate (include, exclude)

            1. if type 1 count != 0, take include = the intersection of all include sets from our type type 1 streams, else include = null
            2. if type 2 count != 0, take exclude = the union of all exclude sets from our type 2 streams, else exclude = null
            3. if include != null and exclude != null, then take include = difference(include, exclude) ... basically remove the intersection of sets from steps 1 & 2 from the set in step 1
            4. if include != null, set exclude = null

            at this point (include, exclude) is either type 0 or 1 or 2 (even though its not technically a stream)
            next we work on the type 3 sets to generate (include3, exclude3)

            5. if type 3 count == 0 skip the rest and just return TrackStream(include, exclude), else var include3 = new Set()
            6. add to include3 any track from any type3 include set if
              6a.      its not in any type3 exclude set
              6b.  or  its on every other type3 include list (generate a type3 include intersection set for quickly determining this)
              6c.  or  when its missing from a type3 include list then it doesn't appear on the exclude list for that particular same stream

            7. var exclude3  = union of all type3 exclude lists.
            8. remove from exclude3 anything that also appears on include3 so exclude3 = difference(exclude3, include3)  ? not sure about this step

            at this point (include3, exclude3) is type 2 and the result from steps 1-4 (include, exclude) is either type 1 or 2
            9. determine the type as 1 or 2
                9a. if the result from steps 1-4 is type 0 then do the following:
                    return new TrackStream(include3, exclude3);

                9b. else if the result from steps 1-4 is type 1 then do the following:
                    finalIncludeSet = (intersection of include and include3) + difference(include, exclude3)
                    return new TrackStream(finalIncludeSet, null)

                9c. else it was type 2 so do the following:
                    finalIncludeSet = include3 - exclude
                    finalExcludeSet = union(exclude, exclude3)
                    return new TrackStream(finalIncludeSet, finalExcludeSet)
          */

pub struct LogicalAnd<ItemType>
where ItemType: Item
{
    dep_ids: Vec<usize>,
    items: Option<Rc<MetaSet<ItemType>>>
}

impl<ItemType, ErrorKind> Node<ItemType, ErrorKind> for LogicalAnd<ItemType>
where ItemType: Item
{
    fn get_items(&mut self, nodes: NodeSlice<ItemType, ErrorKind>) -> Result<Rc<MetaSet<ItemType>>, ErrorKind>
    {
        if self.items.is_none()
        {
            // helper for returning final result (TODO move this elsewhere for other node types to use??)
            let make_result = |include_set: Option<SimpleItemSet<ItemType>>, exclude_set: Option<SimpleItemSet<ItemType>>| {
                if include_set.is_some()
                {
                    if exclude_set.is_some()
                    {
                        return MetaSet::IncludeExclude {include_set: include_set.unwrap(),
                                                        exclude_set: exclude_set.unwrap()};
                    }
                    else
                    {
                        return MetaSet::Include {include_set: include_set.unwrap()};
                    }
                }
                else
                {
                    return MetaSet::Exclude {exclude_set: exclude_set.unwrap()};
                }
            };

            // self.items = Some(MetaSet::Include{include: HashSet::default()})
/*
            first we work on the type 1 and 2 sets to generate (include, exclude)

            1. if type 1 count != 0, take include = the intersection of all include sets from our type type 1 streams, else include = null
            2. if type 2 count != 0, take exclude = the union of all exclude sets from our type 2 streams, else exclude = null
            3. if include != null and exclude != null, then take include = difference(include, exclude) ... basically remove the intersection of sets from steps 1 & 2 from the set in step 1
            4. if include != null, set exclude = null
*/
            // First work on incoming Include and Exclude sets to generate (include, exclude)
            let mut include_set : Option<SimpleItemSet<ItemType>> = None;
            let mut exclude_set : Option<SimpleItemSet<ItemType>> = None;

            let mut dep_item_sets: Vec<Rc<MetaSet<ItemType>>> = Vec::with_capacity(self.dep_ids.len());
            for id in &self.dep_ids
            {
                let dep_items = get_items(nodes, *id)?;
                dep_item_sets.push(dep_items);
            }

            let mut has_include_exclude_deps = false;
            dep_item_sets.iter().for_each(|dep_items| {
                match dep_items.as_ref()
                {
                    &MetaSet::Include { include_set: ref dep_include_set } => {
                        include_set = Some(
                            simple_intersection(include_set.as_ref().unwrap(), dep_include_set)
                        );
                    }
                    &MetaSet::Exclude { exclude_set: ref dep_exclude_set } => {
                        exclude_set = Some(
                            simple_union(exclude_set.as_ref().unwrap(), dep_exclude_set)
                        )
                    }
                    &MetaSet::IncludeExclude {..} => {
                        has_include_exclude_deps = true;
                    }
                }
            });

            if include_set.is_some() && exclude_set.is_some()
            {
                include_set = Some(
                    simple_difference(include_set.as_ref().unwrap(), exclude_set.as_ref().unwrap())
                );
                exclude_set = None;
            }

            // if no incoming IncludeExclude sets, then there's no more work to do
            if !has_include_exclude_deps
            {
                self.items = Some(Rc::new(make_result(include_set, exclude_set)));
                return Ok(self.items.clone().unwrap());
            }

            // else - some additional steps...
            return Ok(self.items.clone().unwrap()) // TODO

        }
        else
        {
            Ok(self.items.clone().unwrap())
        }
    }
}
