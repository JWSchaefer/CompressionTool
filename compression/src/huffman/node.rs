use crate::huffman::types::Weight;

pub trait Node {
    fn get_weight(&self) -> Weight;
}

