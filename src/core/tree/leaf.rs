use super::super::weight::Weight;
use super::node::Node;

pub struct Leaf {
    char: char,
    weight: Weight,
}

impl Leaf {
    pub fn new(char: char, weight: Weight) -> Self {
        Self { char, weight }
    }
}

impl Node for Leaf {
    fn get_char(&self) -> Option<char> {
        Some(self.char)
    }

    fn get_children(&self) -> Option<(&Box<dyn Node>, &Box<dyn Node>)> {
        return None;
    }

    fn get_weight(&self) -> Weight {
        return self.weight;
    }
}
