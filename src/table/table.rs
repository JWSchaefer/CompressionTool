use super::encoding::Encoding;
use super::lookup::Lookup;
use super::weight::Weight;

use std::collections::HashMap;
pub struct Table {
    pub weights: Lookup<Weight>,
    pub encodings: Lookup<Encoding>,
    pub decodings: HashMap<Encoding, char>,
}

impl Table {
    pub fn new(
        weights: Lookup<Weight>,
        encodings: Lookup<Encoding>,
        decodings: HashMap<Encoding, char>,
    ) -> Self {
        Self {
            weights,
            encodings,
            decodings,
        }
    }
}
