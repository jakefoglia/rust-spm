// standard rust
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;


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
    Include {set: SimpleItemSet<ItemType>},
    Exclude {set: SimpleItemSet<ItemType>},
}

impl<ItemType> Clone for MetaSet<ItemType>
where ItemType: Item
{
    fn clone(&self) -> Self
    {
        match self {
            Self::Include {set} => Self::Include { set: set.clone() },
            Self::Exclude {set} => Self::Exclude { set: set.clone() },
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
        }
    }

    pub fn get_include_set(&self) -> &SimpleItemSet<ItemType>
    {
        match self {
            MetaSet::Include {ref set} => set,
            MetaSet::Exclude {set: _} => panic!("MetaSet::Exclude does not support get_include_set()"),
        }
    }

    pub fn get_exclude_set(&self) -> &SimpleItemSet<ItemType>
    {
        match self {
            MetaSet::Include {set: _} => panic!("MetaSet::Include does not supoport get_exclude_set()"),
            MetaSet::Exclude {ref set} => set,
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
