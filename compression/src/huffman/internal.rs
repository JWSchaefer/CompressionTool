use crate::huffman::node::Node;
use crate::huffman::types::Weight;

pub struct Internal {
    left  : Box<dyn Node>,
    right : Box<dyn Node>
}

impl Internal {
    pub fn new(left : Box<dyn Node>, right : Box<dyn Node>) -> Self {
        Self {left, right}
    } 
}

impl Node for Internal {
    fn get_weight(&self) -> Weight {
        self.left.get_weight() + self.right.get_weight()
    }
}