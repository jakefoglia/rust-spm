// standard rust
use std::{borrow::BorrowMut, cell::RefCell, cell::RefMut};
use std::collections::HashSet;
use std::rc::Rc;
// crate
use crate::{Node, NodeSlice, MetaSet, MetaItem};

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

pub struct Node_LogicalAnd<Item>
where Item: MetaItem
{
    dep_ids: Vec<usize>,
    items: Option<Rc<MetaSet<Item>>>
}

impl<Item, Error> Node<Item, Error> for Node_LogicalAnd<Item>
where Item: MetaItem
{
    fn get_items(&mut self, nodes: NodeSlice<Item, Error>) -> Result<Rc<MetaSet<Item>>, Error>
    {
        if self.items.is_none()
        {
            // self.items = Some(MetaSet::Include{include: HashSet::default()})
/*
            first we work on the type 1 and 2 sets to generate (include, exclude)

            1. if type 1 count != 0, take include = the intersection of all include sets from our type type 1 streams, else include = null
            2. if type 2 count != 0, take exclude = the union of all exclude sets from our type 2 streams, else exclude = null
            3. if include != null and exclude != null, then take include = difference(include, exclude) ... basically remove the intersection of sets from steps 1 & 2 from the set in step 1
            4. if include != null, set exclude = null
*/
            // First work on incoming Include and Exclude sets to generate (include, exclude)
            let (include, exclude) : (Option<HashSet<Rc<Item>>>, Option<HashSet<Rc<Item>>>) = (None, None);

            for dep_id in self.dep_ids.iter()
            {
                let dep_items = get_items(nodes, *dep_id);
            }

        }

        Ok(self.items.clone().unwrap())
    }
}
