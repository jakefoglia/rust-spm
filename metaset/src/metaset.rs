// standard rust
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;


/*
A metaset is represents this:

Include U (P - Exclude)

Therefore any MetaSet with an Exclude set is infinite
*/

pub trait Item: PartialEq + Eq + Hash {}

pub struct RcItem<ItemType: Item> (Rc<ItemType>);

impl<ItemType> Clone for RcItem<ItemType>
where ItemType: Item
{
    fn clone(&self) -> RcItem<ItemType> { RcItem (Rc::clone(&self.0)) }
}

impl<ItemType> Hash for RcItem<ItemType>
where ItemType: Item
{
    fn hash<H>(&self, hasher: &mut H) where H: Hasher {
        hasher.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}

impl<ItemType> PartialEq for RcItem<ItemType>
where ItemType: Item
{
    fn eq(&self, other: &RcItem<ItemType>) -> bool {
        *self.0 == *other.0
    }
}
impl<ItemType> Eq for RcItem<ItemType>
where ItemType: Item
{}

pub type SimpleItemSet<ItemType> = HashSet<RcItem<ItemType>>;

pub enum MetaSet<ItemType>
where ItemType: Item
{
    Include {include_set: SimpleItemSet<ItemType>},
    Exclude {exclude_set: SimpleItemSet<ItemType>},
    IncludeExclude {include_set: SimpleItemSet<ItemType>, exclude_set: SimpleItemSet<ItemType>},
}

impl<ItemType> Clone for MetaSet<ItemType>
where ItemType: Item
{
    fn clone(&self) -> Self
    {
        match self {
            Self::Include {include_set} => Self::Include { include_set: include_set.clone() },
            Self::Exclude {exclude_set} => Self::Exclude { exclude_set: exclude_set.clone() },
            Self::IncludeExclude {include_set, exclude_set} => Self::IncludeExclude { include_set: include_set.clone(), exclude_set: exclude_set.clone() },
        }
    }
}

impl<ItemType> MetaSet<ItemType>
where ItemType: Item
{
    pub fn is_finite(&self) -> bool
    {
        match self {
            MetaSet::Include {..} => true,
            MetaSet::Exclude {..} => false,
            MetaSet::IncludeExclude {..} => false,
        }
    }

    pub fn get_set(&self) -> &SimpleItemSet<ItemType>
    {
        if let MetaSet::Include{ref include_set} = self
        {
            return include_set;
        }
        else
        {
            panic!("MetaSet instance is not finite!");
        }
    }

    pub fn has_include_set(&self) -> bool
    {
        match self {
            MetaSet::Include {..} => true,
            MetaSet::Exclude {..} => false,
            MetaSet::IncludeExclude {..} => true,
        }
    }

    pub fn get_include_set(&self) -> &SimpleItemSet<ItemType>
    {
        match self {
            MetaSet::Include {ref include_set} => include_set,
            MetaSet::Exclude {exclude_set: _} => panic!("MetaSet::Exclude has no member include_set"),
            MetaSet::IncludeExclude {ref include_set, exclude_set: _} => include_set,

        }
    }

    pub fn has_exclude_set(&self) -> bool
    {
        match self {
            MetaSet::Include {..} => false,
            MetaSet::Exclude {..} => true,
            MetaSet::IncludeExclude {..} => true,
        }
    }

    pub fn get_exclude_set(&self) -> &SimpleItemSet<ItemType>
    {
        match self {
            MetaSet::Include {include_set: _} => panic!("MetaSet::Include has no member exclude_set"),
            MetaSet::Exclude {ref exclude_set} => exclude_set,
            MetaSet::IncludeExclude {include_set: _, ref exclude_set} => exclude_set,
        }
    }
}

pub fn simple_intersection<ItemType>(set1: &SimpleItemSet<ItemType>, set2: &SimpleItemSet<ItemType>) -> SimpleItemSet<ItemType>
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

pub fn simple_union<ItemType>(set1: &SimpleItemSet<ItemType>, set2: &SimpleItemSet<ItemType>) -> SimpleItemSet<ItemType>
where ItemType: Item
{
    let mut result: SimpleItemSet<ItemType> = (*set1).clone();

    for item in set2
    {
        result.insert(item.clone());
    }

    result
}

pub fn simple_difference<ItemType>(set1: &SimpleItemSet<ItemType>, set2: &SimpleItemSet<ItemType>) -> SimpleItemSet<ItemType>
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
