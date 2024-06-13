use super::lookup::Lookup;
use super::weight::Weight;
use super::super::table::encoding::Encoding;


pub struct Table{
    pub weights  : Lookup<Weight>,
    pub encodings : Lookup<Encoding>,
}

impl Table {
    pub fn new(weights : Lookup<Weight>, encodings : Lookup<Encoding>) -> Self {
        Self {
            weights,
            encodings
        }
    }
}