use crate::huffman::node::Node;
use crate::huffman::types::Weight;

pub struct Leaf 
{
    char   : char,
    weight : Weight
}

impl Leaf {
    pub fn new(char : char, weight : Weight) -> Self {
        Self { char , weight }
    }

    pub fn get_char(&self ) -> char {
        return self.char;
    }
}

impl Node for Leaf {


    fn get_weight(&self ) -> Weight{
        return self.weight;
    }


}