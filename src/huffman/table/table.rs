use super::weight::Weight;
use super::encoding::Encoding;
use super::lookup::Lookup;
use super::super::huffman::constant::SPACER;

use std::iter::zip;
pub struct Table{
    pub weights  : Lookup<Weight>,
    pub encodings  : Lookup<Encoding>,
}

impl Table {
    pub fn new(weights : Lookup<Weight>, encodings  : Lookup<Encoding>) -> Self {
        Self {
            weights,
            encodings,
        }
    }

}