// standard rust
use std::collections::HashSet;
use std::future::Future;
use std::io::ErrorKind;
// crate
use crate::infiset::*;

pub trait Node<Item>
where Item: PartialEq
{
    fn get_items(&self) -> Result<Infiset<Item>, ErrorKind>;
    fn get_internal_dependencies(&self) -> &[usize];
    fn resolve_external_dependencies(&self) -> Option<Box<dyn Future<Output = Result<(), ErrorKind>> + Send>>
    {
        None
    }
}

pub struct LogicalAnd<Item>
where Item: PartialEq
{
    internal_deps: Vec<usize>,
    set: Option<Infiset<Item>>
}

impl<Item> Node<Item> for LogicalAnd<Item>
where Item: PartialEq
{
    fn get_internal_dependencies(&self) -> &[usize] {
        &self.internal_deps
    }

    fn get_items(&self) -> Result<Infiset<Item>, ErrorKind>
    {
        Ok(Infiset::Include{include: HashSet::default()})
    }
}
