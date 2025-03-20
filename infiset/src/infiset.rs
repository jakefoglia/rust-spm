// standard rust
use std::collections::HashSet;
use std::rc::Rc;

pub enum Infiset<Item>
where Item: PartialEq
{
    Include {include: HashSet<Rc<Item>>},
    Exclude {exlude: HashSet<Rc<Item>>},
    IncludeExclude {include: HashSet<Rc<Item>>, exclude: HashSet<Rc<Item>>},
}
