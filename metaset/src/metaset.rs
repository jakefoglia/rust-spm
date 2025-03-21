// standard rust
use std::collections::HashSet;
use std::rc::Rc;

pub trait MetaItem : PartialEq {}

/*
A metaset is represents this:

Include U (P - Exclude)

Therefore any MetaSet with an Exclude set is infinite

*/

pub enum MetaSet<Item>
where Item: MetaItem
{
    Include {include: HashSet<Rc<Item>>},
    Exclude {exclude: HashSet<Rc<Item>>},
    IncludeExclude {include: HashSet<Rc<Item>>, exclude: HashSet<Rc<Item>>},
}

impl<Item> Clone for MetaSet<Item>
where Item: MetaItem
{
    fn clone(&self) -> Self
    {
        match self {
            Self::Include {include} => Self::Include { include: include.clone() },
            Self::Exclude {exclude} => Self::Exclude { exclude: exclude.clone() },
            Self::IncludeExclude {include, exclude} => Self::IncludeExclude { include: include.clone(), exclude: exclude.clone() },
        }
    }
}

impl<Item> MetaSet<Item>
where Item: MetaItem
{
    fn is_finite(&self) -> bool
    {
        match self {
            Include => true,
            IncludeExclude => true,
            Exclude => false
        }
    }
}
