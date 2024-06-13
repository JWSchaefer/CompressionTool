use super::node::Node;
use super::super::table::weight::Weight;

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

    fn get_char(&self) -> Option::<char> {
        None
    }

    fn get_children(&self) -> Option<(&Box<dyn Node>, &Box<dyn Node>)> {
        return Some((&self.left, &self.right));
    }

    fn get_weight(&self) -> Weight {
        self.left.get_weight() + self.right.get_weight()
    }
} 