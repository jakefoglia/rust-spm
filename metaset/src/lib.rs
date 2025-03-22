pub mod node;
pub mod metaset;

pub use metaset::MetaSet;
pub use node::{Node, types};

use metaset::*;
use node::*;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
