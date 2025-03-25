// crate
use crate::metaset::{Item, SimpleItemSet};

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
