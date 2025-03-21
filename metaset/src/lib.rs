pub mod node;
pub mod metaset;

pub use node::{Node, NodeSlice};
pub use node::node_types::*;
pub use metaset::{MetaSet, MetaItem};

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
