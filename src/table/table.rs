use super::code::Code;
use super::lookup::Lookup;
use super::weight::Weight;

use std::collections::HashMap;
pub struct Table {
    pub weights: Lookup<Weight>,
    pub encodings: Lookup<Code>,
    pub decodings: HashMap<Code, char>,
}

impl Table {
    pub fn new(
        weights: Lookup<Weight>,
        encodings: Lookup<Code>,
        decodings: HashMap<Code, char>,
    ) -> Self {
        Self {
            weights,
            encodings,
            decodings,
        }
    }
}
